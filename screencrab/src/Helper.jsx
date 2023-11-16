import React, {useState} from 'react';
import './styles.css';
import {invoke} from "@tauri-apps/api/tauri";
import {LogicalPosition, LogicalSize, WebviewWindow} from '@tauri-apps/api/window';
import {window} from "@tauri-apps/api";

function Helper({  }) {
    const [dragging, setDragging] = useState(false);
    const [size, setSize] = useState({ width: 0, height: 0});
    const [position, setPosition] = useState( { x: 0, y:0 });

    function handleMouseDown(event) {
        event.preventDefault();
        setDragging(true);
        setPosition({ x: event.clientX, y: event.clientY } );
        setSize({width: 0, height: 0});
    }

    function handleMouseMove(event) {
        event.preventDefault();
        if(!dragging) return;

        setSize({width: event.clientX - position.x, height: event.clientY - position.y})
    }

    async function handleMouseUp(event) {
        event.preventDefault();
        setDragging(false);
        let window = await WebviewWindow.getFocusedWindow();
        await invoke("custom_area_selection", {id: window.label, x: position.x, y: position.y, width: size.width, height: size.height}).then(() => {});
        setSize({ width: 0, height: 0});
        setPosition( { x: 0, y:0 });
    }

    return (
        <div style={{
            position: "fixed",
            width: "100%",
            height: "100%",
            margin: 0,
            padding: 0,
            overflow: "hidden",
            backgroundColor: "rgba(0, 0, 0, 0.01)",
            cursor: "crosshair",
        }}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}>
        <rect style={{
            position: "fixed",
            top: position.y,
            left: position.x,
            backgroundColor: "rgba(255,255,255,0.1)", width: size.width, height: size.height,
            cursor: "crosshair"}}
        ></rect>
    </div>);
}

export default Helper;
