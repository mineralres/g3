import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, Badge } from 'antd';
import React, { useState, useEffect } from 'react'
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { ExclamationCircleFilled, } from '@ant-design/icons';
import "./order.css";


function truncateString(str: String, num: number) {
    if (str.length <= num) {
        return str
    }
    return str.slice(0, num) + '...'
}

const InstrumentRow = (props: any) => {
    return <tr>
        <td>{props.index + 1}</td>
        <td>{props.exchange}</td>
        <td>{props.symbol}</td>
        <td>{props.name}</td>
        <td>{props.volume_multiple}</td>
        <td>{props.price_tick}</td>
    </tr>
}

export default () => {
    const [instrumentList, setInstrumentList] = useState([]);
    useEffect(() => {
        invoke('instrument_rows').then(res => {
            console.log('order rows', res);
            setInstrumentList(res as any);
        });
        async function test_listen() {
            const unlisten = await appWindow.listen('cta-event', (event: any) => {
                if (event.tp == "LoginCompleted") {
                    if (instrumentList.length === 0) {
                        invoke('instrument_rows').then(res => {
                            setInstrumentList(res as any);
                        });

                    }
                }
            });
            return [unlisten];
        }
        const unlisten = test_listen();
        return () => {
            unlisten.then((ul) => ul.forEach((uf) => uf()));
        }
    }, []);
    return (
        <div>
            <table id="customers" style={{ width: '100%' }}>
                <colgroup>
                    <col span={1} style={{ width: '5%', minWidth: "40px" }}></col>
                    <col span={1} style={{ width: '5%', minWidth: "40px" }}></col>
                    <col span={1} style={{ width: '5%', textAlign: "center" }}></col>
                    <col span={1} style={{ width: '10%', minWidth: "80px" }}></col>
                    <col span={1} style={{ width: '8%' }}></col>
                </colgroup>
                <tr>
                    <th>序号</th>
                    <th>交易所</th>
                    <th>合约</th>
                    <th>名称</th>
                    <th>合约乘数</th>
                    <th>最小变动</th>
                </tr>
                {instrumentList.map((e: any, index) => <InstrumentRow index={index} key={index} {...e} > </InstrumentRow>)}
            </table>

        </div >
    )
}