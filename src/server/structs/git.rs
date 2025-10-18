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
	pub fn git_config_username (username: String ) -> Result<_, Error>{
		let repo = Repository::open(".")?;
		let mut config = repo.config()?;
		config.set_str("user.name", username)?;
		println!("Updated git username locally!");
		Ok(())
	}
	
	//git config email will change the users email 
	pub fn git_config_email (email: String) -> Result<_, Error>{
		let repo = Repository::open(".")?;
		let mut config = repo.config()?;
		config.set_str("user.email", email)?;
		println!("Updated git email locally!");
		Ok(())
	}
	
	//we adding
	pub fn git_add (filename: String) -> Result<_, Error>{
		let repo = Repository::open(".")?;
		let mut index = repo.index()?;
		match index {
			"." => {
				let mut status_opts = StatusOptions::new();
				let statuses = repo.statuses(Some(&mut status_opts))?;
				for entry in statuses.iter() {
					if let Some(path) = entry.path() {
						index.add_path(std::path::Path::new(path))?;
					}
				}
			}
			_ => index.add_path(std::path::Path::new(filename))?;
		}
		index.write()?;
		println!("add staged");
		Ok(())
	}
	
	//we commiting
	pub fn git_commit(message: String) -> Result<_, Error>{
		let repo = Repository::open(".")?;
		let sig = repo.signature()?; 
		let mut index = repo.index()?;
		let tree_id = index.write_tree()?;
		let tree = repo.find_tree(tree_id)?;

		//attaches previous commit to this one
		let parent_commit = repo.head()?.peel_to_commit()?;

		// Make the commit!
		repo.commit(
			Some("HEAD"),   // update HEAD
			&sig,           // author
			&sig,           // committer
			message,        // message
			&tree,          // new tree object
			&[&parent_commit], // parent commit(s)
		)?;

		Ok(())
	}	
	
	//implement a fake "git push"
	
	//resets last commit
	pub fn git_reset(compiler_flag: String) -> Result<_, Error>{
		let repo = Repository.open(".")?;
		let target_commit = repo.revparse_single("HEAD~1")?; //hard-codes it so only the last commit is reset
		match compiler_flag:
			"--soft" => repo.reset(&target_commit, ResetType::Soft, None)?;
			_ => repo.reset(&target_commit, ResetType::Hard, None)?;
		Ok(())
	}	
	
	
}

//Git struct
//creating
