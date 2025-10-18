import React, { type ReactHTMLElement } from "react";

export default function({ content } : {content: ReactHTMLElement<HTMLElement>}) {
    return <p className="text-lg">{content}</p>;
} 