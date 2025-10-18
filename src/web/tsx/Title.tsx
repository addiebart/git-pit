import React from "react";

export default function({textContent} : {textContent: string}) {
    return (
        <h1 className="title text-7xl">{textContent}</h1>
    );
}