import { useState } from "react";
import ShellPrompt from "./ShellPrompt";
import TaskInfo from "./TaskInfo";
import StatusText from "./StatusText";
import levels from "./data/levels";

export default function LevelPage() {
    const [lvlidx, setLevel] = useState(0);
    const [statusText, setStatusText] = useState(" ");
    const [statusColor, setStatusColor] = useState("");

    // Recalculate level and its properties on each render
    const level = (lvlidx < levels.length) ? levels[lvlidx]! : levels[levels.length - 1]!;
    const taskDetailsContent = level.details;
    const title = level.title;
    const regexes = [...level.successCaptureSequence]; // shallow copy

    const port = 8081;
    const socket = new WebSocket(`ws://127.0.0.1:${port}`);

    socket.onmessage = (event) => {
        const messageContent = event.data as string;
        console.log(`WS: Message got: ${messageContent}`);

        let trimmedMessage = messageContent.replace(/(:\s*[0-9]+)/, ""); // Remove status code for user.
        setStatusColor("");
        if (messageContent.includes("400")) {
            setStatusColor("text-[var(--red)]");
        }
        else if (messageContent.includes("200") || messageContent.includes("201")) {
            setStatusColor("text-[var(--terminalgreen)]");
        }
        else if (messageContent.includes("Failed to add: could not find")) {
            setStatusColor("text-[var(--red)]");
            trimmedMessage = trimmedMessage.replace(/('.*\/)/, "").replace(/('.*)/, "");
        }
        else if (messageContent.includes("Failed to open repo:")) {
            setStatusColor("text-[var(--red)]");
            trimmedMessage = "The repository has not yet been initalized"
        }
        setStatusText(trimmedMessage);

        if (regexes.length < 1) {
            levelComplete();
        }
        else if (regexes[0]!.test(messageContent)) {
            regexes.splice(0, 1); // Front pop regexes
            if (regexes.length === 0) {
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
            <span className="ml-4"><span className={statusColor}>
                <StatusText>{statusText}</StatusText>
            </span></span>
            <div className="spacer my-1"></div>
            <ShellPrompt onSubmit={(value: string) => socket.send(value)}/>
            <div className="bottomMargin mb-16"></div>
        </>
    );
}