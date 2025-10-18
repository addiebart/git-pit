import React from 'react';
import TopNav from './TopNav';
import LevelPage from './LevelPage';

export default function Page() {
  return (
    <div className="min-h-screen flex flex-col" id="PageFlexbox">
      <TopNav/>
      <main className="mx-[10vw] flex flex-col flex-1 min-h-0 h-full">
            <LevelPage/>
      </main>
    </div>
  );
}