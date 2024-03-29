import "../../node_modules/xterm/css/xterm.css";
import { Terminal } from 'xterm';
import React, { useState, useEffect, useRef } from 'react'
import { emit, listen, UnlistenFn } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { FitAddon } from 'xterm-addon-fit';

export default () => {
	const divRef = useRef<HTMLDivElement>(null);;
	const [term, setTerm] = useState(new Terminal({
		convertEol: true,
	}));

	const initTerminal = () => {
		if (divRef && divRef.current) {
			if (divRef.current.firstChild) {
				divRef.current.removeChild(divRef.current.firstChild);
			}
			term.open(divRef.current);
			const node = divRef.current;
		}
	};

	useEffect(() => {
		initTerminal();
		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);
		fitAddon.fit();
		async function test_listen() {
			const unlisten = await appWindow.listen('new-log-line', (event: any) => {
				term.write(event.payload.message);
			});
			return unlisten;
		}
		const unlisten = test_listen();
		return () => {
			unlisten.then((f) => f());
		}
	}, [divRef]);

	return (
		<div id="log"
			style={{ margin: 1, width: '100%', height: '800' }}
			ref={divRef}
		/>

	);
}