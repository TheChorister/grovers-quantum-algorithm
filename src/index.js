import React, { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.jsx';

const rootNode = document.getElementById('app');
const root = createRoot(rootNode);

root.render(<StrictMode><App bits={4} /></StrictMode>);
