import { Outlet, Link } from "react-router-dom";
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { AppstoreOutlined, AccountBookOutlined, SettingOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Menu } from 'antd';
import React, { useState, useEffect } from 'react'
const { SubMenu } = Menu;


const items: MenuProps['items'] = [
	{
		label: <Link to={"account"}>账号管理</Link>,
		key: 'account',
		icon: <AccountBookOutlined />
	},
	{
		label: <Link to={"trading"}>交易下单</Link>,
		key: 'trading',
		icon: <AppstoreOutlined />,
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
	console.log("root");
	useEffect(() => {
		// async function test_listen() {
		// 	appWindow.listen('test-event', (event: any) => {
		// 		console.log('test-event ', event);
		// 	});
		// 	const unlisten = await listen('event', (event) => {
		// 		// event.event is the event name (useful if you want to use a single callback fn for multiple event types)
		// 		// event.payload is the payload object
		// 		console.log('handle ', event);
		// 	})
		// }
		// test_listen();
	});

	const onClick: MenuProps['onClick'] = (e) => {
		setCurrent(e.key);
	};

	return (<div>
		<Menu onClick={onClick} selectedKeys={[current]} mode="horizontal" items={items} ></Menu>
		<Outlet></Outlet>
	</div>);
}