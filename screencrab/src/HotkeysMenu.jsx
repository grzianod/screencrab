import React, { useState, useEffect, useRef } from "react";
import 'bootstrap/dist/css/bootstrap.min.css';
import {Container} from "react-bootstrap";
import { invoke } from '@tauri-apps/api/tauri';
import "./App.css";

function saveData(data, setFeedback) {
  invoke('write_to_json', { input: { hotkeyData: data } })  // Nest data under 'input' and 'hotkeyData'
    .then(() => {
      setFeedback("Data written successfully");
    })
    .catch((err) => {
      setFeedback("Failed to write data: " + err);
    });
}

function KeyCaptureInput({ value, onChange, name }) {
  const [currentKeys, setCurrentKeys] = useState([]);
  const inputRef = useRef(null);

  const handleKeyDown = (event) => {
    event.preventDefault();
    let capturedKey = event.key;
    
    if (capturedKey === "Meta") {
      capturedKey = "Command";  // Replace "meta" with "command"
    }

    if (!currentKeys.includes(capturedKey)) {
      setCurrentKeys((prevKeys) => [...prevKeys, capturedKey]);
    }
  };

  const handleKeyUp = (event) => {
    event.preventDefault();
    if (currentKeys.length) {
      onChange({ target: { name, value: currentKeys.join('+').replace("Meta", "Command") } });  // Replace "meta" with "command" in the joined string
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

function HotkeyForm({ hotkeys, setHotkeys }) {
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
    setInputs({ ...inputs, [command]: currentKeys.join('+') });
    setCurrentKeys([]); // Clear the current keys after setting
  };

  const handleChange = (event) => {
    const { name, value } = event.target;
    setInputs({ ...inputs, [name]: value });
  };

  const handleSubmit = (event) => {
    event.preventDefault();   
    saveData(inputs, setFeedback);  // passing setFeedback as a callback
  };

  function formatLabel(str) {
    // Replace underscores with spaces and capitalize the first letter
    return str.replace(/_/g, ' ').replace(/^\w/, (c) => c.toUpperCase());
  }

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <table className="table table-bordered">
          <tbody>
            {Object.keys(hotkeys).map((command) => (
              <tr key={command}>
                <td style={{ width: '50%' }}>
                  <label>{formatLabel(command)}</label>
                </td>
                <td style={{ width: '50%' }} 
                  className={command === selectedCommand ? 'selected-cell' : ''}
                onClick={() => setSelectedCommand(command)}>
                  <KeyCaptureInput
                    name={command}
                    value={inputs[command]}
                    onChange={handleChange}
                  />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        <button type="submit">Update Hotkeys and Restart</button>
      </form>
      {feedback && <p>{feedback}</p>}
    </div>
  );
}

function App() {
  const [hotkeys, setHotkeys] = useState({});
  const [error, setError] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  // Fetch JSON data when component mounts
  useEffect(() => {
    fetch('src-tauri/src/hotkeys.json')
      .then(response => {
        if (!response.ok) {
          throw new Error("Failed to fetch hotkeys.");
        }
        return response.json();
      })
      .then(data => {
        setHotkeys(data);
        setIsLoading(false);
      })
      .catch(err => {
        console.error("Error loading hotkeys:", err);
        setError(err.message);
        setIsLoading(false);
        console.log("Error state set to:", err.message); // Add this line
      });
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
        <HotkeyForm hotkeys={hotkeys} setHotkeys={setHotkeys} />
      )}
    </Container>
  );
}


export default App;