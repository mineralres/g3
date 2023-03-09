import "../../node_modules/xterm/css/xterm.css";
import { Terminal } from 'xterm';
import React, { useState, useEffect, useRef } from 'react'
import { emit, listen, UnlistenFn } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { FitAddon } from 'xterm-addon-fit';

const fitAddon = new FitAddon();

export default () => {
	const divRef = useRef<HTMLDivElement>(null);;
	const [term, setTerm] = useState(new Terminal({
		convertEol: true,
		cursorBlink: true, // 光标闪烁
		cursorStyle: "block" // 光标样式
	}));

	const initTerminal = () => {
		const fitAddon = new FitAddon();
		term.loadAddon(fitAddon);
		if (divRef && divRef.current) {
			console.log('ref', divRef.current);
			if (divRef.current.firstChild) {
				divRef.current.removeChild(divRef.current.firstChild);
			}
			term.open(divRef.current);
			const node = divRef.current;
		}
		fitAddon.fit();
		// term.onKey(({ key, domEvent }) => {
		// 	term.write(key);
		// 	if (domEvent.keyCode === 13) {
		// 		term.write("\r\n");
		// 	}
		// });
	};

	useEffect(() => {
		initTerminal();
		async function test_listen() {
			const unlisten = await appWindow.listen('test-event', (event: any) => {
				console.log('test-event ', event);
				term.write(event.payload.message);
				term.write('\r\n');
			});
			return unlisten;
			// const unlisten = await listen('event', (event) => {
			// 	// event.event is the event name (useful if you want to use a single callback fn for multiple event types)
			// 	// event.payload is the payload object
			// 	console.log('handle ', event);
			// })
		}
		const unlisten = test_listen();
		return () => {
			unlisten.then((f) => f());
		}
	}, [divRef]);

	return (
		<div
			style={{ margin: 10, width: '800', height: '90vh' }}
			ref={divRef}
		/>

	);
}