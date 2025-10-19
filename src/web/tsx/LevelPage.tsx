import { useState } from "react";
import ShellPrompt from "./ShellPrompt";
import TaskInfo from "./TaskInfo";
import StatusText from "./StatusText";
import levels from "./data/levels";

export default function LevelPage() {
    const [lvlidx, setLevel] = useState(0);
    const [statusText, setStatusText] = useState(" ");

    // Recalculate level and its properties on each render
    const level = (lvlidx < levels.length) ? levels[lvlidx]! : levels[levels.length - 1]!;
    const taskDetailsContent = level.details;
    const title = level.title;
    const regexes = [...level.successCaptureSequence]; // shallow copy

    const port = 8081;
    const socket = new WebSocket(`ws://127.0.0.1:${port}`);

    socket.onmessage = (event) => {
        const messageContent = event.data;
        console.log(`WS: Message got: ${messageContent}`);
        setStatusText(messageContent);

        if (regexes.length < 1) {
            levelComplete();
        }
        else if (regexes[0]!.test(messageContent)) {
            console.log("Regex matches!");
            regexes.splice(0, 1); // Front pop regexes
            if (regexes.length === 0) {
                console.log("Level Complete!");
                levelComplete();
            }
        }
        else {
            // Do nothing, wait for next message
        }
    };

    function levelComplete() {
        const audio = new Audio("/mp3/ding.mp3");
        audio.play();
        console.log(levels.length);
        if (lvlidx < levels.length - 1) { // if there is another level to play]
            setLevel(lvlidx + 1);
        }
    }

    return (
        <>
            <div className="flex flex-col flex-1">
                
                <div className="filler flex-1"></div>
                <TaskInfo taskDetailsContent={taskDetailsContent} title={title}></TaskInfo>
                <div className="filler flex-1"></div>
            </div>
            <span className="ml-4"><StatusText>{statusText}</StatusText></span>
            <div className="spacer my-1"></div>
            <ShellPrompt onSubmit={(value: string) => socket.send(value)}/>
            <div className="bottomMargin mb-16"></div>
        </>
    );
}