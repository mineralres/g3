import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, Badge } from 'antd';
import React, { useState, useEffect, useMemo } from 'react'
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
    const t_product = (pt: number) => {
        if (pt == 49) {
            return "期货";
        } else if (pt == 50) {
            return "期货期权";
        } else if (pt == 51) {
            return "组合";
        } else if (pt == 52) {
            return "即期";
        } else if (pt == 53) {
            return "期转现";
        } else if (pt == 54) {
            return "现货期权";
        } else if (pt == 55) {
            return "TAS合约";
        } else if (pt == 56) {
            return "金属指数";
        }
        return pt;
        ///期货
        // #define THOST_FTDC_PC_Futures '1'
        // ///期货期权
        // #define THOST_FTDC_PC_Options '2'
        // ///组合
        // #define THOST_FTDC_PC_Combination '3'
        // ///即期
        // #define THOST_FTDC_PC_Spot '4'
        // ///期转现
        // #define THOST_FTDC_PC_EFP '5'
        // ///现货期权
        // #define THOST_FTDC_PC_SpotOption '6'
        // ///TAS合约
        // #define THOST_FTDC_PC_TAS '7'
        // ///金属指数
        // #define THOST_FTDC_PC_MI 'I'
    }
    return <tr>
        <td>{props.index + 1}</td>
        <td>{props.exchange}</td>
        <td>{props.symbol}</td>
        <td>{props.name}</td>
        <td>{props.volume_multiple}</td>
        <td>{props.price_tick}</td>
        <td>{t_product(props.product_type)}</td>
        <td>{props.expire_date}</td>
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
                    <th>品种类型</th>
                    <th>最后日期</th>
                    <th>保证金</th>
                    <th>手续费</th>
                </tr>
                {instrumentList.map((e: any, index) => <InstrumentRow index={index} key={index} {...e} > </InstrumentRow>)}
            </table>

        </div >
    )
}