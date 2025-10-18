use git2::Repository;
use smol::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};

fn main() -> std::io::Result<()> {
    smol::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!(" Git server running on http://127.0.0.1:8080");

        loop {
            let (mut stream, _) = listener.accept().await?;
            smol::spawn(async move {
                let mut buffer = [0u8; 1024];
                if let Ok(n) = stream.read(&mut buffer).await {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    println!(" Request:\n{}", request);

                    // crude path extraction
                    let response = if request.starts_with("POST /init") {
                        // pick a directory to init the repo
                        let dir = std::path::Path::new("repos/my_repo");
                        println!("Path: {}", dir.display());

                        // ensure the parent folder exists
                        if let Err(e) = std::fs::create_dir_all(&dir) {
                            eprintln!("❌ Failed to create dir: {}", e);
                        }

                        match Repository::init(&dir) {
                            Ok(_) => {
                                println!("✅ Repo initialized at {:?}", dir);
                                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nRepository initialized!\r\n"
                            }
                            Err(err) => {
                                eprintln!("❌ Git init failed: {}", err);
                                "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nFailed to init repository\r\n"
                            }
                        }
                    } else {
                        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nNot found\r\n"
                    };

                    let _ = stream.write_all(response.as_bytes()).await;
                }
            })
            .detach();
        }
    })
}
