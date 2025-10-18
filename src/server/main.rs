use git2::Repository;
use smol::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};
use async_tungstenite::accept_async;            
use async_tungstenite::tungstenite::protocol::Message;  
use futures::{StreamExt, SinkExt}; 
use smol::prelude::*;

fn main() -> std::io::Result<()> {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        println!("my beautifyl websocket server is server running on {:?}", listener.local_addr());

        loop {
            let (stream, addr) = listener.accept().await?;
			println!("New tcp connection from {addr}");
			smol::spawn(handle_connection(stream, addr)).detach();
            }
			
            // Ok(())
        })
    }

async fn handle_connection(stream: async_net::TcpStream, addr: std::net::SocketAddr){
	match accept_async(stream).await{
		Ok(mut websocket) => {
				println!("websocketconnectionestablished with {addr}");
				//listens for messages
				while let Some(msg) = futures::StreamExt::next(&mut websocket).await{
					match msg {
						Ok(Message::Text(text)) => {
							println!("Recieved from {addr}: {text}"); 
							//this is where you'd call worker function, then send back to addr
							//call gitrunner
							//gitrunner should execute given git function, then return Success or Failure
							//Success/Failure should then be sent over to the JS/HTML
							if websocket.send(Message::Text(format!("Echo: {text}").into())).await.is_err() {
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

