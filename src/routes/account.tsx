import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, message } from 'antd';
import React, { useState, useEffect } from 'react'
import { Outlet, Link, useNavigate } from "react-router-dom";
const { Option } = Select;

const AccountCard = (props: any) => {
	const { account, broker_id, trade_front, handleDelete } = props;
	const [messageApi, contextHolder] = message.useMessage();
	return <>
		{contextHolder}
		<Card title={account} extra={<a href="#">More</a>} style={{ width: 300 }}>
			<p>{broker_id}:{account}</p>
			<p>{trade_front}</p>
			<Button onClick={() => {
				handleDelete(broker_id, account);
			}}>删除</Button>
		</Card></>
}

export default () => {
	const [messageApi, contextHolder] = message.useMessage();
	const navigate = useNavigate();
	const [accountList, setAccountList] = useState([]);
	const [isAddOpen, setIsAddOpen] = useState(false);
	const [form] = Form.useForm();
	useEffect(() => {
		invoke('account_list').then(res => {
			setAccountList(res as any);
		});
		invoke('default_account').then(res => {
			form.setFieldsValue(res);
		});
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
			{accountList.map((e: any, index) => <AccountCard handleDelete={(broker_id: string, account: string) => {
				invoke('delete_account', { brokerId: broker_id, account }).then(res => {
					invoke('account_list').then(res => {
						setAccountList(res as any);
					});
					messageApi.info('删除账户成功');
				}).catch(err => {
					messageApi.error(err);
				});
			}} key={index} {...e} > </AccountCard>)}
			<Divider></Divider>
			<Button onClick={() => {
				console.log("添加新账户")
				setIsAddOpen(true);
			}}>添加新账户+</Button>
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