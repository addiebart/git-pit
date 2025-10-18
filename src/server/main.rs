use git2::Repository;
use smol::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpListener};
mod structs;
use structs::{git::GitRunner};  

fn main() -> std::io::Result<()> {
	if let Ok(gitrunner) = GitRunner::init(){
		println!("git made!");
		std::fs::remove_dir_all("repo")?;
	}else{
		println!(":(");
	}
	Ok(())
}
