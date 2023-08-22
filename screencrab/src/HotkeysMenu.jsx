import React, {useState, useEffect, useRef} from "react";
import 'bootstrap/dist/css/bootstrap.min.css';
import {Button, Container, Form, FormText, Table} from "react-bootstrap";
import {invoke} from '@tauri-apps/api/tauri';
import "./App.css";

function saveData(data, setFeedback) {
    invoke('write_to_json', {input: {hotkeyData: data}})  // Nest data under 'input' and 'hotkeyData'
        .then(() => {
            setFeedback("Hotkeys written successfully");
        })
        .catch((err) => {
            setFeedback("Failed to write data: " + err);
        });
}

function KeyCaptureInput({value, onChange, name}) {
    const [currentKeys, setCurrentKeys] = useState([]);
    const inputRef = useRef(null);

    const handleKeyDown = (event) => {
        event.preventDefault();
        let capturedKey = event.code;
    
        if (capturedKey.startsWith("Key")) {
            capturedKey = capturedKey.slice(3);
        } else if (capturedKey.startsWith("Digit")) {
            capturedKey = capturedKey.slice(5);
        } else {
            switch (capturedKey) {
                case "MetaLeft":
                case "MetaRight":
                    capturedKey = "CmdOrCtrl";
                    break;
                case "AltLeft":
                case "AltRight":
                    capturedKey = "Option";
                    break;
                // Add other cases as needed
            }
        }
    
        if (!currentKeys.includes(capturedKey)) {
            setCurrentKeys((prevKeys) => [...prevKeys, capturedKey]);
        }
    };

    const handleKeyUp = (event) => {
        event.preventDefault();
        if (currentKeys.length) {
            onChange({target: {name, value: currentKeys.join('+').replace("Meta", "CmdOrCtrl")}});  // Replace "meta" with "command" in the joined string
        }
        setTimeout(() => setCurrentKeys([]), 500);
    };

    useEffect(() => {
        const inputElement = inputRef.current;
        inputElement.addEventListener('keydown', handleKeyDown);
        inputElement.addEventListener('keyup', handleKeyUp);
        return () => {
            inputElement.removeEventListener('keydown', handleKeyDown);
            inputElement.removeEventListener('keyup', handleKeyUp);
        };
    }, [currentKeys]);

    return (
        <input
            ref={inputRef}
            className="w-100"
            type="text"
            name={name}
            value={value}
            readOnly
        />
    );
}

function HotkeyForm({hotkeys, setHotkeys}) {
    const [inputs, setInputs] = useState(hotkeys);
    const [feedback, setFeedback] = useState(null);  // for feedback messages
    const [currentKeys, setCurrentKeys] = useState([]);
    const [selectedCommand, setSelectedCommand] = useState(null);

    const handleKeyDown = (event, command) => {
        event.preventDefault(); // Prevent default action of the keypress
        if (!currentKeys.includes(event.key)) {
            setCurrentKeys(prevKeys => [...prevKeys, event.key]);
        }
    };

    const handleKeyUp = (event, command) => {
        event.preventDefault(); // Prevent default action of the key release
        setInputs({...inputs, [command]: currentKeys.join('+')});
        setCurrentKeys([]); // Clear the current keys after setting
    };

    const handleChange = (event) => {
        const {name, value} = event.target;
        setInputs({...inputs, [name]: value});
    };

    const handleSubmit = (event) => {
        event.preventDefault();
        saveData(inputs, setFeedback);  // passing setFeedback as a callback
    };

    function formatLabel(str) {
        // Replace underscores with spaces and capitalize the first letter
        let replacedString = str.replace(/_/g, ' ');
        let words = replacedString.split(' ');

        for (let i = 0; i < words.length; i++) {
            words[i] = words[i][0].toUpperCase() + words[i].slice(1);
        }

        return words.join(' ');
    }

    function formatInput(str) {
        let command = str;
        command = command.replace("CmdOrCtrl", String.fromCharCode(0x2318));
        command = command.replace("Control", String.fromCharCode(0x2303));
        command = command.replace("Option", String.fromCharCode(0x2325));
        command = command.replace("Shift", String.fromCharCode(0x21E7));
        command = command.replace(/\+/g, "");
        command = command.toUpperCase();
        return command;
    }

    return (
        <>
        <Container className="background-container p-0 m-0"></Container>

    <Container className={"flex-row justify-content-center p-0 m-0 w-100"}>
        <div className={"col-4"}></div>
        <Container style={{zIndex: "2", position: "relative"}} className={"w-100 align-items-center"}>
            <strong style={{fontSize: "1.5rem"}} className={"m-2 mb-3"}>Shortcut Keys</strong>
            <Form onSubmit={handleSubmit}>
                <Table className="table table-bordered">
                    <tbody>
                    {Object.keys(hotkeys).map((command) => (
                        <tr key={command} >
                            <td style={{width: '50%', textAlign: "center", verticalAlign: "middle"}}>
                                <label>{formatLabel(command)}</label>
                            </td>
                            <td style={{width: '50%', textAlign: "center", verticalAlign: "middle"}}
                                className={command === selectedCommand ? 'selected-cell' : ''}
                                onClick={() => setSelectedCommand(command)}>
                                <KeyCaptureInput
                                    name={command}
                                    value={formatInput(inputs[command])}
                                    onChange={handleChange}
                                />
                            </td>
                        </tr>
                    ))}
                    </tbody>
                </Table>
                <Button variant={"primary"} type="submit">Update Hotkeys and Restart</Button>
            </Form>
            {feedback && <p>{feedback}</p>}
        </Container>
    </Container>
        </>
    );
}

function App() {
    const [hotkeys, setHotkeys] = useState({});
    const [error, setError] = useState(null);
    const [isLoading, setIsLoading] = useState(true);

    // Fetch JSON data when component mounts
    useEffect(() => {
        invoke("load_hotkeys", {})
                    .then(response => {
                        setHotkeys(JSON.parse(response));
                    })
                    .catch(err => {
                        setError(err.message);
                    })
            .finally(() => setIsLoading(false));
    }, []);

    if (isLoading) {
        return <div>Loading...</div>;
    }

    return (
        <Container className={"flex-row justify-content-center p-0 m-0 w-100"}>
            {error ? (
                <div className="alert alert-danger" role="alert">
                    <h2>Error</h2>
                    <p>Error loading hotkeys: {error}</p>
                </div>
            ) : (
                <HotkeyForm hotkeys={hotkeys} setHotkeys={setHotkeys}/>
            )}
        </Container>
    );
}


export default App;