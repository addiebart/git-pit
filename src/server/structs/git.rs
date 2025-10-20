//git.rs (Facilitating git calls to backend using smol)
use git2::{Repository, StatusOptions}; //BranchType::{Local, Remote}, build::CheckoutBuilder};

pub struct GitRunner;

impl GitRunner{
	//init will create a repo
	pub fn init() -> String{
		//making dir
		let dir = std::path::Path::new("repo");

		if let Err(e) = std::fs::create_dir_all(&dir){
			return format!("failed to create dir: {}", e);
		} 
		//making repo
		let file_loc = std::path::Path::new("repo/addme.txt");
		if file_loc.exists(){
			return "addme already initialized: Err".to_string()
		}else{
			match std::fs::write("repo/addme.txt", "Rust vs Typescript, a story that predates time, one must imagine a union of the two unhappy...."){
				Ok(_) => {},
				Err(err) => return format!("Addme Init failed: {}", err),
			}
		}
		match Repository::init(&dir){
			Ok(_repo) => "Repo initialized: 201".to_string(),
			Err(err) => format!("git init failed: {}", err),
		}

	}
	//uninit deletes the repo directory (and therefore the repo) (ran on game end)
	pub fn uninit() -> std::io::Result<()>{
		std::fs::remove_dir_all("repo")
	}
	
	//git config username will change the users name 
	pub fn git_config_username (username: String ) -> String{
		let repo = Repository::open("repo");
		let mut config: git2::Config = repo.expect("fatal error").config().expect("...");
		match config.set_str("user.name", &username) {
			Ok(_) => "Updated git username: 200".to_string(),
			Err(e) => format!("Failed to set username: {}", e),
		}
	}
	
	//git config email will change the users email 
	pub fn git_config_email (email: String) -> String{
		let repo = Repository::open("repo");
		let mut config: git2::Config = repo.expect("fatal error").config().expect("fatal error");
		match config.set_str("user.email", &email) {
			Ok(_) => "Updated git email: 200".to_string(),
			Err(e) => format!("Failed to set user email: {}", e),
		}
	}

	//we adding
	pub fn git_add (filename: String) -> String{
		let repo = match Repository::open("repo") {
			Ok(r) => r,
			Err(e) => return format!("Failed to add: {}", e)
		};

		let mut index = match repo.index(){
			Ok(i) => i,
			Err(e) => return format!("Failed to index file: {}", e),
		};

		match filename.as_str() {
			"." => {
				let mut status_opts = StatusOptions::new();
				match repo.statuses(Some(&mut status_opts)){
					Ok(statuses) => {
						for entry in statuses.iter() {
							if let Some(path) = entry.path() {
								match &mut index.add_path(std::path::Path::new(path)) {
									Ok(_) => println!("'add' succeeded: 200, path: {:?}", path),
									Err(e) => return format!("Failed to add: {}", e),
								}
							}
						}
					},
					Err(e) => return format!("Failed to obtain status information: {}", e),
				};
			},
			_ => { 
				match index.add_path(std::path::Path::new(&filename)){
					Ok(_) => println!("'add' succeeded: 200, path: {:?}", filename),
					Err(e) => return format!("Failed to add: {}", e),
				}
				println!("hey!");
			}
		}
		match index.write(){
			Ok(_path) => format!("File write succeeded: 200 path: {}", filename),
			Err(e) => format!("File write failed: {}", e),
		}
	}
	
	//we commiting
	pub fn git_commit(message: String) -> String{
		let repo = match Repository::open("repo") {
			Ok(r) => r,
			Err(e) => return format!("Failed to add: {}", e)
		};

		let mut index = match repo.index(){
			Ok(i) => i,
			Err(e) => return format!("Failed to index file: {}", e),
		};

		let sig = match repo.signature(){
			Ok(s) => s,
			Err(e) => return format!("Failed to get signature: {}", e),
		};

		let tree_id = match index.write_tree(){
			Ok(t) => t,
			Err(e) => return format!("Failed to write index as tree: {}", e),
		};

		let tree = match repo.find_tree(tree_id){
			Ok(t) => t,
			Err(e) => return format!("Failed to find tree: {}", e),
		};

		let parent_commit = match repo.head(){
			Ok(peel) => {
				match peel.peel_to_commit(){
					Ok(p) => p,
					Err(e) => return format!("Failed to turn peel to commit: {}", e),
				}
			}
			Err(_e) => {
				match repo.commit(
					Some("HEAD"), //update HEAD
					&sig, //author
					&sig, //commiter
					&message, //message
					&tree,
					&[]
					) {
					Ok(_) => return "Commit succeeded: 200".to_string(),
					Err(e) => return format!("Failed to add: {}", e),	
				}
			}
		};

		match repo.commit(
			Some("HEAD"), //update HEAD
			&sig, //author
			&sig, //commiter
			&message, //message
			&tree,
			&[&parent_commit]
			) {
			Ok(_) => "Commit succeeded: 200".to_string(),
			Err(e) => format!("Failed to add: {}", e),	
		}
	}	
	
	/*
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
	*/
	//git branch <branch_name>
	pub fn git_checkout(branch_name: String) -> String {
		let repo = match Repository::open("repo") {
			Ok(r) => r,
			Err(e) => return format!("Failed to open repo: {}", e),
		};
		let (object, reference) = match repo.revparse_ext(&branch_name) {
			Ok((o, r)) => (o, r),
			Err(e) => return format!("Failed to find branch: {}", e),
		};
		if let Err(e) = repo.checkout_tree(&object, None) {
			return format!("Failed to checkout: {}", e);
		};
		let res = match reference {
			// gref is an actual reference like branches or tags
			Some(gref) => repo.set_head(gref.name().unwrap()),
			// this is a commit, not a reference
			None => repo.set_head_detached(object.id()),
		};
		match res {
			// Ok(_) => return "checkout complete!".to_string(),
			Ok(_) => format!("Checkout successful: 200: branch name: {}", branch_name),
			Err(e) => return format!("Failed to set head: {}", e)
		}
	}
	pub fn git_branch(branch_name: String) -> String{
		let repo = match Repository::open("repo") {
			Ok(r) => r,
			Err(e) => return format!("Failed to open repo: {}", e),
		};

		// Try to get HEAD; handle case where repo is empty
		let head_commit = match repo.head()
			.ok()
			.and_then(|h| h.peel_to_commit().ok()) {
			Some(c) => c,
			None => {
				return "Cannot create branch: no commits exist yet (empty repo): 400".to_string();
			}
		};

		match repo.branch(&branch_name, &head_commit, false) {
			Ok(_) => format!("Created branch '{}': 200", branch_name),
			Err(e) => format!("Failed to create branch: {}", e),
		}
		
	}
	/*
	//executes git branch -a ... will do later
	pub fn git_branch_namecheck(&mut self) -> Result<(), Error> {
		Ok(())
	}
	*/
	//git branch --show-current
	pub fn git_branch_show_current() -> String{
		let repo = Repository::open("repo");
		let binding = repo.expect("fatal error");
		let mut head = binding.head();
		if head.as_mut().expect("fatal error").is_branch() {
			if let Some(name) = head.expect("fatal error").shorthand() {
				return format!("Current branch {}: 200", name); //we'll need to return this eventually
			} else {return ("Detached HEAD: 400").to_string();}
		} else { return ("Detached HEAD: 400").to_string();}
	}
	/*
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
