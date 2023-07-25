import {useEffect, useState, useRef} from "react";
import 'bootstrap/dist/css/bootstrap.min.css';
import { invoke } from "@tauri-apps/api/tauri";
import {Container, Button, FormText, Form, Dropdown} from "react-bootstrap";
import "./App.css";
import isEmpty from "validator/es/lib/isEmpty.js";

function App() {
  const [mode, setMode] = useState("capture");
  const [view, setView] = useState("fullscreen");
  const [duration, setDuration] = useState(0);
  const [pointer, setPointer] = useState(false);
  const [text, setText] = useState(undefined);
  const [name, setName] = useState("");
  const [path, setPath] = useState("");
  const [countdown, setCountdown] = useState(0);
  const [capturing, setCapturing] = useState(false);
  const [isCounting, setIsCounting] = useState(false);


    async function wait(countdown) {
        if (countdown <= 0) {
            await new Promise((resolve) => setTimeout(resolve, 5));
            setCountdown(countdown);
            return;
        }

        await new Promise((resolve) => setTimeout(resolve, 1000));
        setCountdown((countdown) => countdown - 1);
        await wait(countdown - 1)
    }

    async function capture() {
        setCountdown(duration);
        setIsCounting(true);
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
                setCapturing(true);
                await invoke("capture", {mode, view, pointer, path});
                setCapturing(false);
            }
        }

        if(duration > 0)
            handleCountdown();

    }, [countdown, isCounting]);

    async function stopCapture() {
        setCountdown(0);
        setIsCounting(false);
    }

    async function openFolderDialog() {
        const result = await invoke("folder_dialog");
        if(result.response)
            setPath(result.response);
        else
            console.log(result.error); /* TODO: handle error */
    }

    useEffect( () => {
        invoke("cwd")
            .then( (result) => {
                if(result.response)
                    setPath(result.response);
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

          <Container className={"flex-row align-self-center"}>

          <Container className={"col-4"}></Container>
      <Container style={{zIndex: "2", position: "relative"}} className={"w-75 mx-5 col-8 p-0"}>
          <Container className={"flex-row p-0 align-items-center mb-2"}>
              {text ? <h2>text</h2> : false}
              <FormText className={"m-2"}>Save to</FormText>
              <Form>
                  <div style={{ position: "relative" }}>
                      <Form.Control
                          type="text"
                          value={path}
                          readOnly
                          style={{minWidth: "40rem"}}
                          onChange={ (event) => setPath(event.target.value)}
                      />
                  </div>
              </Form>
                  <Button variant={"light"} className="mx-2" onClick={openFolderDialog}>
                      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                           className="bi bi-folder-plus" viewBox="0 0 16 16">
                          <path
                              d="m.5 3 .04.87a1.99 1.99 0 0 0-.342 1.311l.637 7A2 2 0 0 0 2.826 14H9v-1H2.826a1 1 0 0 1-.995-.91l-.637-7A1 1 0 0 1 2.19 4h11.62a1 1 0 0 1 .996 1.09L14.54 8h1.005l.256-2.819A2 2 0 0 0 13.81 3H9.828a2 2 0 0 1-1.414-.586l-.828-.828A2 2 0 0 0 6.172 1H2.5a2 2 0 0 0-2 2Zm5.672-1a1 1 0 0 1 .707.293L7.586 3H2.19c-.24 0-.47.042-.683.12L1.5 2.98a1 1 0 0 1 1-.98h3.672Z"/>
                          <path
                              d="M13.5 9a.5.5 0 0 1 .5.5V11h1.5a.5.5 0 1 1 0 1H14v1.5a.5.5 0 1 1-1 0V12h-1.5a.5.5 0 0 1 0-1H13V9.5a.5.5 0 0 1 .5-.5Z"/>
                      </svg>
                  </Button>
          </Container>
          <Container className={"flex-row p-0 align-items-center"}>
              <FormText className={"m-2"}>Name:</FormText>
              <Form>
                  <div style={{ position: "relative" }}>
                      <Form.Control
                          type="text"
                          value={name}
                          style={{minWidth: "15rem"}}
                          onChange={ (event) => setName(event.target.value)}
                      />
                  </div>
              </Form>
              <Dropdown className={"ms-2"}>
                  <Dropdown.Toggle variant="light" id="dropdown-basic">
                      Save as
                  </Dropdown.Toggle>

                  <Dropdown.Menu>
                      <Dropdown.Item className={mode==="record" ? "d-none" : false}>.jpeg</Dropdown.Item>
                      <Dropdown.Item className={mode==="record" ? "d-none" : false}>.png</Dropdown.Item>
                      <Dropdown.Item className={mode==="capture" ? "d-none" : false}>.mov</Dropdown.Item>
                      <Dropdown.Item className={mode==="capture" ? "d-none" : false}>.mp4</Dropdown.Item>
                      <Dropdown.Item className={mode==="capture" ? "d-none" : false}>.gif</Dropdown.Item>
                  </Dropdown.Menu>
              </Dropdown>
          </Container>

        <Container className={"flex-row align-items-center justify-content-center"}>

        <Container className={"d-flex flex-column align-items-center justify-content-center p-0"}>
          <FormText>Capture</FormText>
          <Container className={"d-flex flex-row p-0"}>
          <Button className={"m-1"} variant={mode === "capture" && view === "fullscreen" ? "primary" : "outline-primary"}
                  title={"Capture Entire Screen"}
                  onClick={() => {
                    setMode("capture");
                    setView("fullscreen"); } }>
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                 className="bi bi-window-desktop" viewBox="0 0 16 16">
              <path d="M3.5 11a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-9Z"/>
              <path
                  d="M2.375 1A2.366 2.366 0 0 0 0 3.357v9.286A2.366 2.366 0 0 0 2.375 15h11.25A2.366 2.366 0 0 0 16 12.643V3.357A2.366 2.366 0 0 0 13.625 1H2.375ZM1 3.357C1 2.612 1.611 2 2.375 2h11.25C14.389 2 15 2.612 15 3.357V4H1v-.643ZM1 5h14v7.643c0 .745-.611 1.357-1.375 1.357H2.375A1.366 1.366 0 0 1 1 12.643V5Z"/>
            </svg>
          </Button>

          <Button className={"m-1"} variant={mode === "capture" && view === "window" ? "primary" : "outline-primary"}
                  title={"Capture Selected Window"}
                  onClick={() => {
                    setMode("capture");
                    setView("window"); } }>
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" className="bi bi-window"
               viewBox="0 0 16 16">
            <path
                d="M2.5 4a.5.5 0 1 0 0-1 .5.5 0 0 0 0 1zm2-.5a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0zm1 .5a.5.5 0 1 0 0-1 .5.5 0 0 0 0 1z"/>
            <path
                d="M2 1a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V3a2 2 0 0 0-2-2H2zm13 2v2H1V3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1zM2 14a1 1 0 0 1-1-1V6h14v7a1 1 0 0 1-1 1H2z"/>
          </svg>
          </Button>

          <Button className={"m-1"} variant={mode === "capture" && view === "custom" ? "primary" : "outline-primary"}
                  title={"Capture Selected Portion"}
                  onClick={() => {
                    setMode("capture");
                    setView("custom"); } }>
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
               className="bi bi-fullscreen" viewBox="0 0 16 16">
            <path
                d="M1.5 1a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0v-4A1.5 1.5 0 0 1 1.5 0h4a.5.5 0 0 1 0 1h-4zM10 .5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 16 1.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5zM.5 10a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 0 14.5v-4a.5.5 0 0 1 .5-.5zm15 0a.5.5 0 0 1 .5.5v4a1.5 1.5 0 0 1-1.5 1.5h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5z"/>
          </svg></Button>
          </Container>
          </Container>


          <Container className={"d-flex flex-column align-items-center justify-content-center p-0 m-2"}>
            <FormText className={"title-record"}>Record</FormText>
            <Container className={"d-flex flex-row p-0"}>
          <Button title={"Record Entire Screen"} className={"m-1"} variant={mode === "record" && view === "fullscreen" ? "danger" : "outline-danger"}
                  onClick={() => {
                    setMode("record");
                    setView("fullscreen"); } }>
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
              onClick={() => {
                setMode("record");
                setView("custom"); } }
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

            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 m-2"}>
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

            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 m-2"}>
                <FormText>Show Mouse Pointer</FormText>
                <Form.Switch checked={pointer} onChange={() => setPointer( (pointer) => !pointer)}></Form.Switch>
            </Container>

            <Container className={"d-flex flex-column align-items-center justify-content-center p-0 m-2"}>
                <FormText>&nbsp;</FormText>
                {!capturing ?
                     countdown > 0 ? <Button className={"m-1"} variant={"danger"} onClick={stopCapture}>Cancel</Button> :
                    <Button className={"m-1"} variant={mode === "capture" ? "primary" : "danger"} onClick={capture}>{mode[0].toUpperCase() + mode.slice(1)}</Button> : false }
            </Container>

            </Container>
      </Container>

          </Container>

      </>
  );
}

export default App;
