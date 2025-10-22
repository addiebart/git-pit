use smol::net::TcpListener;
use std::time::Duration;
mod structs;
use structs::server;
use http_handle::Server;

fn main() -> std::io::Result<()> {
  let server = Server::new("127.0.0.1:8080", "./build/web");
	
    // Run the server in a separate thread so it doesn't block
	let _server_handle = std::thread::spawn(move || {
        server.start().expect("Server failed to start");
    });

    let _ = smol::block_on::<std::io::Result<()>>(async {
        let listener = TcpListener::bind("127.0.0.1:8081").await?;
        println!("Websocket server listening @ {:?}", listener.local_addr());
        loop {
            let (stream, addr) = listener.accept().await?;
			println!("New tcp connection from {addr}");
			smol::spawn(server::handle_connection(stream, addr)).detach();
        }
	});
	std::thread::sleep(Duration::from_secs(2));
	println!("Server has been running for 2 seconds, shutting down...");
	Ok(())
}
