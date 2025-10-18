import type { JSX } from "react";

export type Level = {
    number: number,
    title: string,
    details: JSX.Element
    goal: string
};

export default [
    {
        number: 0,
        title: "Level 0",
        details: (<></>),
        goal: "INIT"
    },
    {
        number: 1,
        title: "Level 1",
        details: (<></>),
        goal: "Complete the second task"
    },
    {
        number: 2,
        title: "Level 2",
        details: (<></>),
        goal: "Complete the third task"
    },
    {
        number: 3,
        title: "Level 3",
        details: (<></>),
        goal: "Complete the fourth task"
    },
    {
        number: 4,
        title: "Level 4",
        details: (<></>),
        goal: "Complete the fifth task"
    },
    {
        number: 5,
        title: "Level 5",
        details: (<></>),
        goal: "Complete the sixth task"
    },
    {
        number: 6,
        title: "Level 6",
        details: (<></>),
        goal: "Complete the seventh task"
    },
    {
        number: 7,
        title: "Level 7",
        details: (<></>),
        goal: "Complete the eighth task"
    },
    {
        number: 8,
        title: "Level 8",
        details: (<></>),
        goal: "Complete the ninth task"
    },
    {
        number: 9,
        title: "Level 9",
        details: (<></>),
        goal: "Complete the tenth task"
    }
] as Level[];