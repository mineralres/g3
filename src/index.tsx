import React, { useState, useEffect, useRef } from 'react'
import ReactDOM from 'react-dom/client';
import './index.css';
import reportWebVitals from './reportWebVitals';
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import ErrorPage from "./error-page";
import Root from "./routes/root";
import Account from "./routes/account";
import Trading from "./routes/trading";
import Log from "./routes/log";
import { invoke } from '@tauri-apps/api/tauri';
import { FloatButton, Modal } from 'antd';

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "account",
        element: <Account></Account>,
      },
      {
        path: "trading",
        element: <Trading></Trading>
      },
    ],
  },
]);

document.addEventListener('DOMContentLoaded', () => {
  invoke('close_splashscreen');
})

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
const RootA = () => {
  const [showLog, setShowLog] = useState(false);
  return (
    <React.StrictMode>
      <RouterProvider router={router} />
      <FloatButton onClick={() => setShowLog(!showLog)}></FloatButton>
      <Modal forceRender={true} footer={null} width={1200} onOk={() => setShowLog(false)} onCancel={() => setShowLog(false)} open={showLog}>
        <h3>运行日志</h3>
        <Log></Log>
      </Modal>
    </React.StrictMode>
  )
}
root.render(
  <RootA></RootA>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
