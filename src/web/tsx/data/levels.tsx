import type { JSX } from "react";

export type Level = {
    number: number,
    title: string,
    details: JSX.Element
    successCaptureSequence: RegExp[]
};

export default [
    {
        number: 0,
        title: "Enter The Git Pit!!",
        details: (<>
        Git Pit is an educational game that challenges the player to perform Git operations to complete tasks.
        If you're not too familar with git, looking at a <a href="https://education.github.com/git-cheat-sheet-education.pdf" className="underline">git cheat sheet</a> is recommended.
        The point is to learn!
        Create a local Git repsoitory to begin!
        </>),
        successCaptureSequence: [/Repo initialized/]
    },
    {
        number: 1,
        title: "Introduce Yourself.",
        details: (<>
        Thanks for getting us started. I'm excited for us to learn how to use Git together.
        I'm an abstract sentient being designed to sound vaguely human, but you probably have a name.
        I don't really care what it is, but I do care that you know how to tell me what it is.
        Also, let's keep it between you and me. <span className="font-semibold italic">the rest of the globe doesn't need to know.</span>
        </>),
        successCaptureSequence: [/Updated git username/],
    },
    {
        number: 2,
        title: "Enable Tracking",
        details: (<>
        This is important for us to proceded. I need to track you............r files.
        There's a file called "addme.txt" in the directory you're currently in &lp;and can't leave&rp;.
        I need you track that file down. Some also call this staging, adding, or snapshotting. 
        </>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 3,
        title: "Commit to Something.",
        details: (<>
        
        </>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 4,
        title: "Level 4",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 5,
        title: "Level 5",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 6,
        title: "Level 6",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 7,
        title: "Level 7",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 8,
        title: "Level 8",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 9,
        title: "Level 9",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    }
] as Level[];