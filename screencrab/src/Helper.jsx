import React, {useState} from 'react';
import './styles.css';
import {invoke} from "@tauri-apps/api/tauri";

function Helper({ imagePath }) {
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
        await invoke("custom_area_selection", {x: position.x, y: position.y, width: size.width, height: size.height}).then(() => {});
        setSize({ width: 0, height: 0});
        setPosition( { x: 0, y:0 });
    }

    return (<>
        <div style={{
            position: "fixed",
            width: "100%",
            height: "100%",
            margin: 0,
            padding: 0,
            overflow: "hidden"
        }}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}></div>
        <div style={{
            position: "fixed",
            top: position.y,
            left: position.x,
            backgroundColor: "rgba(255,255,255,0.1)", width: size.width, height: size.height}}></div>
    </>);
}

export default Helper;
