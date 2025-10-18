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
        title: "Level 0",
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
        title: "Level 1",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 2,
        title: "Level 2",
        details: (<></>),
        successCaptureSequence: [/.*?/],
    },
    {
        number: 3,
        title: "Level 3",
        details: (<></>),
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