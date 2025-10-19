use git2::Repository;
use smol::{io::{AsyncReadExt, Error, ErrorKind}, io::AsyncWriteExt, net::{TcpListener, TcpStream}, spawn, Timer};
use std::time::Duration;
use smol::prelude::*;
mod structs;
use structs::git::{Parser, GitRunner};
use http::{HeaderMap, Request, HeaderName, HeaderValue, request::Builder, Version, Method, StatusCode, Response};
use mews::{WebSocketContext, WebSocket, Connection, Message};

fn main() -> std::io::Result<()> {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8081").await?;
        println!("Websocket server listening @ {:?}", listener.local_addr());
        loop {
            let (stream, addr) = listener.accept().await?;
			println!("New tcp connection from {addr}");
			smol::spawn(handle_connection(stream, addr)).detach();
        }
	})
}

async fn parse_http(stream: &mut TcpStream) -> std::io::Result<Request<()>>{
	let mut buffer: [u8; 2048] = [0u8; 2048];
	let buffer_size: usize = stream.read(&mut buffer).await?;
	//split buffer somehow
	let request_text: String = std::str::from_utf8(&buffer).unwrap().to_string();
	let mut lines = request_text.split("\r\n");
	let request_line = lines.next().unwrap_or_default();
	
    let mut parts = request_line.split_whitespace();
    let method_str = parts.next().unwrap_or("GET");
    
    let path = parts.next().unwrap_or("/");
    let _version_str = parts.next().unwrap_or("HTTP/1.1");

    let method = method_str.parse::<Method>().unwrap_or(Method::GET);
    let version = if _version_str == "HTTP/1.0" { Version::HTTP_10 } else { Version::HTTP_11 };

    let mut req: Builder = Request::builder()
		.method(method)
		.uri(path)
		.version(version);

	if let Some(req_map) = req.headers_mut(){//: Option<&mut HashMap<>>
		//set up request
		for line in lines{
			if let Some((key, value)) = line.split_once(":") {
				match (key.trim().parse::<HeaderName>(), value.trim().parse::<HeaderValue>()){
					(Ok(name), Ok(value)) => {
						req_map.insert(name, value);
					},
					_ => return Err(Error::new(ErrorKind::InvalidData, "Data could not be parsed into string")),
				}
			}
		}
	}else{
		return Err(Error::new(ErrorKind::NotFound, "Builder had error!"));
	}
	req.body(()).map_err(|e| Error::new(ErrorKind::NotFound, "Builder error when trying to make request!"))
}

async fn handle_connection(mut stream: TcpStream, addr: std::net::SocketAddr) -> std::io::Result<()>{
	let http_parser = parse_http(&mut stream).await?;
	if let Some(key) = http_parser.headers().get("Sec-WebSocket-Key"){
		match key.to_str(){
			Ok(sec_key) =>{
				let ctx = WebSocketContext::new(sec_key);
				let (sign, ws) = ctx.on_upgrade(
					move |mut conn: Connection<TcpStream>| async move {
						let mut parser = Parser::new();
						while let Ok(Some(msg)) = conn.recv().await {
							match msg {
								Message::Text(text) => {
									println!("Recieved text from {addr}: {text}");
									//this is where you'd call worker function, then send back to addr
									//call gitrunner
									let message: String = parser.parse(text.to_string());
									//Success/Failure should then be sent over to the JS/HTML
									if conn.send(Message::Text(format!("{message}").into())).await.is_err() {
										println!("Failed to send message to {addr}");
										break;
									}
								}
								Message::Binary(bin) => {
									println!("Recieved binary from {addr}: {bin:?}");
								}
								Message::Close(close) => {
									println!("Client {addr} disconnected");
									GitRunner::uninit();
									break;
								}
								_ => {}
							}
							Timer::after(Duration::from_secs(1));
						}
					}
				);
				/* this would be cute to use if possible
				let response = Response::builder()
					.status(StatusCode::SWITCHING_PROTOCOLS)
					.header("Upgrade", "websocket")
					.header("Connection", "Upgrade")
					.header("Sec-WebSocket-Accept", &sign)
					.body(())
					.unwrap();
				*/
				let mut resp_str = format!(
					"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",
					sign
				);

				// You could use `http::Response`'s parts, but manual formatting is simpler here.
				if let Err(e) = stream.write_all(resp_str.as_bytes()).await {
					eprintln!("write error: {e}");
				}
				spawn(ws.manage(stream)).detach();
			}
			Err(e) => eprintln!("Sec-WebSocket-Key string error!! handshake failed at {addr}: {e:?}"),
		}
	}else{
		eprintln!("Sec-WebSocket-Key not found! handshake failed at {addr}");
	}
	Ok(())
}
