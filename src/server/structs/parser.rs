use super::git::GitRunner;

pub struct Parser;


impl Parser{
	
	pub fn parse(input: String) -> String{
		/*
		let str_iter = input.split_ascii_whitespace();//replace with some kinda function because bash doesn't allow \n and \t 
		//make sure str starts with git
		if str_iter.next() != Some("git"){
			return String::from("Non-git command: 400");
		}
		let command = match str_iter.next(){
			Some(c) => c
			None => return String::from("Git command not specified: 400");
		}
		match &command {
			"init" => GitRunner::init(str_iter.remainder()),
			"config" => GitRunner::config(str_iter.remainder()),
			"branch" => GitRunner::config(str_iter.remainder()),
			"checkout" => GitRunner::config(str_iter.remainder()),
			"add" => GitRunner::config(str_iter.remainder()),
			"commit" => GitRunner::config(str_iter.remainder()),
			e => return format!("Did you mean {}", e)
		}
		*/
		//seeing what thing you're calling
		match input.as_str() {
			"git init" => {
				let msg = GitRunner::init(); // just returns String now
				return msg;
			},
			_ => {},
		}
		//yandere dev ahh code...
		if input.starts_with("git config user.name") {
            if let Some(name) = input.strip_prefix("git config user.name ") {
                let msg = GitRunner::git_config_username(name.trim_matches('"').to_string());
                return msg;
            }
        }
		
		if input.starts_with("git config user.email") {
            if let Some(email) = input.strip_prefix("git config user.email ") {
                let msg = GitRunner::git_config_email(email.trim_matches('"').to_string());
                return msg;
            }
        }
		
		if input.starts_with("git branch --show-current") {
            let msg = GitRunner::git_branch_show_current();
            return msg;
        }

		if input.starts_with("git checkout -b ") { // disable to prevent softlock on branch level
			/*
            if let Some(branch_name) = input.strip_prefix("git checkout -b ") {
                let msg = GitRunner::git_branch( branch_name.trim_matches('"').to_string());
                if !msg.contains("Created") {
					return msg;
				}
				let msg = GitRunner::git_checkout(branch_name.trim_matches('"').to_string());
                return msg;
            }
			*/
        }

		else if input.starts_with("git checkout ") {
            if let Some(branch_name) = input.strip_prefix("git checkout ") {
                let msg = GitRunner::git_checkout(branch_name.trim_matches('"').to_string());
                return msg;
            }
        }
		
		if input.starts_with("git branch ") {
            if let Some(branch_name) = input.strip_prefix("git branch ") {
                let msg = GitRunner::git_branch(branch_name.trim_matches('"').to_string());
                return msg;
            }
        }
		
		if input.starts_with("git add") {
            if let Some(file_name) = input.strip_prefix("git add ") {
                let msg = GitRunner::git_add(file_name.to_string());
                return msg;
            }
        }
		
		if input.starts_with("git commit -m") {
			if let Some(message) = input.strip_prefix("git commit -m ") {
                let msg = GitRunner::git_commit(message.trim_matches('"').to_string());
                return msg;
            }
		}
		return String::from("Invalid git command: 400");
	}	
	
}	
