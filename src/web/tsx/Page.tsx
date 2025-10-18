import React, { useState } from 'react';
import TopNav from './TopNav';
import LevelPage from './LevelPage';

export default function Page() {

  const [level, setLevel] = useState(0);

  return (
    <div className="min-h-screen flex flex-col" id="PageFlexbox">
      <TopNav/>
      <main className="mx-[10vw] flex flex-col flex-1 min-h-0 h-full">
            <LevelPage lvlidx={level}/>
      </main>
    </div>
  );
}