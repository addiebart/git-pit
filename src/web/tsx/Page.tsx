import React from 'react';
import TopNav from './TopNav';
import Main from './Main';

export default function Page() {
  return (
    <div className="min-h-screen flex flex-col">
      <TopNav/>
      <Main/>
    </div>
  );
}