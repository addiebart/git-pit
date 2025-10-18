import type { ReactElement } from "react";
import Title from "./Title";

export default function({taskDetailsContent, title} : {taskDetailsContent: ReactElement, title: string}) {
    return (
        <div className="w-full lg:w-3/5">
            <Title textContent={title}></Title>
            <div className="spacer my-4"></div>
            <p className="taskDetails text-lg" id="taskDetails">{taskDetailsContent}</p>
        </div>
    )
}