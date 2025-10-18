import React from "react";
import ShellPrompt from "./ShellPrompt";
import CommandWebsocketManager from "./CommandWebsocketManager";

export default function() {
  const socket = CommandWebsocketManager(8081);

  return (
    <div className="min-h-[100%] flex flex-col justify-end">
      <ShellPrompt onSubmit={(value: string) => socket.send(value)}/>
    </div>
  );
}