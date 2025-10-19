import React from "react";
import { faCodeBranch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export default function({ onSubmit, branch }: { onSubmit: (value: string) => void, branch: string }) {

    const submitHandler: React.FormEventHandler<HTMLFormElement> = (event) => {
        event.preventDefault();
        const inputEle = event.currentTarget.elements.item(0)! as HTMLInputElement;
        const text = inputEle.value.trim().split(/\s+/).join(" ");
        onSubmit(text);
        inputEle.value = "";
    }

    return (
        <div className="text-3xl p-3 border-[var(--lightpurple)] border-4 rounded-4xl">
            <div className="flex gap-2 font-semibold items-center leading-1">
                <span id="hostname" className="text-[var(--terminalgreen)] ml-2">git@smart</span>
                <FontAwesomeIcon icon={faCodeBranch} className="text-[var(--orange)]"/>
                <span id="branchname" className="text-[var(--orange)]">{branch}</span>
                <span id="dollarsign" className="font-bold pr-2 pl-1">$</span>
                <form onSubmit={submitHandler} className="flex flex-1">
                    <input
                        type="text"
                        name="shellprompt"
                        id="shellprompt"
                        autoComplete="off"
                        maxLength={50}
                        className="flex-1 min-w-0 bg-transparent border-0 focus:outline-none p-0 m-0 text-[inherit]"
                    />
                </form>
            </div>
        </div>
    );
}