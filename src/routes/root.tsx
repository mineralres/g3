import { Outlet, Link } from "react-router-dom";
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { AppstoreOutlined, AccountBookOutlined, SettingOutlined, PlusSquareOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Menu } from 'antd';
import React, { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom';
const { SubMenu } = Menu;


const items: MenuProps['items'] = [
	{
		label: "账号管理",
		key: 'account',
		icon: <AccountBookOutlined />,
		children: [
			{
				label: <Link to={"broker"}>经纪商</Link>,
				key: 'broker-list',
				icon: <AccountBookOutlined />,
			},
			{
				label: <Link to={"account"}>交易账号</Link>,
				key: 'account-list',
				icon: <AccountBookOutlined />,
			},
		]
	},
	{
		label: <Link to={"trading"}>交易下单</Link>,
		key: 'trading',
		icon: <AppstoreOutlined />,
		children: [
			{
				label: <Link to={"order"}>报单列表</Link>,
				key: 'order-list',
				icon: <AccountBookOutlined />,
			},
			{
				label: <Link to={"instrument"}>合约列表</Link>,
				key: 'instrument',
				icon: <AccountBookOutlined />,
			},
			{
				label: <Link to={"market-data"}>行情报表</Link>,
				key: 'market-data',
				icon: <AccountBookOutlined />,
			},
			{
				label: <Link to={"position"}>持仓列表</Link>,
				key: 'position',
				icon: <AccountBookOutlined />,
			},
			{
				label: <Link to={"position-detail"}>持仓明细</Link>,
				key: 'position-detail',
				icon: <AccountBookOutlined />,
			},
			{
				label: <Link to={"trade"}>成交明细</Link>,
				key: 'trade',
				icon: <AccountBookOutlined />,
			},
		]

	},
	{
		label: '选项',
		key: 'SubMenu',
		icon: <SettingOutlined />,
		children: [
			{
				label: <Link to={"log"}>日志</Link>,
				key: 'log',
				icon: <AppstoreOutlined />,
			},
			{
				type: 'group',
				label: 'Item 1',
				children: [
					{
						label: 'Option 1',
						key: 'setting:1',
					},
					{
						label: 'Option 2',
						key: 'setting:2',
					},
				],
			}
		],
	}
];

export default () => {
	const navigate = useNavigate();
	useEffect(() => {
		async function test_listen() {
			const unlisten = await listen('window-menu-event', (event: any) => {
				const m = event.payload.message;
				console.log("m", m);
				if (m === 'broker-table') {
					navigate('broker');
				} else if (m === 'account-table') {
					navigate('account-table');
				} else if (m === 'order-table') {
					navigate('order-table');
				} else if (m === 'instrument-table') {
					navigate('instrument-table');
				} else if (m === 'trade-table') {
					navigate('trade-table');
				} else if (m === 'position-table') {
					navigate('position-table');
				} else if (m === 'position-detail-table') {
					navigate('position-detail-table');
				} else if (m === 'market-data-table') {
					navigate('market-data-table');
				}
			});
			return [unlisten];
		}
		const unlisten = test_listen();
		return () => {
			unlisten.then((ul) => ul.forEach((uf) => uf()));
		}
	});


	const onClick: MenuProps['onClick'] = ({ item, key, keyPath, domEvent }) => {
		if (key == "add-new-account" || key == "add-new-broker") {
			emit(key);
		}
		// setCurrent(key);
	};

	return (<div>
		<Outlet></Outlet>
	</div>);
}