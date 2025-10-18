//git.rs (Facilitating git calls to backend using smol)
use git2::{Repository, Error, ResetType, StatusOptions};

pub struct GitRunner{
	command_queue: Vec<String>,
	repo: Repository
}

pub struct Parser{
	git_runner: Option<GitRunner>
	
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
	pub fn uninit() -> std::io::Result<()>{
		std::fs::remove_dir_all("repo")?;
		Ok(())
	}
	
	//git config username will change the users name 
	pub fn git_config_username (&mut self, username: String ) -> Result<(), Error>{
		let repo = Repository::open(".")?;
		let mut config = repo.config()?;
		config.set_str("user.name", &username)?;
		println!("Updated git username locally!");
		Ok(())
	}
	
	//git config email will change the users email 
	pub fn git_config_email (&mut self, email: String) -> Result<(), Error>{
		let repo = Repository::open(".")?;
		let mut config = repo.config()?;
		config.set_str("user.email", &email)?;
		println!("Updated git email locally!");
		Ok(())
	}
	
	//we adding
	/*
	pub fn git_add (&mut self, filename: String) -> Result<_, Error>{
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
			},
			_ => index.add_path(std::path::Path::new(filename))?,
		}
		index.write()?;
		println!("add staged");
		Ok(())
	}
	*/
	//we commiting
	pub fn git_commit(&mut self, message: String) -> Result<(), Error>{
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
			&message,        // message
			&tree,          // new tree object
			&[&parent_commit], // parent commit(s)
		)?;

		Ok(())
	}	
	
	//implement git pull, git pull from origin
	
	//resets last commit
	pub fn git_reset(&mut self, compiler_flag: String) -> Result<(), Error>{
		let repo = Repository::open(".")?;
		let target_commit = repo.revparse_single("HEAD~1")?; //hard-codes it so only the last commit is reset
		match compiler_flag.as_str(){
			"--soft" => repo.reset(&target_commit, ResetType::Soft, None)?,
			_ => repo.reset(&target_commit, ResetType::Hard, None)?,
		}
		Ok(())
	}	
	
	pub fn git_checkout(&mut self, branch_name: String) -> Result<(), Error> {
		Ok(())
			
	}
	
	pub fn git_branch(&mut self, branch_name: String) -> Result<(), Error> {
		Ok(())
		
	}

	pub fn git_merge(&mut self, branch_name: String) -> Result<(), Error> {
		Ok(())
		
	}
	
	//executes git -a branch
	pub fn git_branch_namecheck(&mut self) -> Result<(), Error> {
		Ok(())
	}
	
	pub fn git_branch_show_current(&mut self) -> Result<(), Error> {
		Ok(())
	}
	
	pub fn git_log_follow(&mut self) -> Result<(), Error> {
		Ok(())
	}
	
	pub fn git_stash(&mut self) -> Result<(), Error> {
		Ok(())
	}
}

impl Parser{
	pub fn new()-> Self{
		Self{git_runner: None}
	}
	
	pub fn parse(&mut self, input: String) -> String{
		//make sure str starts with git
		if !input.starts_with("git"){
			return String::from("Invalid command- give me a git command, goddamnit!");
		}
		
		//seeing what thing you're calling
		match input.as_str(){
			"git init" => {
				if let Ok(runner) = GitRunner::init(){
					self.git_runner = Some(runner);
				}
				return String::from("Git repo created!");
			}
			_ => {return String::from("Repo creation unsuccessful")}
		}
		if input.contains("git config user.name"){
			self.git_runner.unwrap().git_config_username(input[11..input.len() - 2].to_string());
			return String::from("Username successfully changed");
		}
		if input.contains("git config user.email"){
			self.git_runner.unwrap().git_config_email(input[11..input.len() - 2].to_string());
			return String::from("Email successfully changed");
		}
		
		return String::from("Invalid command!");
	}	
	
}	
