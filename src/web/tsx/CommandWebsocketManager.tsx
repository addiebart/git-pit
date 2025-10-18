import React from "react";

export default function(port: number): WebSocket {

    //https://developer.mozilla.org/en-US/docs/Web/API/WebSocket
    const socket = new WebSocket(`ws://127.0.0.1:${port}`);

    socket.onopen = (event) => {
        socket.send("WS: Websocket Start!");
    };

    socket.onmessage = (event) => {
        console.log("WS: Got message:", event.data);
    };

    return socket;
}