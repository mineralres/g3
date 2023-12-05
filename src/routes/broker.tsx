import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button, Modal, Form, Input, Select, Divider, message, Badge, Space } from 'antd';
import React, { useState, useEffect } from 'react'
import { Outlet, Link, useNavigate } from "react-router-dom";
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { ExclamationCircleFilled, CloseOutlined } from '@ant-design/icons';
import { ask } from '@tauri-apps/api/dialog';
import "./account.css";

const BrokerRow = (props: any) => {
    return <tr>
        <td>{props.broker_id}</td>
        <td>{props.name}</td>
        <td>{props.user_product_info}</td>
        <td>{props.auth_code}</td>
        <td>{props.app_id}</td>
        <td>
            {props.fronts.map((e: any, index: number) => {
                return <Card title={e.name} size='small'>
                    {/* <p>
                        <span>交易服务器</span>
                        <span>{e.trade_front}</span>
                    </p>
                    <p>
                        <span>交易服务器</span>
                        <span>{e.trade_front}</span>
                    </p>
                    <p>
                        <span>交易服务器</span>
                        <span>{e.trade_front}</span>
                    </p> */}
                    <table>
                        <tr>
                            <td>交易服务器</td>
                            <td>{e.trade_front}</td>
                        </tr>
                        <tr>
                            <td>行情服务器</td>
                            <td>{e.md_front}</td>
                        </tr>
                        <tr>
                            <td>查询服务器</td>
                            <td>{e.query_front}</td>
                        </tr>
                    </table>
                </Card>
            })}
        </td>
        <td><Button type="link" onClick={async () => {
            props.handleEdit();
        }}>修改</Button>
            <Button type="link" onClick={async () => {
                const yes = await ask('确定删除吗?', '删除');
                if (yes) {
                    props.handleDelete(props.broker_id);
                }
            }}>删除</Button>
        </td>
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
        console.log("value", values);
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
            <div style={{ float: "right" }}>
                <Button type="link" onClick={() => {
                    if (!isAddOpen) {
                        setIsAddOpen(true);
                    }
                    setIsAddOpen(true);
                }}>+添加经纪商</Button>
            </div>
            <table id="customers" style={{ width: '100%' }}>
                <colgroup>
                    <col span={1} style={{ width: '5%' }}></col>
                    <col span={1} style={{ width: '5%' }}></col>
                    <col span={1} style={{ width: '10%' }}></col>
                    <col span={1} style={{ width: '10%' }}></col>
                    <col span={1} style={{ width: '10%' }}></col>
                    <col span={1} style={{ width: '30%' }}></col>
                </colgroup>
                <tr>
                    <th>BrokerId</th>
                    <th>名称</th>
                    <th>UserProductInfo</th>
                    <th>AuthCode</th>
                    <th>AppId</th>
                    <th>服务器</th>
                    <th>操作</th>
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
                }}
                    handleEdit={() => {
                        form.setFieldsValue(e);
                        setIsAddOpen(true);
                    }}

                    key={index} {...e} > </BrokerRow>)}
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
                    <Form.Item name="user_product_info" label="产品信息" rules={[{}]}>
                        <Input />
                    </Form.Item>
                    <Form.Item name="auth_code" label="授权码" rules={[{}]}>
                        <Input />
                    </Form.Item>
                    <Form.Item name="app_id" label="AppID" rules={[{}]}>
                        <Input />
                    </Form.Item>
                    <Form.List name="fronts">
                        {(fields, { add, remove }) => (
                            <div style={{ display: 'flex', rowGap: 16, flexDirection: 'column' }}>
                                {fields.map((field) => (
                                    <Card
                                        size="small"
                                        title={`服务器`}
                                        key={field.key}
                                        extra={
                                            <CloseOutlined
                                                onClick={() => {
                                                    remove(field.name);
                                                }}
                                            />
                                        }
                                    >
                                        <Form.Item label="ID" name={[field.name, 'id']}>
                                            <Input />
                                        </Form.Item>
                                        <Form.Item label="名称" name={[field.name, 'name']}>
                                            <Input />
                                        </Form.Item>
                                        <Form.Item label="交易服务器" name={[field.name, 'trade_front']}>
                                            <Input />
                                        </Form.Item>
                                        <Form.Item label="行情服务器" name={[field.name, 'md_front']}>
                                            <Input />
                                        </Form.Item>
                                        <Form.Item label="查询服务器" name={[field.name, 'query_front']}>
                                            <Input />
                                        </Form.Item>
                                    </Card>
                                ))}

                                <Button type="dashed" onClick={() => add()} block>
                                    + 添加服务器组
                                </Button>
                            </div>
                        )}
                    </Form.List>
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