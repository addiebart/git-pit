use git2::Repository;
use smol::{io::{AsyncReadExt, Error, ErrorKind}, io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use async_tungstenite::accept_async;            
use async_tungstenite::tungstenite::protocol::Message;  
use smol::prelude::*;
mod structs;
use structs::git::{Parser, GitRunner};
use http::{HeaderMap, Request, HeaderName, HeaderValue, request::Builder, Version, Method};
use mews::{WebSocketContext, WebSocket, Connection};

fn main() -> std::io::Result<()> {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8081").await?;
        println!("Websocket server listening @ {:?}", listener.local_addr());
        loop {
            let (stream, addr) = listener.accept().await?;
			println!("New tcp connection from {addr}");
			smol::spawn(handle_connection(stream, addr)).detach();
        }
		GitRunner::uninit();
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
	let parser = parse_http(&mut stream).await?;
	if let Some(key) = parser.headers().get("Sec-WebSocket-Key"){
		match key.to_str(){
			Ok(sec_key) =>{
				let ctx = WebSocketContext::new(sec_key);
				let (sign, ws) = ctx.on_upgrade(
					move |mut conn: Connection<TcpStream>| async move {
						while let Ok(Some(msg)) = conn.recv().await {
							match msg {
								mews::Message::Text(text) => {
									println!("Recieved text from {addr}: {text}");
								}
								mews::Message::Binary(bin) => {
									println!("Recieved binary from {addr}: {bin:?}");
								}
								mews::Message::Close(close) => {
									println!("Client {addr} disconnected");
									break;
								}
								_ => {}
							}
						}
					}
				);
			}
			Err(e) => eprintln!("Sec-WebSocket-Key string error!! handshake failed at {addr}: {e:?}"),
		}
	}else{
		eprintln!("Sec-WebSocket-Key not found! handshake failed at {addr}");
	}
	Ok(())
}
/*
async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr){
	let mut parser = Parser::new();
	match accept_async(stream).await{
		Ok(mut websocket) => {
				println!("websocketconnectionestablished with {addr}");
				//listens for messages
				while let Some(msg) = smol::stream::StreamExt::next(&mut websocket).await{
					match msg {
						Ok(Message::Text(text)) => {
                            println!("Recieved from {addr}: {text}"); 
                            //this is where you'd call worker function, then send back to addr
                            //call gitrunner
                            let message: String = parser.parse(text.to_string());
                            //Success/Failure should then be sent over to the JS/HTML
                            if websocket.send(Message::Text(format!("{message}").into())).await.is_err() {
                                println!("Failed to send message to {addr}");
                                break;
                            }
                        }
						Ok(Message::Binary(bin)) => {
							println!("Binary data from {addr}: {bin:?}");
						}
						Ok(Message::Close(_)) | Err(_) => {
							println!("Client {addr} disconnected");
							break;
						}
						_ => {}	
					}	
				}
		}
	Err(e) => eprintln!("Failed WebSocket handshake with {addr}: {e:?}"),
	}	
}

*/
