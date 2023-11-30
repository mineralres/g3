import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, message, Badge } from 'antd';
import React, { useState, useEffect } from 'react'
import { Outlet, Link, useNavigate } from "react-router-dom";
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

const OrderRow = (props: any) => {
	const { account, broker_id, status, order_sys_id, symbol, direction, offset, limit_price, volume } = props;
	const [messageApi, contextHolder] = message.useMessage();
	const i_badge = (status: Number) => {
		if (status == 48) {
			return <Badge status='success' text="完全成交"></Badge>;
		} else if (status == 53) {
			return <Badge status='warning' text="已撤单"></Badge>;
		}
		return <Badge status='default' text={`${status}`}></Badge>;
	}
	const i_direction = (direction: number) => {
		if (direction == 48) {
			return <span style={{ color: "red" }}>多</span>
		}
		return <span style={{ color: "green" }}>空</span>
	}

	const i_offset = (offset: number) => {
		if (offset == 49) {
			return "平仓";
		}
		return "开仓";
	}

	return <tr>
		<td>{order_sys_id}</td>
		<td>{symbol}</td>
		<td>{i_direction(direction)}</td>
		<td>{i_offset(offset)}</td>
		<td>{i_badge(status)}</td>
		<td>{limit_price}</td>
		<td>{props.volume_total_original}</td>
		<td>{props.volume_total_original - props.volume_traded}</td>
		<td>{props.volume_traded}</td>
		<td>{truncateString(props.status_description, 15)}</td>
	</tr>
}

export default () => {
	const [messageApi, contextHolder] = message.useMessage();
	const [orderList, setOrderList] = useState([]);
	useEffect(() => {
		invoke('order_rows').then(res => {
			console.log('order rows', res);
			setOrderList(res as any);
		});
		async function test_listen() {
			const unlisten = await appWindow.listen('cta-event', (event: any) => {
				console.log('order table : cta-event', event);
				if (event.tp == "Order") {
					invoke('get_order_row', { key: event.payload.key, brokerId: event.payload.b, account: event.payload.a }).then(res => {
						console.log('get order row', res);
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
			{contextHolder}
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
					<th>报单编号</th>
					<th>合约</th>
					<th>多空</th>
					<th>开平</th>
					<th>状态</th>
					<th>报单价格</th>
					<th>手数</th>
					<th>未成交</th>
					<th>已成交</th>
					<th>详细状态</th>
					<th>报单时间</th>
				</tr>
				{orderList.map((e: any, index) => <OrderRow key={index} {...e} > </OrderRow>)}
			</table>

		</div>
	)
}