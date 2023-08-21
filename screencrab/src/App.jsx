import {useEffect, useState} from "react";
import 'bootstrap/dist/css/bootstrap.min.css';
import { invoke } from "@tauri-apps/api/tauri";
import {emit, listen, once} from '@tauri-apps/api/event'
import {Container, Button, FormText, Form, Dropdown} from "react-bootstrap";
import "./App.css";
import isEmpty from "validator/es/lib/isEmpty.js";
import { WebviewWindow } from '@tauri-apps/api/window';
import HotkeysMenu from "./HotkeysMenu.jsx";

function App() {
  const [mode, setMode] = useState("capture");
  const [view, setView] = useState("fullscreen");
  const [duration, setDuration] = useState(0);
  const [pointer, setPointer] = useState(false);
  const [text, setText] = useState(undefined);
  const [filePath, setFilePath] = useState("");
  const [countdown, setCountdown] = useState(0);
  const [capturing, setCapturing] = useState(false);
  const [isCounting, setIsCounting] = useState(false);
  const [captureType, setCaptureType] = useState("png");
  const [recordType, setRecordType] = useState("mov");
  const [clipboard, setClipboard] = useState(false);
  const [openFile, setOpenFile] = useState(true);
  const [externalAudio, setExternalAudio] = useState(false);
  const [hotkeys, setHotkeys] = useState(false);


    async function capture(mode, view, duration, pointer, filePath, fileType, clipboard, openFile) {
        let selector = WebviewWindow.getByLabel('selector');
        let area = "";
        setCountdown(duration);
        setIsCounting(true);
        setCapturing(true);

        if(view === "custom") {
            await selector.hide();
            let position = await selector.innerPosition();
            let size = await selector.innerSize();
            let scaleFactor = await selector.scaleFactor();
            area = position.toLogical(scaleFactor).x + "," + position.toLogical(scaleFactor).y + "," + size.toLogical(scaleFactor).width + "," + size.toLogical(scaleFactor).height;
        }

            invoke("capture", {
                mode: mode,
                view: view,
                area: area,
                timer: duration,
                pointer: pointer,
                file_path: filePath,
                file_type: fileType,
                clipboard: clipboard,
                audio: externalAudio,
                open_file: openFile
            })
            .then( (response) => {
                setText(response.response || response.error)
                setTimeout(() => setText(undefined), 5000);
            })
            .catch((err) => {
                setText(err);
                setTimeout(() => setText(undefined), 5000);
            })
            .finally(() => {
                    if(view === "custom") {
                        selector.setResizable(true);
                        selector.hide();
                    }

                    setCapturing(false);
                    setView("fullscreen");
                });
    }

    useEffect( () => {

        const handleCountdown = async () => {
            if (countdown > 0) {
                await new Promise(resolve => setTimeout(resolve, 1000));
                setCountdown(countdown => countdown-1);
            }

            if (countdown <= 0 && isCounting ) {
                setCountdown(0);
                setIsCounting(false);
            }
        }

        handleCountdown().then(() => {});

    }, [countdown, isCounting]);

    async function stopCapture() {
        setCountdown(0);
        setIsCounting(false);

        if(mode === "capture")
            emit("kill", {})
                .then( () => {})
                .catch((err) => console.log("ERROR: "+err) /* TODO: handle error */)
        else if (capturing)
            emit("stop", {})
                .then( () => {})
                .catch((err) => console.log("ERROR: "+err) /* TODO: handle error */)
                .finally(() => setCapturing(false));
        else
            emit("kill", {})
                .then( () => {})
                .catch((err) => console.log("ERROR: "+err) /* TODO: handle error */)
    }

    async function openFolderDialog() {
        const result = await invoke("folder_dialog");
        if(result.response)
            setFilePath(result.response);
        else
            console.log(result.error); /* TODO: handle error */

    }

    async function setCaptureFullscreen() {
        setMode("capture");
        setView("fullscreen");
        await WebviewWindow.getByLabel('selector').hide();
    }

    async function setCaptureCustom() {
        setMode("capture");
        setView("custom");
        await WebviewWindow.getByLabel('selector').show()
    }

    async function setRecordFullscreen() {
        setMode("record");
        setView("fullscreen");
        await WebviewWindow.getByLabel('selector').hide()
        setClipboard(false);
    }

    async function setRecordCustom() {
        setMode("record");
        setView("custom");
        await WebviewWindow.getByLabel('selector').show()
        setClipboard(false);
    }

    useEffect(() => {
        const promise = listen("capture_mouse_pointer", () => {
            setPointer((pointer) => !pointer);
        });
        return () => promise.then(remove => remove());
    });

    useEffect(() => {
        const promise = listen("copy_to_clipboard", (event) => {
            if(mode === "capture") {
                setClipboard((clipboard) => !clipboard);
                setOpenFile(false);
            }
        });
        return () => promise.then(remove => remove());
    });

    useEffect(() => {
        let stream;
        const promise = listen("record_external_audio", (event) => {
            navigator.mediaDevices.getUserMedia({audio: !externalAudio}).then((s) => stream = s);
            setExternalAudio((externalAudio) => !externalAudio);
        });
        return () => promise.then(remove => remove());
    });

    useEffect(() => {
        const promise = listen("open_after_record", () => {
            setOpenFile((openFile) => !openFile);
        });
        return () => promise.then(remove => remove());
    });
    useEffect(() => {
        const promise = listen("edit_after_capture", () => {
            setOpenFile((openFile) => !openFile);
        });
        return () => promise.then(remove => remove());
    });

    useEffect(() => {
        const promise = listen("fullscreen_capture", async () => {
            setCaptureFullscreen().then(async () => await capture("capture", "fullscreen", duration, pointer, filePath, captureType, clipboard, openFile));
        });
            return () => promise.then(remove => remove());
        });
    useEffect(() => {
        const promise = listen("custom_capture", async () => {
            WebviewWindow.getByLabel('selector').isVisible().then( async (value) => {
                if(value) await capture("capture", "custom", duration, pointer, filePath, captureType, clipboard, openFile);
                else await setCaptureCustom();
            })
        });
        return () => promise.then(remove => remove());
    });
    useEffect(() => {
        const promise = listen("fullscreen_record", async () => {
            setRecordFullscreen().then(async () => await capture("record", "fullscreen", duration, pointer, filePath, recordType, clipboard, openFile));
        });
        return () => promise.then(remove => remove());
    });
    useEffect(() => {
        const promise = listen("custom_record", async () => {
            WebviewWindow.getByLabel('selector').isVisible().then( async (value) => {
                if(value) await capture("record", "custom", duration, pointer, filePath, recordType, clipboard, openFile);
                else await setRecordCustom();
            })
        });
        return () => promise.then(remove => remove());
    });
    useEffect(() => {
        const promise = listen("stop_recording", async () => await stopCapture() );
        return () => promise.then(remove => remove());
    });
    useEffect(() => {
        const promise = listen("change_hotkeys", () => {
            setHotkeys((hotkeys) => !hotkeys);
        });
        return () => promise.then(remove => remove());
    });
    
    useEffect( () => {

                invoke("current_default_path")
                    .then((result) => {
                        if (result.response)
                            setFilePath(result.response);
                        else
                            console.log("ERROR"); /* TODO: handle path retrieve error */
                    })
                    .catch(() => console.log("ERROR") /* TODO: handle error */);
    }, []);

    function reserve(event) {
        event.preventDefault();

        if (isEmpty(event.target.value)) {
            setDuration(0);
            return;
        }

        setDuration(parseInt(event.target.value));
    }


    return (hotkeys ?
            <>
            <HotkeysMenu></HotkeysMenu>
                <Button style={{position:"absolute", top: "1rem", right: "1rem"}} variant={"light"} className={"mx-2"} onClick={() => setHotkeys((hotkeys) => !hotkeys)}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                         className="bi bi-arrow-left" viewBox="0 0 16 16">
                        <path fillRule="evenodd"
                              d="M15 8a.5.5 0 0 0-.5-.5H2.707l3.147-3.146a.5.5 0 1 0-.708-.708l-4 4a.5.5 0 0 0 0 .708l4 4a.5.5 0 0 0 .708-.708L2.707 8.5H14.5A.5.5 0 0 0 15 8z"/>
                    </svg>
                </Button>
                </>
            :
            <>
            <Button style={{position:"absolute", top: "1rem", right: "1rem"}} variant={"light"} className={"mx-2"} onClick={() => setHotkeys((hotkeys) => !hotkeys)}>
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                     className="bi bi-sliders" viewBox="0 0 16 16">
                    <path fillRule="evenodd"
                          d="M11.5 2a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zM9.05 3a2.5 2.5 0 0 1 4.9 0H16v1h-2.05a2.5 2.5 0 0 1-4.9 0H0V3h9.05zM4.5 7a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zM2.05 8a2.5 2.5 0 0 1 4.9 0H16v1H6.95a2.5 2.5 0 0 1-4.9 0H0V8h2.05zm9.45 4a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zm-2.45 1a2.5 2.5 0 0 1 4.9 0H16v1h-2.05a2.5 2.5 0 0 1-4.9 0H0v-1h9.05z"/>
                </svg>
            </Button>

                <Container className="background-container p-0 m-0"></Container>

                <Container className={"flex-row justify-content-center p-0 m-0 w-100"}>
                    <div className={"col-2"}></div>

                    <Container style={{zIndex: "2", position: "relative"}} className={"w-100 align-items-center"}>
                        <Container className={"flex-row align-items-center w-100 mb-2 justify-content-center"}>
                            <FormText>Save to:</FormText>
                                    <Form.Control
                                        className={"mx-2"}
                                        style={{width: "30rem"}}
                                        type="text"
                                        value={clipboard ? "Clipboard" : filePath}
                                        disabled={clipboard || capturing}
                                        onChange={ (event) => setFilePath(event.target.value)}
                                    />
                            <Button variant={"light"} className={"me-2"} disabled={clipboard} onClick={openFolderDialog}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                                     className="bi bi-folder-plus" viewBox="0 0 16 16">
                                    <path
                                        d="m.5 3 .04.87a1.99 1.99 0 0 0-.342 1.311l.637 7A2 2 0 0 0 2.826 14H9v-1H2.826a1 1 0 0 1-.995-.91l-.637-7A1 1 0 0 1 2.19 4h11.62a1 1 0 0 1 .996 1.09L14.54 8h1.005l.256-2.819A2 2 0 0 0 13.81 3H9.828a2 2 0 0 1-1.414-.586l-.828-.828A2 2 0 0 0 6.172 1H2.5a2 2 0 0 0-2 2Zm5.672-1a1 1 0 0 1 .707.293L7.586 3H2.19c-.24 0-.47.042-.683.12L1.5 2.98a1 1 0 0 1 1-.98h3.672Z"/>
                                    <path
                                        d="M13.5 9a.5.5 0 0 1 .5.5V11h1.5a.5.5 0 1 1 0 1H14v1.5a.5.5 0 1 1-1 0V12h-1.5a.5.5 0 0 1 0-1H13V9.5a.5.5 0 0 1 .5-.5Z"/>
                                </svg>
                            </Button>
                        </Container>

                        <Container className={"flex-row align-items-center p-0"}>

                            <Container className={"d-flex flex-column align-items-center p-0 justify-content-center p-0 mx-2 w-auto"}>
                                <FormText>Capture</FormText>
                                <Container className={"d-flex flex-row p-0"}>
                                    <Button className={"m-1"} variant={mode === "capture" && view === "fullscreen" ? "primary" : "outline-primary"}
                                            disabled={capturing}
                                            title={"Capture Entire Screen"}
                                            onClick={setCaptureFullscreen}>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                                             className="bi bi-window-desktop" viewBox="0 0 16 16">
                                            <path d="M3.5 11a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-9Z"/>
                                            <path
                                                d="M2.375 1A2.366 2.366 0 0 0 0 3.357v9.286A2.366 2.366 0 0 0 2.375 15h11.25A2.366 2.366 0 0 0 16 12.643V3.357A2.366 2.366 0 0 0 13.625 1H2.375ZM1 3.357C1 2.612 1.611 2 2.375 2h11.25C14.389 2 15 2.612 15 3.357V4H1v-.643ZM1 5h14v7.643c0 .745-.611 1.357-1.375 1.357H2.375A1.366 1.366 0 0 1 1 12.643V5Z"/>
                                        </svg>
                                    </Button>

                                    <Button className={"m-1"} variant={mode === "capture" && view === "custom" ? "primary" : "outline-primary"}
                                            title={"Capture Selected Portion"}
                                            disabled={capturing}
                                            onClick={setCaptureCustom}>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                                             className="bi bi-fullscreen" viewBox="0 0 16 16">
                                            <path
                                                d="M1.5 1a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0v-4A1.5 1.5 0 0 1 1.5 0h4a.5.5 0 0 1 0 1h-4zM10 .5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 16 1.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5zM.5 10a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 0 14.5v-4a.5.5 0 0 1 .5-.5zm15 0a.5.5 0 0 1 .5.5v4a1.5 1.5 0 0 1-1.5 1.5h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5z"/>
                                        </svg></Button>
                                </Container>
                            </Container>


                            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 mx-2 w-auto"}>
                                <FormText className={"title-record"}>Record</FormText>
                                <Container className={"d-flex flex-row p-0"}>
                                    <Button title={"Record Entire Screen"} className={"m-1"}
                                            disabled={capturing}
                                            variant={mode === "record" && view === "fullscreen" ? "danger" : "outline-danger"}
                                            onClick={setRecordFullscreen}>
                                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                                             className="bi bi-window-desktop" viewBox="0 0 16 16">
                                            <path d="M3.5 11a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-9Z"/>
                                            <path
                                                d="M2.375 1A2.366 2.366 0 0 0 0 3.357v9.286A2.366 2.366 0 0 0 2.375 15h11.25A2.366 2.366 0 0 0 16 12.643V3.357A2.366 2.366 0 0 0 13.625 1H2.375ZM1 3.357C1 2.612 1.611 2 2.375 2h11.25C14.389 2 15 2.612 15 3.357V4H1v-.643ZM1 5h14v7.643c0 .745-.611 1.357-1.375 1.357H2.375A1.366 1.366 0 0 1 1 12.643V5Z"/>
                                        </svg>
                                    </Button>

                                    <Button
                                        className={"m-1"}
                                        variant={mode === "record" && view === "custom" ? "danger" : "outline-danger"}
                                        title={"Record Selected Portion"}
                                        disabled={capturing}
                                        onClick={setRecordCustom}>
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="20"
                                            height="20"
                                            fill="currentColor"
                                            style={{ zIndex: "1"}}
                                            className="bi bi-fullscreen"
                                            viewBox="0 0 16 16"
                                        >
                                            <path
                                                d="M1.5 1a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0v-4A1.5 1.5 0 0 1 1.5 0h4a.5.5 0 0 1 0 1h-4zM10 .5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 16 1.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5zM.5 10a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 0 14.5v-4a.5.5 0 0 1 .5-.5zm15 0a.5.5 0 0 1 .5.5v4a1.5 1.5 0 0 1-1.5 1.5h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5z"/>

                                        </svg>
                                    </Button>

                                </Container>
                            </Container>

                            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 mx-4 w-auto"}>
                                <FormText className={countdown > 0 ? "blink" : ""}>Timer [s]</FormText>
                                <Form.Control
                                    type={"text"}
                                    step={1}
                                    min={0}
                                    value={countdown > 0 ? countdown : duration}
                                    className={countdown > 0 ? "blink" : ""}
                                    style={{
                                        display: "inline-block", textAlign: "center", maxWidth: "7rem"
                                    }}
                                    onChange={reserve}></Form.Control>

                            </Container>

                            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 mx-2 w-auto"}>
                                <FormText>&nbsp;</FormText>
                                <Dropdown drop={"down-centered"}>
                                    <Dropdown.Toggle disabled={!mode || capturing} variant="light">
                                        <FormText>Format</FormText>
                                    </Dropdown.Toggle>

                                    <Dropdown.Menu style={{columnCount: "2"}}>
                                        <div className={"column"}>
                                            <Dropdown.Item onClick={() => setCaptureType("pdf")} className={mode==="record" ? "d-none" : false}>
                                                <FormText>pdf</FormText>
                                                {captureType === "pdf" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                            <Dropdown.Item onClick={() => setCaptureType("jpeg")} className={mode==="record" ? "d-none" : false}>
                                                <FormText >jpeg</FormText>
                                                {captureType === "jpeg" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                            <Dropdown.Item onClick={() => setCaptureType("png")} className={mode==="record" ? "d-none" : false}>
                                                <FormText >png</FormText>
                                                {captureType === "png" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                        </div>
                                        <div className={"column"}>
                                            <Dropdown.Item onClick={() => setCaptureType("tiff")} className={mode==="record" ? "d-none" : false}>
                                                <FormText>tiff</FormText>
                                                {captureType === "tiff" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                            <Dropdown.Item onClick={() => setCaptureType("bmp")} className={mode==="record" ? "d-none" : false}>
                                                <FormText>bmp</FormText>
                                                {captureType === "bmp" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                            <Dropdown.Item onClick={() => setRecordType("mov")} className={mode==="capture" ? "d-none" : false}>
                                                <FormText>mov</FormText>
                                                {recordType === "mov" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                            <Dropdown.Item onClick={() => setRecordType("mp4")} className={mode==="capture" ? "d-none" : false}>
                                                <FormText>mp4</FormText>
                                                {recordType === "mp4" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                            <Dropdown.Item onClick={() => setRecordType("avi")} className={mode==="capture" ? "d-none" : false}>
                                                <FormText>avi</FormText>
                                                {recordType === "avi" ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                            <Dropdown.Item onClick={() => mode === "capture" ? setCaptureType("gif") : setRecordType("gif")}>
                                                <FormText>gif</FormText>
                                                {(mode === "record" && recordType === "gif") || (mode === "capture" && captureType === "gif") ?
                                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                         className="bi bi-check" viewBox="0 0 16 16">
                                                        <path
                                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                    </svg> : false}
                                            </Dropdown.Item>
                                        </div>
                                    </Dropdown.Menu>
                                </Dropdown>
                            </Container>


                            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 mx-2 w-auto"}>
                                <FormText className={countdown > 0 ? "blink" : ""}>&nbsp;</FormText>
                                <Dropdown drop={"down-centered"}>
                                    <Dropdown.Toggle className={"mx-2"} disabled={capturing} variant="light" id="dropdown-basic">
                                        <FormText>Options</FormText>
                                    </Dropdown.Toggle>

                                    <Dropdown.Menu>
                                        {mode !== "record" ? <Dropdown.Item onClick={() => setPointer((pointer) => !pointer)}>
                                            <FormText>Capture Mouse Pointer</FormText>
                                            {pointer ?
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                     className="bi bi-check" viewBox="0 0 16 16">
                                                    <path
                                                        d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                </svg> : false}
                                        </Dropdown.Item> : false }
                                        {mode !== "record" ? <Dropdown.Item onClick={() => {setClipboard((clipboard) => !clipboard);
                                            setOpenFile(false);
                                        }}>
                                            <FormText>Copy To Clipboard</FormText>
                                            {clipboard ?
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                     className="bi bi-check" viewBox="0 0 16 16">
                                                    <path
                                                        d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                </svg> : false}
                                        </Dropdown.Item> : false }
                                        {mode === "record" ? <Dropdown.Item onClick={async () => {
                                            navigator.mediaDevices.getUserMedia({audio: !externalAudio}).then((s) => stream = s);
                                            setExternalAudio((externalAudio) => !externalAudio);
                                        } }>
                                            <FormText>Record External Audio</FormText>
                                            {externalAudio ?
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                     className="bi bi-check" viewBox="0 0 16 16">
                                                    <path
                                                        d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                </svg> : false}
                                        </Dropdown.Item> : false }
                                        <Dropdown.Item onClick={() => setOpenFile((openFile) => !openFile)}>
                                            <FormText>{mode === "capture" ? "Edit After Capture" : "Open After Record"}</FormText>
                                            {openFile ?
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                                     className="bi bi-check" viewBox="0 0 16 16">
                                                    <path
                                                        d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                                </svg> : false}
                                        </Dropdown.Item>
                                    </Dropdown.Menu>
                                </Dropdown>
                            </Container>

                            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 mx-2 w-auto"}>
                                <FormText>&nbsp;</FormText>
                                { countdown > 0 ? <Button className={"m-1"} variant={"danger"} onClick={stopCapture}>Cancel</Button> :
                                    mode==="record" && capturing ? <Button className={"m-1"} variant={"danger"} onClick={stopCapture}>Stop</Button> :
                                        (mode !== undefined && view !== undefined) ? <Button className={"m-1"} variant={mode === "capture" ? "primary" : "danger"} onClick={async () => await capture(mode, view, duration, pointer, filePath, mode==="capture" ? captureType : recordType, clipboard, openFile)}>{mode[0].toUpperCase() + mode.slice(1)}</Button> : false}
                            </Container>

                        </Container>
                        <Container className={"justify-content-center p-0 m-1"}>
                            <FormText>{text}</FormText>
                        </Container>
                    </Container>



                </Container>
            </>);

}


export default App;
