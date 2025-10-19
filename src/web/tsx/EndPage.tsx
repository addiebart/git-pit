import TaskInfo from "./TaskInfo";

export default function EndPage() {
    const title = "Thanks for Playing!";
    const taskDetailsContent = (<>
        That marks the end of Git Smart. Due to limited time,
        we weren't able to implement levels involving remotes or
        more complex levels with series of commands
        (ex: stash, checkout, commit, push, checkout back, pop stash) as we originally intended.
        We hope you enjoyed and/or learned something!
        <br/><br />
        -- Addison, Eyassu, and Lily
    </>)

    return (
        <>
            <div className="flex flex-col flex-1">
                <div className="filler flex-1"></div>
                <TaskInfo taskDetailsContent={taskDetailsContent} title={title}></TaskInfo>
                <div className="filler flex-1 flex flex-col">
                    <div className="filler flex-1"></div>
                    <button 
                    className="text-5xl p-4 border-[var(--lightpurple)] border-4 rounded-4xl w-fit mx-auto cursor-pointer hover:bg-[var(--slate)]"
                    onClick={() => location.reload()}>
                        Reset Game
                    </button>
                    <div className="filler flex-1"></div>
                </div>
            </div>
            <div className="bottomMargin mb-16"></div>
        </>
    );
}