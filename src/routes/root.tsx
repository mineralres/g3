import { Outlet, Link } from "react-router-dom";
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { AppstoreOutlined, AccountBookOutlined, SettingOutlined, PlusSquareOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Menu } from 'antd';
import React, { useState, useEffect } from 'react'
const { SubMenu } = Menu;


const items: MenuProps['items'] = [
	{
		label: "账号",
		key: 'account',
		icon: <AccountBookOutlined />,
		children: [
			{
				label: <Link to={"account"}>账号列表</Link>,
				key: 'account-list',
				icon: <AccountBookOutlined />,
			},
			{
				label: "添加账号",
				key: 'add-new-account',
				icon: <PlusSquareOutlined />,
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
	const [current, setCurrent] = useState('account');
	useEffect(() => {
	});

	const onClick: MenuProps['onClick'] = ({ item, key, keyPath, domEvent }) => {
		if (key == "add-new-account") {
			emit(key);
		}
		// setCurrent(key);
	};

	return (<div>
		<Menu onClick={onClick} mode="horizontal" items={items} ></Menu>
		<Outlet></Outlet>
	</div>);
}