import {useState} from "react";
import 'bootstrap/dist/css/bootstrap.min.css';
import {Container} from "react-bootstrap";
import "./App.css";

function HotkeyForm({ hotkeys, setHotkeys }) {
    // Stato locale per tenere traccia degli input del form
    const [inputs, setInputs] = useState(hotkeys);
  
    const handleChange = (event) => {
      const { name, value } = event.target;
      setInputs({ ...inputs, [name]: value });
    };
  
    const handleSubmit = (event) => {
      event.preventDefault();
      // Aggiorna le hotkeys con i nuovi valori
      setHotkeys(inputs);
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
        <button type="submit">Update Hotkeys</button>
      </form>
    );
}

function App() {
  const [hotkeys, setHotkeys] = useState({
    command1: 'Ctrl+1',
    command2: 'Ctrl+2',
    // ... altri comandi
  });

  return (

    <Container className={"flex-row justify-content-center p-0 m-0 w-100"}>
    <HotkeyForm hotkeys={hotkeys} setHotkeys={setHotkeys} />
    </Container>
    );
}


export default App;