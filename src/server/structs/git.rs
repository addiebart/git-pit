//git.rs (Facilitating git calls to backend using smol)
use git2::{Repository, Error, ResetType, StatusOptions};

pub struct GitRunner{
	command_queue: Vec<String>,
	repo: Repository
}

pub struct Parser{
	pub git_runner: Option<GitRunner>
	
}

impl GitRunner{
	//init will create a repo
	pub fn init() -> String{
		//making dir
		let dir = std::path::Path::new("repo");
		if let Err(e) = std::fs::create_dir_all(&dir){
			return format!("failed to create dir: {}", e);
		} 
		//making repo
		match Repository::init(&dir){
			Ok(_) => "Repo initialized: 201".to_string(),
			Err(err) => format!("git init failed: {}", err),
		}	
	}
	//uninit deletes the repo directory (and therefore the repo) (ran on game end)
	pub fn uninit() -> std::io::Result<()>{
		std::fs::remove_dir_all("repo")?;
		Ok(())
	}
	
	//git config username will change the users name 
	pub fn git_config_username (&mut self, username: String ) -> String{
		let repo = Repository::open("repo");
		let mut config = repo.expect("Always returns String").config().expect("Always returns String");
		match config.set_str("user.name", &username) {
			Ok(_) => "Updated git username: 200".to_string(),
			Err(e) => format!("Failed to set username: {}", e),
		}
	}
	
	//git config email will change the users email 
	pub fn git_config_email (&mut self, email: String) -> String{
		let repo = Repository::open("repo");
		let mut config = repo.expect("Always returns String").config().expect("Always returns String");
		match config.set_str("user.email", &email) {
			Ok(_) => "Updated git email: 200".to_string(),
			Err(e) => format!("Failed to set user email: {}", e),
		}
	}
	
	/*
	pub fn git_add (&mut self, filename: String) -> Result<(), Error>{
		let repo = Repository::open(".")?;
		let mut index = repo.index()?;
		match filename.as_str() {
			"." => {
				let mut status_opts = StatusOptions::new();
				let statuses = repo.statuses(Some(&mut status_opts))?;
				for entry in statuses.iter() {
					if let Some(path) = entry.path() {
						index.add_path(std::path::Path::new(path))?;
					}
				}
			},
			_ => index.add_path(std::path::Path::new(&filename))?,
		}
		index.write()?;
		println!("add staged");
		Ok(())
	}

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
	
	/* gay
	pub fn pull(&mut self) -> Result<(), Error> {
		let repo = Repository::open(".")?;

		// set up remote (usually "origin")
		let mut remote = repo.find_remote("origin")?;

		// set up callbacks for authentication (for HTTPS or SSH)
		let mut cb = RemoteCallbacks::new();
		cb.credentials(|_url, username_from_url, _allowed_types| {
			// This tries SSH first, falls back to default creds
			Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
		});

		// set up fetch options using callbacks
		let mut fo = FetchOptions::new();
		fo.remote_callbacks(cb);

		// fetch updates from origin
		println!("Fetching from origin...");
		remote.fetch(&["main"], Some(&mut fo), None)?;
		println!("Fetch complete!");

		// figure out what branch we’re on
		let head = repo.head()?.shorthand().unwrap().to_string();
		let fetch_head = repo.find_reference("FETCH_HEAD")?;
		let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

		// merge the fetched branch into local
		let analysis = repo.merge_analysis(&[&fetch_commit])?;
		if analysis.0.is_fast_forward() {
			println!("Fast-forwarding {}...", head);

			let mut ref_head = repo.find_reference(&format!("refs/heads/{}", head))?;
			ref_head.set_target(fetch_commit.id(), "Fast-Forward")?;
			repo.set_head(&format!("refs/heads/{}", head))?;
			repo.checkout_head(Some(
				git2::build::CheckoutBuilder::default()
					.force(), // overwrite working tree
			))?;
			println!("Fast-forward complete!");
		} else {
			println!("Non-fast-forward; you’d need a merge or rebase manually.");
		}

		Ok(())
	}
	*/

	/* also gay ill deal with it later 
	pub fn git_clone(&mut self, url: String) -> Result<(), Error> {
		Ok(())
			
	}
	*/
	
