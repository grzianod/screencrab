import React, {useState} from "react";
import "./styles.css";
import {Button, Container, Image, Form, FormText, Spinner} from "react-bootstrap";
import {invoke} from "@tauri-apps/api/tauri";

 function SplashScreen() {
     const [loading, setLoading] = useState(false);
     const [text, setText] = useState(undefined);

     async function checkRequirements() {
         setLoading(true);
         setText("Checking requirements...");
         await invoke("check_requirements", {})
             .then(() => {})
             .catch(() => {
                 setText("Something went wrong!");
             })
             .finally(() => setLoading(false));
     }

     return (
         <>
             <Container className={"d-flex align-items-center justify-content-center"}>
                 <Image src={"icon.png"} style={{width: "10rem", height: "10rem"}}></Image>

                 <strong><p style={{fontSize: "2rem", margin: "0rem"}}>Welcome to Screen Crab!</p></strong>
                 <strong><i>Capturing Moments with Ease</i></strong>
                 <Container className={"d-flex align-items-center justify-content-center"}>
                     <p>Welcome to Screen Crab, an intuitive and feature-rich application, from which you can
                         effortlessly capture, edit, and share screenshots and screen recordings, helping you communicate
                         and collaborate like never before.</p>
                 </Container>
                 <Container className={"d-flex align-items-center justify-content-center"}>
                     {loading ? <Spinner></Spinner> : <Button variant={"outline-dark"} onClick={checkRequirements}>Let's Start!</Button> }
                     {text ? <FormText>{text}</FormText> : false}
                 </Container>
             </Container>
         </>
     )
 }

export default SplashScreen;