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
import Order from "./routes/order";
import Log from "./routes/log";
import Instrument from "./routes/instrument";
import MarketData from './routes/market_data';
import PositionDetail from './routes/position_detail';
import Position from './routes/position';
import Trade from './routes/trade';
import Broker from './routes/broker';

import { invoke } from '@tauri-apps/api/tauri';
import { FloatButton, Modal } from 'antd';

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "/",
        element: <Account></Account>,
      },
      {
        path: "order-table",
        element: <Order></Order>,
      },
      {
        path: "account-table",
        element: <Account></Account>,
      },
      {
        path: "instrument-table",
        element: <Instrument></Instrument>,
      },
      {
        path: "market-data-table",
        element: <MarketData></MarketData>,
      },
      {
        path: "position-detail-table",
        element: <PositionDetail></PositionDetail>,
      },
      {
        path: "position-table",
        element: <Position></Position>,
      },
      {
        path: "trade-table",
        element: <Trade></Trade>,
      },
      {
        path: "broker",
        element: <Broker></Broker>,
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
