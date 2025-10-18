import React from "react";

export default function() {
    return (
        <nav role="navigation" aria-label="Main" className="mx-[5%] px-[5%] mb-16 font-semibold text-lg border-b-1 border-x-1 rounded-b-2xl">
           <div className="flex items-center py-2">
                <span className="mr-4">Git Pit -- A Educational Git-Based Game for&nbsp;
                    <span className="text-[var(--lightpurple)]">Hack Cats '25!</span> 
                </span>

                <ul className="flex gap-[8px] ml-auto">
                    <li>
                        <button type="button" className="px-2 py-1 rounded-md hover:bg-[var(--slate)] transition-colors text-[var(--cream)] cursor-pointer">
                            Levels
                        </button>
                    </li>
                    <li>
                        <button type="button" className="px-2 py-1 rounded-md hover:bg-[var(--slate)] transition-colors text-[var(--cream)] cursor-pointer">
                            About
                        </button>
                    </li>
                    <li>
                        <button type="button" className="px-2 py-1 rounded-md hover:bg-[var(--slate)] transition-colors text-[var(--cream)] cursor-pointer">
                            Source
                        </button>
                    </li>
                </ul>
           </div>
        </nav>
    )
}