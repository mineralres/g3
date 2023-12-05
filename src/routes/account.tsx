import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, message, Badge } from 'antd';
import React, { useState, useEffect } from 'react'
import { Outlet, Link, useNavigate } from "react-router-dom";
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { ExclamationCircleFilled, } from '@ant-design/icons';
import { ask } from '@tauri-apps/api/dialog';
import "./account.css";
import broker from './broker';

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
		} else if (status == "LoginCompleted") {
			return <Badge status='success' text='登陆完成'></Badge>;
		}
		return <Badge status='default' text='未连接'></Badge>;
	}
	const i_profit = (profit: number) => {
		if (profit > 0) {
			return <span style={{ color: "red" }}>{profit.toFixed(2)}</span>
		} else if (profit < 0) {
			return <span style={{ color: "green" }}>{profit.toFixed(2)}</span>
		}
		return <span >{profit.toFixed(1)}</span>
	}

	return <tr>
		<td>{props.broker_name}({props.front_group_name})</td>
		<td>{account}</td>
		<td>{i_profit(props.position_profit)}</td>
		<td>{i_profit(props.closed_profit)}</td>
		<td>{equity.toFixed(2)}</td>
		<td>{props.available.toFixed(0)}</td>
		<td>{i_badge(status)}</td>
		<td>
			<Button type="link" onClick={async () => {
				props.handleEdit();
			}}>修改</Button>
			<Button type="link" onClick={async () => {
				const yes = await ask('确认删除账户?', '删除');
				if (yes) {
					handleDelete(broker_id, account);
				}
			}}>删除</Button></td>
	</tr>
}

export default () => {
	const [messageApi, contextHolder] = message.useMessage();
	const navigate = useNavigate();
	const [accountList, setAccountList] = useState([]);
	const [brokerList, setBrokerList] = useState([]);
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
				invoke('broker_list').then(res => {
					setBrokerList(res as any);
					if (!isAddOpen) {
						setIsAddOpen(true);
					}
					setIsAddOpen(true);
				});
			});
			const unlisten2 = await listen('cta-event', (event: any) => {
				console.log('account window: cta-event', event);
				if (event.tp !== "OnRtnOrder"
					&& event.tp !== "OnRtnTrade"
				) {
					invoke('account_list').then(res => {
						setAccountList(res as any);
					});
				}

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
		invoke('delete_account', { brokerId: account.broker_id, account: account.account }).then(res => {
			invoke('add_account', { account }).then(res => {
				invoke('account_list').then(res => {
					setAccountList(res as any);
				});
			}).catch(err => {
				console.log("add account err ", err)
				messageApi.error(err);
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
	const getFrontGroup = (broker_id: String) => {
		console.log("getFrontGroup ", broker_id);
		for (let i = 0; i < brokerList.length; i++) {
			let broker: any = brokerList[i];
			if (broker.broker_id === broker_id) {
				return broker.fronts;
			}
		}
		return [];
	}

	return (
		<div>
			<div style={{ float: "right" }}>
				<Button type="link" onClick={() => {
					invoke('broker_list').then(res => {
						setBrokerList(res as any);
						if (!isAddOpen) {
							setIsAddOpen(true);
						}
						setIsAddOpen(true);
					});
				}}>+添加账户</Button>
			</div>
			{contextHolder}
			<table id="customers" style={{ width: '100%' }}>
				<colgroup>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
					<col span={1} style={{ width: '10%' }}></col>
				</colgroup>
				<tr>
					<th>经纪商</th>
					<th>账号</th>
					<th>持仓盈亏</th>
					<th>平仓盈亏</th>
					<th>动态权益</th>
					<th>可用资金</th>
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
				}}
					handleEdit={() => {
						invoke('broker_list').then(res => {
							setBrokerList(res as any);
							form.setFieldsValue(e);
							if (!isAddOpen) {
								setIsAddOpen(true);
							}
							setIsAddOpen(true);
						});
					}}

					key={index} {...e} > </AccountCard>)}
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
						<Select
							placeholder="选择经纪商"
						>
							{
								brokerList.map((b: any) => {
									return <Option value={b.broker_id}>{b.name}</Option>
								})
							}
						</Select>
					</Form.Item>
					<Form.Item
						noStyle
						shouldUpdate={(prevValues, currentValues) => prevValues.broker_id !== currentValues.broker_id}
					>
						{({ getFieldValue }) =>
							<Form.Item name="front_group" label="front_group" rules={[{ required: true }]}>
								<Select
									placeholder="选择服务器"
								>
									{
										getFrontGroup(form.getFieldValue("broker_id")).map((fg: any) => {
											return <Option value={fg.id}>{fg.name}</Option>
										})
									}
								</Select>
							</Form.Item>
						}
					</Form.Item>
					<Form.Item name="account" label="Account" rules={[{ required: true }]}>
						<Input />
					</Form.Item>
					<Form.Item name="password" label="密码" rules={[{ required: true }]}>
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