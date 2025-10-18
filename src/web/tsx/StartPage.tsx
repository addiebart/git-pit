import ShellPrompt from "./ShellPrompt";
import CommandWebsocketManager from "./CommandWebsocketManager";
import Title from "./Title";

export default function() {
  const socket = CommandWebsocketManager(8081);

  const taskDetailsContent = (
    <>
      Git Pit is an educational game that challenges the player to perform Git operations to complete tasks.
      If you're not too familar with git, looking at a git cheat sheet is recommended. The point is to learn!
    </>
  );

  return (
    <>
      <div className="filler flex-1"></div>
      <Title textContent="Enter the Pit"></Title>
      <div className="filler flex-1"></div>
      <ShellPrompt onSubmit={(value: string) => socket.send(value)}/>
      <div className="bottomMargin mb-16"></div>
    </>
  );
}