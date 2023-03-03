import React, { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri';


function App() {
  useEffect(() => {
    invoke('close_splashscreen');
  });
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
