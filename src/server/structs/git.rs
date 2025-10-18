//git.rs (Facilitating git calls to backend using smol)
use git2::{Repository, Error};
pub struct GitRunner{
	command_queue: Vec<String>,
	repo: Repository
}
impl GitRunner{
	pub fn init() -> Result<GitRunner, Error>{
		let dir = std::path::Path::new("repo");
		if let Err(e) = std::fs::create_dir_all(&dir){
			eprintln!("failed to create directory {}", e);
		} 
		match Repository::init(&dir){
			Ok(repo) => return Ok(Self{command_queue: Vec::new(), repo}),
			Err(err) => return Err(err)
		}
		
	}
}

//Git struct
//creating
