import {useEffect, useState} from "react";
import 'bootstrap/dist/css/bootstrap.min.css';
import { invoke } from "@tauri-apps/api/tauri";
import {emit, listen, once} from '@tauri-apps/api/event'
import {Container, Button, FormText, Form, Dropdown} from "react-bootstrap";
import "./App.css";
import isEmpty from "validator/es/lib/isEmpty.js";
import { WebviewWindow } from '@tauri-apps/api/window';

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
  const [fileType, setFileType] = useState("png");
  const [clipboard, setClipboard] = useState(false);
  const [audio, setAudio] = useState(false);
  const [openFile, setOpenFile] = useState(true);

    async function capture() {
        let selector = WebviewWindow.getByLabel('selector');
        let area = "";
        setCountdown(duration);
        setIsCounting(true);
        setCapturing(true);

        if(view === "custom") {
            selector.hide();
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
                audio: audio,
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

        handleCountdown()

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

    useEffect( () => {
        invoke("cuhd")
            .then( (result) => {
                if(result.response)
                    setFilePath(result.response);
                else
                    console.log("ERROR"); /* TODO: handle path retrieve error */
            })
            .catch( (err) => console.log("ERROR") /* TODO: handle error */);
    }, []);

    function reserve(event) {
        event.preventDefault();

        if (isEmpty(event.target.value)) {
            setDuration(0);
            return;
        }

        setDuration(parseInt(event.target.value));
        return;

    }


        return (
            <>
        <Container className="background-container"></Container>

          <Container className={"flex-row align-self-center p-4"}>

          <Container className={"col-5"}></Container>
      <Container style={{zIndex: "2", position: "relative"}} className={"w-100 col-7 p-0 mt-0 justify-content-center"}>
          <Container className={"flex-row p-0 align-items-center mb-2"}>
              <FormText className={"m-2"}>Save to</FormText>
              <Form>
                  <div style={{ position: "relative" }}>
                      <Form.Control
                          type="text"
                          value={clipboard ? "Clipboard" : filePath}
                          disabled={clipboard || capturing}
                          style={{minWidth: "35rem"}}
                          onChange={ (event) => setFilePath(event.target.value)}
                      />
                  </div>
              </Form>
              <Button variant={"light"} disabled={clipboard} className="mx-2" onClick={openFolderDialog}>
                  <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                       className="bi bi-folder-plus" viewBox="0 0 16 16">
                      <path
                          d="m.5 3 .04.87a1.99 1.99 0 0 0-.342 1.311l.637 7A2 2 0 0 0 2.826 14H9v-1H2.826a1 1 0 0 1-.995-.91l-.637-7A1 1 0 0 1 2.19 4h11.62a1 1 0 0 1 .996 1.09L14.54 8h1.005l.256-2.819A2 2 0 0 0 13.81 3H9.828a2 2 0 0 1-1.414-.586l-.828-.828A2 2 0 0 0 6.172 1H2.5a2 2 0 0 0-2 2Zm5.672-1a1 1 0 0 1 .707.293L7.586 3H2.19c-.24 0-.47.042-.683.12L1.5 2.98a1 1 0 0 1 1-.98h3.672Z"/>
                      <path
                          d="M13.5 9a.5.5 0 0 1 .5.5V11h1.5a.5.5 0 1 1 0 1H14v1.5a.5.5 0 1 1-1 0V12h-1.5a.5.5 0 0 1 0-1H13V9.5a.5.5 0 0 1 .5-.5Z"/>
                  </svg>
              </Button>

              <Dropdown drop={"down-centered"} className={"ms-3"}>
                  <Dropdown.Toggle disabled={!mode || capturing} variant="light" id="dropdown-basic">
                      <FormText>{clipboard ? "Copy" : "Save"} as</FormText>
                  </Dropdown.Toggle>

                  <Dropdown.Menu style={{maxWidth: "3rem"}}>
                      <Dropdown.Item onClick={() => setFileType("pdf")} className={mode==="record" ? "d-none" : false}>
                          <FormText >pdf</FormText>
                          {fileType === "pdf" ?
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                       className="bi bi-check" viewBox="0 0 16 16">
                                      <path
                                          d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                  </svg> : false}
                      </Dropdown.Item>
                      <Dropdown.Item onClick={() => setFileType("jpeg")} className={mode==="record" ? "d-none" : false}>
                          <FormText >jpeg</FormText>
                          {fileType === "jpeg" ?
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                       className="bi bi-check" viewBox="0 0 16 16">
                                      <path
                                          d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                  </svg> : false}
                      </Dropdown.Item>
                      <Dropdown.Item onClick={() => setFileType("png")} className={mode==="record" ? "d-none" : false}>
                          <FormText >png</FormText>
                              {fileType === "png" ?
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                       className="bi bi-check" viewBox="0 0 16 16">
                                      <path
                                          d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                  </svg> : false}
                      </Dropdown.Item>
                      <Dropdown.Item onClick={() => setFileType("tiff")} className={mode==="record" ? "d-none" : false}>
                          <FormText>tiff</FormText>
                              {fileType === "tiff" ?
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                       className="bi bi-check" viewBox="0 0 16 16">
                                      <path
                                          d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                  </svg> : false}
                      </Dropdown.Item>
                      <Dropdown.Item onClick={() => setFileType("mov")} className={mode==="capture" ? "d-none" : false}>
                          <FormText className={fileType !== "mov" ? "ms-3" : false }>mov</FormText>
                          {fileType === "mov" ?
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                       className="bi bi-check" viewBox="0 0 16 16">
                                      <path
                                          d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                  </svg> : false}
                      </Dropdown.Item>
                      <Dropdown.Item onClick={() => setFileType("mp4")} className={mode==="capture" ? "d-none" : false}>
                          <FormText>mp4</FormText>
                          {fileType === "mp4" ?
                                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                       className="bi bi-check" viewBox="0 0 16 16">
                                      <path
                                          d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                  </svg> : false}
                      </Dropdown.Item>
                      <Dropdown.Item onClick={() => setFileType("avi")} className={mode==="capture" ? "d-none" : false}>
                          <FormText>avi</FormText>
                          {fileType === "avi" ?
                              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                   className="bi bi-check" viewBox="0 0 16 16">
                                  <path
                                      d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                              </svg> : false}
                      </Dropdown.Item>
                      <Dropdown.Item onClick={() => setFileType("gif")} className={mode==="capture" ? "d-none" : false}>
                          <FormText>gif</FormText>
                          {fileType === "gif" ?
                              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                   className="bi bi-check" viewBox="0 0 16 16">
                                  <path
                                      d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                              </svg> : false}
                      </Dropdown.Item>
                  </Dropdown.Menu>
              </Dropdown>
          </Container>

        <Container className={"flex-row align-items-center mb-1"}>

        <Container className={"d-flex flex-column align-items-center justify-content-center p-0"}>
          <FormText>Capture</FormText>
          <Container className={"d-flex flex-row p-0"}>
          <Button className={"m-1"} variant={mode === "capture" && view === "fullscreen" ? "primary" : "outline-primary"}
                  disabled={capturing}
                  title={"Capture Entire Screen"}
                  onClick={() => {
                      if(mode!=="capture") setFileType("png");

                      setMode("capture");
                      setView("fullscreen");
                      WebviewWindow.getByLabel('selector').hide()
                  } }>
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
                  onClick={() => {

                      if(mode!=="capture") setFileType("png");

                      setMode("capture");
                      setView("custom");
                      WebviewWindow.getByLabel('selector').show()

                      }
                }>
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
               className="bi bi-fullscreen" viewBox="0 0 16 16">
            <path
                d="M1.5 1a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0v-4A1.5 1.5 0 0 1 1.5 0h4a.5.5 0 0 1 0 1h-4zM10 .5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 16 1.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5zM.5 10a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 0 14.5v-4a.5.5 0 0 1 .5-.5zm15 0a.5.5 0 0 1 .5.5v4a1.5 1.5 0 0 1-1.5 1.5h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5z"/>
          </svg></Button>
          </Container>
          </Container>


          <Container className={"d-flex flex-column align-items-center justify-content-center p-0"}>
            <FormText className={"title-record"}>Record</FormText>
            <Container className={"d-flex flex-row p-0"}>
          <Button title={"Record Entire Screen"} className={"m-1"}
                  disabled={capturing}
                  variant={mode === "record" && view === "fullscreen" ? "danger" : "outline-danger"}
                  onClick={() => {
                      
                    if(mode !== "record") setFileType("mov");

                      setMode("record");
                      setView("fullscreen");
                      WebviewWindow.getByLabel('selector').hide()

                      setClipboard(false);
                  } }>
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
              onClick={() => {

                  if(mode!=="record") setFileType("mov");

                  setMode("record");
                  setView("custom");
                  WebviewWindow.getByLabel('selector').show()

                  setClipboard(false);

                 }
          }
          >
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

            <Container className={"d-flex flex-column align-items-center justify-content-center p-0"}>
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

            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 mx-1"}>
                <FormText>&nbsp;</FormText>
                <Dropdown>
                    <Dropdown.Toggle disabled={capturing} variant="light" id="dropdown-basic">
                        <FormText>Options</FormText>
                    </Dropdown.Toggle>

                    <Dropdown.Menu>
                        <Dropdown.Item onClick={() => setPointer((pointer) => !pointer)}>
                            <Container className={"flex-row justify-content-center align-items-center p-0 m-0"}>
                                {pointer ?
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                         className="bi bi-check" viewBox="0 0 16 16">
                                        <path
                                            d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                                    </svg> : false}
                            <FormText className={!pointer ? "ms-3" : false }>{mode === "capture" ? "Capture" : "Record"} Mouse Pointer</FormText>
                            </Container>
                        </Dropdown.Item>
                        <Dropdown.Item onClick={() => setClipboard((clipboard) => !clipboard)} className={mode==="record" ? "d-none" : false}>
                            {clipboard ?
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                 className="bi bi-check" viewBox="0 0 16 16">
                                <path
                                    d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                            </svg> : false}
                            <FormText className={!clipboard ? "ms-3" : false }>Copy to Clipboard</FormText>
                        </Dropdown.Item>
                        <Dropdown.Item onClick={() => setAudio((audio) => !audio)} className={mode==="capture" ? "d-none" : false}>{audio ?
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                 className="bi bi-check" viewBox="0 0 16 16">
                                <path
                                    d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                            </svg> : false}
                            <FormText className={!audio ? "ms-3" : false}>Record External Audio</FormText>
                        </Dropdown.Item>
                        <Dropdown.Item onClick={() => setOpenFile((openFile) => !openFile)}>
                            {openFile ?
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                 className="bi bi-check" viewBox="0 0 16 16">
                                <path
                                    d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425a.267.267 0 0 1 .02-.022z"/>
                            </svg> : false}
                            <FormText className={!openFile ? "ms-3" : false }>{mode === "capture" ? "Edit After Capture" : "Open After Record"}</FormText>
                        </Dropdown.Item>
                    </Dropdown.Menu>
                </Dropdown>
            </Container>

            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 mx-1"}>
                <FormText>&nbsp;</FormText>
                { countdown > 0 ? <Button className={"m-1"} variant={"danger"} onClick={stopCapture}>Cancel</Button> :
                    mode==="record" && capturing ? <Button className={"m-1"} variant={"danger"} onClick={stopCapture}>Stop</Button> :
                        (mode !== undefined && view !== undefined) ? <Button className={"m-1"} variant={mode === "capture" ? "primary" : "danger"} onClick={capture}>{mode[0].toUpperCase() + mode.slice(1)}</Button> : false}
            </Container>

            </Container>
          <Container className={"align-self-baseline justify-content-center p-0 m-1"}>
            <FormText>{text}</FormText>
          </Container>
      </Container>



          </Container>

      </>
  );
}

export default App;
