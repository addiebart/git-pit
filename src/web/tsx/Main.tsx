import React from "react";
import StartPage from "./StartPage";

export default function() {
    return (
        <div className="flex flex-col flex-1 min-h-0">
            <main className="mx-[10vw] flex-1 min-h-0">
                <StartPage />
            </main>
        </div>
    )
}