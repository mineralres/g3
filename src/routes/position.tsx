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

const PositionRow = (props: any) => {
    const i_badge = (status: Number) => {
        if (status == 48) {
            return <Badge status='success' text="完全成交"></Badge>;
        } else if (status == 53) {
            return <Badge status='warning' text="已撤单"></Badge>;
        }
        return <Badge status='default' text={`${status}`}></Badge>;
    }
    const i_direction = (direction: number) => {
        /// 0 净， 1多， 2空
        if (direction == 48) {
            return <span style={{ color: "#555" }}>净</span>
        } else if (direction == 50) {
            return <span style={{ color: "red" }}>多</span>
        } else if (direction == 51) {
            return <span style={{ color: "green" }}>空</span>
        }
        return <span style={{ color: "green" }}>空</span>
    }


    return <tr>
        <td>{props.account}</td>
        <td>{props.exchange}</td>
        <td>{props.symbol}</td>
        <td>{i_direction(props.direction)}</td>
        <td>{props.open_cost}</td>
        <td>{props.position}</td>
    </tr>
}

export default () => {
    const [positionList, setPositionList] = useState([]);
    useEffect(() => {
        invoke('position_rows').then(res => {
            console.log('order rows', res);
            setPositionList(res as any);
        });
        async function test_listen() {
            const unlisten = await appWindow.listen('cta-event', (event: any) => {
                console.log('position table : cta-event', event);
                if (event.tp == "Position") {
                    invoke('get_position_row', { key: event.payload.key, brokerId: event.payload.b, account: event.payload.a }).then(res => {
                        console.log('get position row', res);
                    });
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
                    <col span={1} style={{ width: '10%', minWidth: "80px" }}></col>
                    <col span={1} style={{ width: '5%', minWidth: "40px" }}></col>
                    <col span={1} style={{ width: '5%', textAlign: "center" }}></col>
                    <col span={1} style={{ width: '5%' }}></col>
                    <col span={1} style={{ width: '8%' }}></col>
                    <col span={1} style={{ width: '8%' }}></col>
                    <col span={1} style={{ width: '10%' }}></col>
                    <col span={1} style={{ width: '10%' }}></col>
                    <col span={1} style={{ width: '10%' }}></col>
                    <col span={1} style={{ width: '15%' }}></col>
                    <col span={1} style={{ width: '10%' }}></col>
                </colgroup>
                <tr>
                    <th>账号</th>
                    <th>交易所</th>
                    <th>合约</th>
                    <th>多空</th>
                    <th>均价</th>
                    <th>手数</th>
                </tr>
                {positionList.map((e: any, index) => <PositionRow key={index} {...e} > </PositionRow>)}
            </table>

        </div>
    )
}