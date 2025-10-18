//git.rs (Facilitating git calls to backend using smol)
use git2::{Repository, Error};

pub struct GitRunner{
	command_queue: Vec<String>,
	repo: Repository
}
impl GitRunner{
	//init will create a repo
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
	//uninit deletes the repo directory (and therefore the repo) (ran on game end)
	pub fn uninit() -> Result<()>{
		std::fs::remove_dir_all("repo")?;
		Ok(())
	}
	
	//git config username will change the users name 
	pub fn git_config_username (String username) -> Result<GitRunner, Error>{
		let repo = Repository::open(".")?;
		let mut config = repo.config()?;
		config.set_str("user.name", username)?;
		println!("Updated git username locally!");
		Ok(())
	}
	
	//git config email will change the users email 
	pub fn git_config_email (String email) -> Result<GitRunner, Error>{
		let repo = Repository::open(".")?;
		let mut config = repo.config()?;
		config.set_str("user.email", email)?;
		println!("Updated git email locally!");
		Ok(())
	}
}

//Git struct
//creating
