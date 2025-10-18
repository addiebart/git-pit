use smol::{io, net, prelude::*, Unblock};
mod structs;

fn main() -> io::Result<()> {
    smol::block_on(async {
        let mut stream = net::TcpStream::connect("127.0.0.1:8080").await?;
        let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(req).await?;

        let mut stdout = Unblock::new(std::io::stdout());
        io::copy(stream, &mut stdout).await?;
        Ok(())
    })
}

