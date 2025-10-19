# Git Smart

## Inspiration
We've talked to many students who feel like their courses don't teach them how to use Git, and who have noted it as something they feel their Computer Science programs could improve upon.
## Summary
Git Smart is a gamified introduction/tutorial to Git that covers six different git subcommands. Users are encouraged to reference a Git cheat sheet as the solve the challenges presented by the prompts.
## How we built it
Git Smart is a full-stack web application. On the frontend, Git Smart is written in Typescript + React and makes use of Vite, Tailwind and the WebSocket API. The backend is written in Rust and acts as the web server and WebSocket server while also managing the Git repository the user interacts with. The backend uses Smol and Git2 to support asynchronous code and Git repository management, respectively.
## Challenges we ran into
Our team had never developed a full-stack application before and as such, most of our challenges were in figuring out how to coordinate the front and back ends. Specifically, both teams struggled to establish the WebSocket connection between the two as we had little prior experience in programatic networking.
## Accomplishments that we're proud of
We think that there are plenty of things to be proud with this project. We finished on time, it's well polished, the frontend looks nice, we were able to write the backend in Rust, and most importantly, we covered and lot of new ground and learned along the way.
## What we learned
We learned a lot building our first full-stack app. Firstly, we used and learned about a lot of new technologies in this project including Vite, Tailwind, WebSocket, and Smol for Async Rust. Somewhat less tangibly, we learned about the importance of coodination for a larger scale project, quickly realizing we benefited greatly knowing what each other were working on and when changes would be seen upstream.
## What's next for Git Smart
The next steps would be to add more advanced challenges for users to complete. Currently, the challenges are all one command each and don't make use of remotes. To more realistically educate users on Git, this would be a priority.
## Install Instructions
- Ensure rustup is installed on your system. It can be installed from here: http://rustup.rs/
- Ensure node is installed on your system. It can be installed from here: https://nodejs.org/en/download
- Clone, or download and extract, this repository: `git clone https://github.com/addiebart/git-smart`
- cd into the repository: `cd git-smart`
- Build the backend: `cargo build --release`
- Install dependencies for the frontend `npm install`
- Build the frontend: `npm run build`
- The installation is now complete and the server executable is at git-smart/build/server/release/git-smart(.exe) and will start the web server at localhost:8080.
- Enjoy! 
