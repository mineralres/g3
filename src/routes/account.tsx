import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, message, Badge } from 'antd';
import React, { useState, useEffect } from 'react'
import { Outlet, Link, useNavigate } from "react-router-dom";
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { ExclamationCircleFilled, } from '@ant-design/icons';
import "./account.css";

const { Option } = Select;
const { confirm } = Modal;

const AccountCard = (props: any) => {
	const { account, broker_id, equity, status, status_description, handleDelete } = props;
	const [messageApi, contextHolder] = message.useMessage();
	const i_badge = (status: String) => {
		if (status === "UnKown") {
			return <Badge status='default' text='未连接'></Badge>;
		} else if (status == "Connected") {
			return <Badge status='success' text='已连接'></Badge>;
		} else if (status == "Disconnected") {
			return <Badge status='error' text='已断开'></Badge>;
		} else if (status == "AuthenticateFailed") {
			return <Badge status='error' text='认证失败'></Badge>;
		} else if (status == "AuthenticateSucceeded") {
			return <Badge status='success' text='认证成功'></Badge>;
		} else if (status == "LoginFailed") {
			return <Badge status='error' text={`登陆失败(${status_description})`}></Badge>;
		} else if (status == "LoginSucceeded") {
			return <Badge status='success' text='登陆成功'></Badge>;
		}
		return <Badge status='default' text='未连接'></Badge>;
	}

	return <tr>
		<td>{broker_id}</td>
		<td>{account}</td>
		<td>{equity}</td>
		<td>{i_badge(status)}</td>
		<td><Button type="link" onClick={() => {
			let title = "确认删除?";
			confirm({
				title,
				icon: <ExclamationCircleFilled />,
				content: '删除账户',
				onOk() {
					handleDelete(broker_id, account);
				},
				onCancel() {
				},
			});
		}}>删除</Button></td>

	</tr>
}

export default () => {
	const [messageApi, contextHolder] = message.useMessage();
	const navigate = useNavigate();
	const [accountList, setAccountList] = useState([]);
	const [isAddOpen, setIsAddOpen] = useState(false);
	const [form] = Form.useForm();
	useEffect(() => {
		invoke('account_list').then(res => {
			console.log('account list', res);
			setAccountList(res as any);
		});
		invoke('default_account').then(res => {
			form.setFieldsValue(res);
		});
		async function test_listen() {
			const unlisten = await listen('add-new-account', (event: any) => {
				console.log('on add-new-account');
				if (!isAddOpen) {
					setIsAddOpen(true);
				}
				setIsAddOpen(true);
			});
			const unlisten2 = await listen('cta-event', (event: any) => {
				console.log('account window: cta-event', event);
				invoke('account_list').then(res => {
					setAccountList(res as any);
				});
			});

			return [unlisten, unlisten2];
		}
		const unlisten = test_listen();
		return () => {
			unlisten.then((ul) => ul.forEach((uf) => uf()));
		}
	}, []);
	const onFinish = (values: any) => {
		let account = form.getFieldsValue(true);
		invoke('add_account', { account }).then(res => {
			messageApi.info('添加成功');
			invoke('account_list').then(res => {
				setAccountList(res as any);
			});
		}).catch(err => {
			console.log("add account err ", err)
			messageApi.error(err);
		});
		setIsAddOpen(false);
	};

	const onReset = () => {
		invoke('default_account').then(res => {
			form.setFieldsValue(res);
		})
	};
	const layout = {
		labelCol: { span: 8 },
		wrapperCol: { span: 16 },
	};

	const tailLayout = {
		wrapperCol: { offset: 8, span: 16 },
	};

	return (
		<div>
			{contextHolder}
			<table id="customers" style={{ width: '100%' }}>
				<colgroup>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '20%' }}></col>
					<col span={1} style={{ width: '50%' }}></col>
				</colgroup>
				<tr>
					<th>BrokerId</th>
					<th>账号</th>
					<th>权益</th>
					<th>状态</th>
					<th>操作</th>
				</tr>
				{accountList.map((e: any, index) => <AccountCard handleDelete={(broker_id: string, account: string) => {
					invoke('delete_account', { brokerId: broker_id, account }).then(res => {
						invoke('account_list').then(res => {
							console.log('account list', res);
							setAccountList(res as any);
						});
						messageApi.info('删除账户成功');
					}).catch(err => {
						messageApi.error(err);
					});
				}} key={index} {...e} > </AccountCard>)}
			</table>
			<Modal title="添加账户" footer={null} open={isAddOpen} onOk={() => { setIsAddOpen(false); }} onCancel={() => { setIsAddOpen(false) }}>
				<Form
					{...layout}
					form={form}
					name="control-hooks"
					onFinish={onFinish}
					style={{ maxWidth: 600 }}
				>
					<Form.Item name="broker_id" label="BrokerID" rules={[{ required: true }]}>
						<Input />
					</Form.Item>
					<Form.Item name="account" label="Account" rules={[{ required: true }]}>
						<Input />
					</Form.Item>
					<Form.Item name="password" label="密码" rules={[{ required: true }]}>
						<Input />
					</Form.Item>
					<Form.Item name="trade_front" label="交易服务器" rules={[{ required: true }]}>
						<Input />
					</Form.Item>
					<Form.Item name="user_product_info" label="产品信息" rules={[{}]}>
						<Input />
					</Form.Item>
					<Form.Item name="auth_code" label="授权码" rules={[{}]}>
						<Input />
					</Form.Item>
					<Form.Item name="app_id" label="AppID" rules={[{}]}>
						<Input />
					</Form.Item>
					<Form.Item {...tailLayout}>
						<Button type="primary" htmlType="submit">
							提交
						</Button>
						<Button htmlType="button" onClick={onReset}>
							重置
						</Button>
					</Form.Item>
				</Form>
			</Modal>
		</div>
	)
}