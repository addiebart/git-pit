import ShellPrompt from "./ShellPrompt";
import CommandWebsocketManager from "./CommandWebsocketManager";
import TaskInfo from "./TaskInfo";
import StatusText from "./StatusText";

export default function() {
  const socket = CommandWebsocketManager(8081);

  const taskDetailsContent = (
    <>
      Git Pit is an educational game that challenges the player to perform Git operations to complete tasks.
      If you're not too familar with git, looking at a <a href="https://education.github.com/git-cheat-sheet-education.pdf" className="underline">git cheat sheet</a> is recommended.
      The point is to learn!
      Create a local Git repsoitory to begin!
    </>
  );

  return (
    <>
      <div className="flex flex-col flex-1">
        <div className="filler flex-1"></div>
        <TaskInfo taskDetailsContent={taskDetailsContent} title="Enter the Pit!"></TaskInfo>
        <div className="filler flex-1"></div>
      </div>
      <StatusText></StatusText>
      <ShellPrompt onSubmit={(value: string) => socket.send(value)}/>
      <div className="bottomMargin mb-16"></div>
    </>
  );
}