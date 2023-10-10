import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import Annotations from "./Annotations.jsx";

ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
        <Annotations imagePath={"/Users/grazianodinocca/Desktop/test.png"} />
    </React.StrictMode>
);
