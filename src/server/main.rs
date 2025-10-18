use git2::Repository;
use smol::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener, net::TcpStream};
use async_tungstenite::accept_async;            
use async_tungstenite::tungstenite::protocol::Message;  
use smol::prelude::*;
mod structs;
use structs::git::{Parser, GitRunner};

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

