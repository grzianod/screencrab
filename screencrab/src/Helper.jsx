import React, {useEffect, useState} from 'react';
import './styles.css';
import {invoke} from "@tauri-apps/api/tauri";
import {LogicalPosition, LogicalSize, WebviewWindow} from '@tauri-apps/api/window';
import {window} from "@tauri-apps/api";

function Helper({  }) {
    const [dragging, setDragging] = useState(false);
    const [size, setSize] = useState({ width: 0, height: 0});
    const [position, setPosition] = useState( { left: 0, top: 0 });
    const [direction, setDirection] = useState("SE");

    function handleMouseDown(event) {
        event.preventDefault();
        setDragging(true);
        setPosition({ left: event.clientX, top: event.clientY } );
        setSize({width: 0, height: 0});
    }

    function handleMouseMove(event) {
        event.preventDefault();
        if((event.clientX - position.left) > 0 && (event.clientY - position.top) < 0)
            setDirection("NE");
        if((event.clientX - position.left) > 0 && (event.clientY - position.top) > 0)
            setDirection("SE");
        if((event.clientX - position.left) < 0 && (event.clientY - position.top) > 0)
            setDirection("SO");
        if((event.clientX - position.left) < 0 && (event.clientY - position.top) < 0)
            setDirection("NO");
        if(!dragging) return;

        setSize({width: Math.abs(event.clientX - position.left), height: Math.abs(event.clientY - position.top)})
    }

    async function handleMouseUp(event) {
        event.preventDefault();
        setDragging(false);
        let window = await WebviewWindow.getFocusedWindow();
        await invoke("custom_area_selection", {
            id: window.label,
            left: (direction === "NO" || direction === "SO") ? (position.left - size.width) : position.left,
            top: (direction === "NO" || direction === "NE") ? (position.top - size.height) :  position.top,
            width: size.width,
            height: size.height
        });
        setSize({ width: 0, height: 0});
        setPosition( { left: 0, top:0 });
    }

    useEffect(() => {
        document.addEventListener("keyup", async (event) => {
           if(event.key === "Escape") {
               await invoke("hide_all_helpers", {});
           }
        });
    }, []);

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
                position: "absolute",
                top: (direction === "NO" || direction === "NE") ? position.top - size.height : (direction === "SE" || direction === "SO") ? position.top : false,
                left: (direction === "NO" || direction === "SO") ? position.left - size.width : (direction === "SE" || direction === "NE") ? position.left : false,
                backgroundColor: "rgba(255,255,255,0.1)",
                width: size.width,
                height: size.height,
                border: "0.1vmin dashed antiquewhite",
                cursor: "crosshair"}}
            ></rect>

        </div>);
}

export default Helper;
