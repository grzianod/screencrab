import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import {Button, Container, Spinner, Row, FormText} from 'react-bootstrap';
import { listen } from "@tauri-apps/api/event";
import { SketchPicker } from 'react-color';

const Tools = () => {
    const [text, setText] = useState(undefined);
    const [imagePath, setImagePath] = useState(undefined);
    const [imageSrc, setImageSrc] = useState('');
    const [loading, setLoading] = useState(false);
    const [color, setColor] = useState('#ffffff'); // Initial color is white
    const [picker, setPicker] = useState(false);

    const handleColorChange = (newColor) => {
        setColor(newColor.hex);
    };

    const loadImage = async (path) => {
        try {
            setImagePath(path);
            const imageBytes = await invoke('get_image_bytes', {path: path});

            // Convert Uint8Array to base64
            const binary = imageBytes.reduce((acc, byte) => acc + String.fromCharCode(byte), '');
            const dataUrl = `data:image/png;base64,${btoa(binary)}`;

            setImageSrc(dataUrl);
            setLoading(false);
        } catch (error) {
            console.error('Error loading image:', error);
        }
    };

    const deleteFile = () => {
        invoke('delete_file', {path: imagePath})
            .then(() => {})
            .catch((err) => setText(err));
        console.log("CLIC");
    }

    useEffect(() => {
        setLoading(true);
    }, []);

    useEffect(() => {
        if (loading === true) loadImage(undefined);
    }, [loading]);

    useEffect(() => {
        const promise = listen("path", (event) => {
            loadImage(event.payload);
        });
        return () => promise.then(remove => remove());
    });

    return (
        <Container fluid className="d-xxl-flex flex-xxl-column m-0">
            <Container style={{ position: "fixed", top: "0", width: "100%" }} className="m-1 p-3">
                <Row className="justify-content-around">
                <Row className="justify-content-around">
                    <Button variant="outline-primary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-pen" viewBox="0 0 16 16">
                            <path
                                d="m13.498.795.149-.149a1.207 1.207 0 1 1 1.707 1.708l-.149.148a1.5 1.5 0 0 1-.059 2.059L4.854 14.854a.5.5 0 0 1-.233.131l-4 1a.5.5 0 0 1-.606-.606l1-4a.5.5 0 0 1 .131-.232l9.642-9.642a.5.5 0 0 0-.642.056L6.854 4.854a.5.5 0 1 1-.708-.708L9.44.854A1.5 1.5 0 0 1 11.5.796a1.5 1.5 0 0 1 1.998-.001zm-.644.766a.5.5 0 0 0-.707 0L1.95 11.756l-.764 3.057 3.057-.764L14.44 3.854a.5.5 0 0 0 0-.708l-1.585-1.585z"/>
                        </svg></Button>
                    <Button variant="outline-primary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-circle-square" viewBox="0 0 16 16">
                            <path d="M0 6a6 6 0 1 1 12 0A6 6 0 0 1 0 6z"/>
                            <path
                                d="M12.93 5h1.57a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5v-1.57a6.953 6.953 0 0 1-1-.22v1.79A1.5 1.5 0 0 0 5.5 16h9a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 4h-1.79c.097.324.17.658.22 1z"/>
                        </svg></Button>
                    <Button variant="outline-primary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-type" viewBox="0 0 16 16">
                            <path
                                d="m2.244 13.081.943-2.803H6.66l.944 2.803H8.86L5.54 3.75H4.322L1 13.081h1.244zm2.7-7.923L6.34 9.314H3.51l1.4-4.156h.034zm9.146 7.027h.035v.896h1.128V8.125c0-1.51-1.114-2.345-2.646-2.345-1.736 0-2.59.916-2.666 2.174h1.108c.068-.718.595-1.19 1.517-1.19.971 0 1.518.52 1.518 1.464v.731H12.19c-1.647.007-2.522.8-2.522 2.058 0 1.319.957 2.18 2.345 2.18 1.06 0 1.716-.43 2.078-1.011zm-1.763.035c-.752 0-1.456-.397-1.456-1.244 0-.65.424-1.115 1.408-1.115h1.805v.834c0 .896-.752 1.525-1.757 1.525z"/>
                        </svg></Button>
                </Row>
                    <Row className="justify-content-around">
                        <Button variant="outline-primary" className={"m-2"} onClick={() => setPicker((picker) => !picker)}>
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                 className="bi bi-eyedropper" viewBox="0 0 16 16">
                                <path
                                    d="M13.354.646a1.207 1.207 0 0 0-1.708 0L8.5 3.793l-.646-.647a.5.5 0 1 0-.708.708L8.293 5l-7.147 7.146A.5.5 0 0 0 1 12.5v1.793l-.854.853a.5.5 0 1 0 .708.707L1.707 15H3.5a.5.5 0 0 0 .354-.146L11 7.707l1.146 1.147a.5.5 0 0 0 .708-.708l-.647-.646 3.147-3.146a1.207 1.207 0 0 0 0-1.708l-2-2zM2 12.707l7-7L10.293 7l-7 7H2v-1.293z"/>
                            </svg>
                            </Button>
                        {picker ? <SketchPicker style={{position: "fixed"}} color={color} onChange={handleColorChange}>
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
                                 className="bi bi-palette" viewBox="0 0 16 16">
                                <path
                                    d="M8 5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zm4 3a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zM5.5 7a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zm.5 6a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z"/>
                                <path
                                    d="M16 8c0 3.15-1.866 2.585-3.567 2.07C11.42 9.763 10.465 9.473 10 10c-.603.683-.475 1.819-.351 2.92C9.826 14.495 9.996 16 8 16a8 8 0 1 1 8-8zm-8 7c.611 0 .654-.171.655-.176.078-.146.124-.464.07-1.119-.014-.168-.037-.37-.061-.591-.052-.464-.112-1.005-.118-1.462-.01-.707.083-1.61.704-2.314.369-.417.845-.578 1.272-.618.404-.038.812.026 1.16.104.343.077.702.186 1.025.284l.028.008c.346.105.658.199.953.266.653.148.904.083.991.024C14.717 9.38 15 9.161 15 8a7 7 0 1 0-7 7z"/>
                            </svg>
                        </SketchPicker> : false }
                    </Row>
                <Row className="justify-content-around">
                    <Button variant="outline-primary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-crop" viewBox="0 0 16 16">
                            <path
                                d="M3.5.5A.5.5 0 0 1 4 1v13h13a.5.5 0 0 1 0 1h-2v2a.5.5 0 0 1-1 0v-2H3.5a.5.5 0 0 1-.5-.5V4H1a.5.5 0 0 1 0-1h2V1a.5.5 0 0 1 .5-.5zm2.5 3a.5.5 0 0 1 .5-.5h8a.5.5 0 0 1 .5.5v8a.5.5 0 0 1-1 0V4H6.5a.5.5 0 0 1-.5-.5z"/>
                        </svg></Button>
                    <Button variant="outline-primary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fillRule="evenodd"
                                  d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2v1z"/>
                            <path
                                d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466z"/>
                        </svg>
                        </Button>
                    <Button variant="outline-primary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-arrow-counterclockwise" viewBox="0 0 16 16">
                            <path fillRule="evenodd"
                                  d="M8 3a5 5 0 1 1-4.546 2.914.5.5 0 0 0-.908-.417A6 6 0 1 0 8 2v1z"/>
                            <path
                                d="M8 4.466V.534a.25.25 0 0 0-.41-.192L5.23 2.308a.25.25 0 0 0 0 .384l2.36 1.966A.25.25 0 0 0 8 4.466z"/>
                        </svg>
                        </Button>
                </Row>
                <Row className="justify-content-around">
                    <Button variant="outline-secondary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-zoom-in" viewBox="0 0 16 16">
                            <path fillRule="evenodd"
                                  d="M6.5 12a5.5 5.5 0 1 0 0-11 5.5 5.5 0 0 0 0 11zM13 6.5a6.5 6.5 0 1 1-13 0 6.5 6.5 0 0 1 13 0z"/>
                            <path
                                d="M10.344 11.742c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1 6.538 6.538 0 0 1-1.398 1.4z"/>
                            <path fillRule="evenodd"
                                  d="M6.5 3a.5.5 0 0 1 .5.5V6h2.5a.5.5 0 0 1 0 1H7v2.5a.5.5 0 0 1-1 0V7H3.5a.5.5 0 0 1 0-1H6V3.5a.5.5 0 0 1 .5-.5z"/>
                        </svg></Button>
                    <Button variant="outline-secondary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-zoom-out" viewBox="0 0 16 16">
                            <path fillRule="evenodd"
                                  d="M6.5 12a5.5 5.5 0 1 0 0-11 5.5 5.5 0 0 0 0 11zM13 6.5a6.5 6.5 0 1 1-13 0 6.5 6.5 0 0 1 13 0z"/>
                            <path
                                d="M10.344 11.742c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1 6.538 6.538 0 0 1-1.398 1.4z"/>
                            <path fillRule="evenodd"
                                  d="M3 6.5a.5.5 0 0 1 .5-.5h6a.5.5 0 0 1 0 1h-6a.5.5 0 0 1-.5-.5z"/>
                        </svg></Button>
                    <Button variant="outline-secondary" className={"m-2"}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor"
                             className="bi bi-arrows-fullscreen" viewBox="0 0 16 16">
                            <path fillRule="evenodd"
                                  d="M5.828 10.172a.5.5 0 0 0-.707 0l-4.096 4.096V11.5a.5.5 0 0 0-1 0v3.975a.5.5 0 0 0 .5.5H4.5a.5.5 0 0 0 0-1H1.732l4.096-4.096a.5.5 0 0 0 0-.707zm4.344 0a.5.5 0 0 1 .707 0l4.096 4.096V11.5a.5.5 0 1 1 1 0v3.975a.5.5 0 0 1-.5.5H11.5a.5.5 0 0 1 0-1h2.768l-4.096-4.096a.5.5 0 0 1 0-.707zm0-4.344a.5.5 0 0 0 .707 0l4.096-4.096V4.5a.5.5 0 1 0 1 0V.525a.5.5 0 0 0-.5-.5H11.5a.5.5 0 0 0 0 1h2.768l-4.096 4.096a.5.5 0 0 0 0 .707zm-4.344 0a.5.5 0 0 1-.707 0L1.025 1.732V4.5a.5.5 0 0 1-1 0V.525a.5.5 0 0 1 .5-.5H4.5a.5.5 0 0 1 0 1H1.732l4.096 4.096a.5.5 0 0 1 0 .707z"/>
                        </svg></Button>
                </Row>
                </Row>
            </Container>
            <Container className="d-xxl-flex align-items-center justify-content-center p-0" style={{ position: "fixed", marginTop: "10vh", marginBottom: "10vh" }}>
                {loading ?
                    <Container style={{ maxWidth: '80vw', maxHeight: '80vh', width: 'auto', height: 'auto' }}>
                        <Spinner variant={"secondary"}></Spinner>
                    </Container> :
                    <img
                        src={imageSrc}
                        alt="ScreenCrab"
                        style={{ maxWidth: '80vw', maxHeight: '80vh', width: 'auto', height: 'auto' }}
                    />}
            </Container>
            <Container style={{ position: "fixed", bottom: "0", width: "100%" }} className="m-1 p-3">
                <Row className="justify-content-around">
                <Row className="justify-content-around">
                    <Row className="justify-content-around">
                    <Button variant="danger" className={"m-2"}
                            onClick={deleteFile}>Delete</Button>
                    </Row>
                    <Row className="justify-content-around">
                        <FormText>{text}</FormText>
                    </Row>
                </Row>
                <Row className="justify-content-around">
                    <Button variant="outline-danger" className={"m-2"}>Cancel</Button>
                    <Button variant="primary" className={"m-2"}>Save</Button>
                </Row>
                </Row>
            </Container>
        </Container>
    );
};

export default Tools;
