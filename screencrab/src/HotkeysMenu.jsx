import {useState, useEffect} from "react";
import 'bootstrap/dist/css/bootstrap.min.css';
import {Container} from "react-bootstrap";
import "./App.css";

function HotkeyForm({ hotkeys, setHotkeys }) {
    const [inputs, setInputs] = useState(hotkeys);
  
    const handleChange = (event) => {
      const { name, value } = event.target;
      setInputs({ ...inputs, [name]: value });
    };
  
    const handleSubmit = (event) => {
      event.preventDefault();
      setHotkeys(inputs);
      // Send command to Tauri backend to log the message
      window.__TAURI__.invoke({
        cmd: 'logMessage',
        message: 'Button has been pressed!'
      });
      // Write to the JSON file
      window.__TAURI__.fs
        .writeFile({
          path: 'src-tauri/src/hotkeys.json',
          contents: JSON.stringify(inputs, null, 2)
        })
        .catch((error) => {
          console.error("Failed to write to hotkeys.json:", error);
        });

      // Restart the Tauri app
      window.__TAURI__.app.relaunch();
      window.__TAURI__.app.exit(0);
    };
  
    return (
      <form onSubmit={handleSubmit}>
        {Object.keys(hotkeys).map((command) => (
          <div key={command}>
            <label>{command}</label>
            <input
              type="text"
              name={command}
              value={inputs[command]}
              onChange={handleChange}
            />
          </div>
        ))}
        <button type="submit">Update Hotkeys and Restart</button>
      </form>
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
      });
  }, []);

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <Container className={"flex-row justify-content-center p-0 m-0 w-100"}>
      {error ? (
        <div className="alert alert-danger" role="alert">
          Error loading hotkeys: {error}
        </div>
      ) : (
        <HotkeyForm hotkeys={hotkeys} setHotkeys={setHotkeys} />
      )}
    </Container>
  );
}


export default App;