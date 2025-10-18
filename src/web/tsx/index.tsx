import React from 'react';
import { createRoot } from 'react-dom/client';

import Page from './Page';

import "../css/main.css";

const root = createRoot(document.querySelector("#root")!);
root.render(<Page></Page>);