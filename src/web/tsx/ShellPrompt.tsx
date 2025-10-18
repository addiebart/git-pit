import React from "react";
import { faCodeBranch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export default function() {
    return (
        <div className="text-3xl p-4 border-[var(--lightpurple)] border-4 rounded-4xl">
            <div className="flex gap-2 font-semibold items-center">
                <span id="hostname" className="text-[var(--terminalgreen)]">git@pit</span>
                <span id="path" className="text-[var(--terminalgreen)]">~/repo</span>
                <FontAwesomeIcon icon={faCodeBranch} className="text-[var(--orange)]"/>
                <span id="dollarsign" className="font-bold italic pr-2">$</span>
                <input
                    type="text"
                    name="shellprompt"
                    id="shellprompt"
                    maxLength={50}
                    className="flex-1 min-w-0 bg-transparent border-0 focus:outline-none p-0 m-0 text-[inherit]"
                />
            </div>
        </div>
    );
}