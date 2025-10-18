import { useState } from "react";
import ShellPrompt from "./ShellPrompt";
import TaskInfo from "./TaskInfo";
import StatusText from "./StatusText";

export default function() {

    const taskDetailsContent = (
    <>
        Git Pit is an educational game that challenges the player to perform Git operations to complete tasks.
        If you're not too familar with git, looking at a <a href="https://education.github.com/git-cheat-sheet-education.pdf" className="underline">git cheat sheet</a> is recommended.
        The point is to learn!
        Create a local Git repsoitory to begin!
    </>
    );

    const [statusText, setStatusText] = useState(" ");
    
    //https://developer.mozilla.org/en-US/docs/Web/API/WebSocket
    const port = 8081
    const socket = new WebSocket(`ws://127.0.0.1:${port}`);

    socket.onopen = (event) => {
        
    };

    socket.onmessage = (event) => {
        const messageContent = event.data;
        console.log(`WS: Message got: ${messageContent}"`);
        setStatusText(messageContent);
    };

    return (
    <>
        <div className="flex flex-col flex-1">
            <div className="filler flex-1"></div>
            <TaskInfo taskDetailsContent={taskDetailsContent} title="Enter the Pit!"></TaskInfo>
            <div className="filler flex-1"></div>
        </div>
        <span className="ml-4"><StatusText>{statusText}</StatusText></span>
        <div className="spacer my-1"></div>
        <ShellPrompt onSubmit={(value: string) => socket.send(value)}/>
        <div className="bottomMargin mb-16"></div>
    </>
    );
}