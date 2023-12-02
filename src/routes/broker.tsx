import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, message, Badge } from 'antd';
import React, { useState, useEffect } from 'react'
import { Outlet, Link, useNavigate } from "react-router-dom";
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { ExclamationCircleFilled, } from '@ant-design/icons';
import { ask } from '@tauri-apps/api/dialog';
import "./account.css";

const { Option } = Select;
const { confirm } = Modal;

const BrokerRow = (props: any) => {
    return <tr>
        <td>{props.broker_id}</td>
        <td>{props.name}</td>
        <td><Button type="link" onClick={async () => {
            const yes = await ask('Are you sure?', 'Tauri');
            console.log("sure ", yes);
            let title = "确认删除?";
            confirm({
                title,
                icon: <ExclamationCircleFilled />,
                content: '删除经纪商',
                onOk() {
                    props.handleDelete(props.broker_id);
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
    const [brokerList, setBrokerList] = useState([]);
    const [isAddOpen, setIsAddOpen] = useState(false);
    const [form] = Form.useForm();
    useEffect(() => {
        invoke('broker_list').then(res => {
            console.log('account list', res);
            setBrokerList(res as any);
        });
        invoke('default_account').then(res => {
            form.setFieldsValue(res);
        });
        async function test_listen() {
            const unlisten = await listen('add-new-broker', (event: any) => {
                console.log('on add-new-broker');
                if (!isAddOpen) {
                    setIsAddOpen(true);
                }
                setIsAddOpen(true);
            });
            const unlisten2 = await listen('cta-event', (event: any) => {
            });

            return [unlisten, unlisten2];
        }
        const unlisten = test_listen();
        return () => {
            unlisten.then((ul) => ul.forEach((uf) => uf()));
        }
    }, []);
    const onFinish = (values: any) => {
        let broker = form.getFieldsValue(true);
        invoke('set_broker', { broker }).then(res => {
            messageApi.info('添加成功');
            invoke('broker_list').then(res => {
                setBrokerList(res as any);
            });
        }).catch(err => {
            console.log("add broker err ", err)
            messageApi.error(err);
        });
        setIsAddOpen(false);
    };

    const onReset = () => {
        invoke('default_broker').then(res => {
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
                    <th>名称</th>
                </tr>
                {brokerList.map((e: any, index) => <BrokerRow handleDelete={(broker_id: string) => {
                    invoke('delete_broker', { brokerId: broker_id }).then(res => {
                        invoke('broker_list').then(res => {
                            console.log('broker list', res);
                            setBrokerList(res as any);
                        });
                        messageApi.info('删除账户成功');
                    }).catch(err => {
                        messageApi.error(err);
                    });
                }} key={index} {...e} > </BrokerRow>)}
            </table>
            <Modal title="添加经纪商" footer={null} open={isAddOpen} onOk={() => { setIsAddOpen(false); }} onCancel={() => { setIsAddOpen(false) }}>
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
                    <Form.Item name="name" label="名称" rules={[{ required: true }]}>
                        <Input />
                    </Form.Item>
                    <Form.Item name="trade_fronts" label="交易服务器" rules={[{ required: true, type: "array" }]}>
                        <Select mode="tags">
                        </Select>
                    </Form.Item>
                    <Form.Item name="md_fronts" label="行情服务器" rules={[{ required: true, type: "array" }]}>
                        <Select mode="tags" >
                        </Select>
                    </Form.Item>
                    <Form.Item name="query_fronts" label="查询服务器" rules={[{ required: true, type: "array" }]}>
                        <Select mode="tags" >
                        </Select>
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