	pub fn git_checkout(&mut self, branch_name: String) -> Result<(), Error> {
		let repo = Repository::open(".")?;

		// Find the local branch reference
		let (object, reference_opt) = repo.revparse_ext(&format!("refs/heads/{}", branch_name.as_str()))?;

		if let Some(reference) = reference_opt {
			repo.set_head(reference.name().unwrap())?;
		} else {
			repo.set_head_detached(object.id())?;
		}

		// Update working tree to match new HEAD
		repo.checkout_head(Some(
			git2::build::CheckoutBuilder::default()
				.force() // overwrite working tree files if needed
		))?;

		println!("Switched to branch '{}'", branch_name);
		Ok(())
	}
	
	//git branch <branch_name>
	pub fn git_branch(&mut self, branch_name: String) -> Result<(), Error> {
		let repo = Repository::open(".")?;
		let head_commit = repo.head()?.peel_to_commit()?;
		repo.branch(branch_name.as_str(), &head_commit, false)?;
		println!("Created branch '{}'", branch_name);
		Ok(())
		
	}
	
	//executes git branch -a ... will do later
	pub fn git_branch_namecheck(&mut self) -> Result<(), Error> {
		Ok(())
	}
	
	//git branch --show-current
	pub fn git_branch_show_current(&mut self) -> Result<(), Error> {
		let repo = Repository::open(".")?;
		let head = repo.head()?;
		if head.is_branch() {
			if let Some(name) = head.shorthand() {
				println!("Current branch: {}", name); //we'll need to return this eventually
			}
		} else {
			println!("Detached HEAD");
		}
		Ok(())
	}
	
	pub fn git_merge(&mut self, branch_name: String) -> Result<(), Error> {
		Ok(())
		
	}
	
	pub fn git_log_follow(&mut self) -> Result<(), Error> {
		Ok(())
	}
	
	pub fn git_stash(&mut self) -> Result<(), Error> {
		Ok(())
	}
	*/
}

impl Parser{
	pub fn new()-> Self{
		Self{git_runner: None}
	}
	
	pub fn parse(&mut self, input: String) -> String{
		//make sure str starts with git
		if !input.starts_with("git"){
			return String::from("Non-git command: 400");
		}
		
		//seeing what thing you're calling
		match input.as_str() {
			"git init" => {
				let msg = GitRunner::init(); // just returns String now
				return msg;
			},
			_ => {},
		}
		
		if input.starts_with("git config user.name") {
            if let Some(name) = input.strip_prefix("git config user.name ") {
                let msg = self.git_runner.as_mut().unwrap().git_config_username(name.trim_matches('"').to_string());
                return msg;
            }
        }
		
		if input.starts_with("git config user.email") {
            if let Some(email) = input.strip_prefix("git config user.email ") {
                let msg = self.git_runner.as_mut().unwrap().git_config_email(email.trim_matches('"').to_string());
                return msg;
            }
        }
		/*
		if input.contains("git config user.name"){
			self.git_runner.as_mut().unwrap().git_config_username(input[11..input.len() - 1].to_string());
			return String::from("Username successfully changed");
		} else if input.contains("git config user.email"){
			self.git_runner.as_mut().unwrap().git_config_email(input[11..input.len() - 1].to_string());
			return String::from("Email successfully changed");
		} else if input.contains("git add"){
			let substr = input[7..input.len()].to_string();
			self.git_runner.as_mut().unwrap().git_add(substr);
			return String::from("Add");
		} else if input.contains("git commit -m"){
			let substr = input[14..input.len() - 1].to_string();
			self.git_runner.as_mut().unwrap().git_commit(substr);
			return String::from("Commit");
		} else if input.contains("git reset"){
			let substr = input[11..17].to_string();
			self.git_runner.as_mut().unwrap().git_reset(substr);
			return String::from("Resetty");
		} else if input.contains("git checkout"){
			let substr = input[14..input.len() - 1].to_string();
			self.git_runner.as_mut().unwrap().git_checkout(substr);
			return String::from("Checking out!");
		} else if input.contains("git branch"){
			if input.contains("-a"){
				return String::from("Lillian Brooks-Kanost is too lazy to code this rn")
			} else if input.contains("--show-current"){
				self.git_runner.as_mut().unwrap().git_branch_show_current();
				return String::from("show a current branch");
			} else {
				let substr = input[12..input.len() - 1].to_string();
				self.git_runner.as_mut().unwrap().git_branch(substr);
				return String::from("making a new branch");
			}
		}
		*/
		return String::from("Invalid git command: 400");
	}	
	
}	
