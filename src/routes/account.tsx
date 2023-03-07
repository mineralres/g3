import { invoke } from '@tauri-apps/api/tauri';
import { Card, Button } from 'antd';
import React, { useState, useEffect } from 'react'
import { Outlet, Link, useNavigate } from "react-router-dom";

export default () => {
	const navigate = useNavigate();
	const [accountList, setAccountList] = useState([]);
	useEffect(() => {
		console.log("account effect");
		invoke('account_list').then(res => {
			console.log('account list', res);
			setAccountList(res as any);
		})
	}, []);
	return (
		<div>
			{
				accountList.length == 0 ? <Button onClick={() => {
					console.log("添加新账户")
				}}>添加新账户+</Button> : <> {accountList.map((e: any, index) => <Card key={index} > {e.name} </Card>)} </>
			}
		</div>
	)
}