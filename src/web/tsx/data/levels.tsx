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
        title: "Time to Git Smart",
        details: (<>
        Git Smart is an educational game that challenges the player to perform Git operations to complete tasks.
        If you're not too familar with Git, looking at a <a href="https://education.github.com/git-cheat-sheet-education.pdf" className="underline">Git cheat sheet</a> is recommended.
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
        This is important for us to proceed. I need to track you............r files.
        There's a file called "addme.txt" in the directory you're currently in (and can't leave).
        I need you track that file down. Some also call this staging, adding, or snapshotting. 
        </>),
        successCaptureSequence: [/File write succeeded.*addme.txt/],
    },
    {
        number: 3,
        title: "Commit to Something.",
        details: (<>
        In Git, A "commit" is a "permanent" snapshot in the version history of a repository.
        These snapshots are the basis of Git, the idea being that you can return or build upon them at any time.
        I need you to make a commit from the file you added with a custom note.
        </>),
        successCaptureSequence: [/Commit succeeded/],
    },
    {
        number: 4,
        title: "Branch Out.",
        details: (<>
        Branches are like parallel timelines for your repository. 
        At any point, you can split off from the current state of your repository to work an alternate timeline.
        This is helpful if you want to have a quick and easy way to revert your changes
        or if you're working in a group wheres multiple people want to work at the same time without steping on each other's toes.
        Create a new branch called "branchtwo" <span className="font-semibold italic">without switching to it.</span>
        </>),
        successCaptureSequence: [/Created branch 'branchtwo'/],
    },
    {
        number: 5,
        title: "At the Grocery Store.",
        details: (<>
        The checkout command allows you to change your active branch to another in your repository. 
        You can also use "git checkout -b &lt;branchname&gt;" to create a branch and switch to it in one step.
        Switch your active branch to the one you created in the previous task.
        </>),
        successCaptureSequence: [/Checkout successful.*branchtwo/],
    },
    /*
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
    */
] as Level[];