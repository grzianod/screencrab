import React, { useEffect, useState, useRef } from 'react';
import { Image } from 'react-bootstrap';
import { readBinaryFile } from '@tauri-apps/api/fs';
import { Container } from 'react-bootstrap';
import { listen } from '@tauri-apps/api/event';
import './styles.css';

function Annotations({ imagePath }) {
    const [imageLoaded, setImageLoaded] = useState(false);
    const canvasRef = useRef(null);

    useEffect(() => {
        // Load the image when the component mounts
        loadImage(imagePath);
    }, [imagePath]);

    const loadImage = async (path) => {
        try {
            const imageFile = await readBinaryFile(path);

            const imageSrc = `data:image/jpeg;base64,${Buffer.from(imageFile).toString('base64')}`;

            setImageLoaded(true);

            const canvas = canvasRef.current;
            const ctx = canvas.getContext('2d');

            const image = new Image();
            image.src = imageSrc;

            image.onload = () => {
                // Draw the image on the canvas
                ctx.drawImage(image, 0, 0, canvas.width, canvas.height);

                // You can add more drawing operations here, e.g., annotations
            };
        } catch (error) {
            console.error('Error loading image:', error);
        }
    };

    return (
        <Container>
            <canvas ref={canvasRef} width={800} height={600}></canvas>
            {imageLoaded ? null : <p>Loading image...</p>}
        </Container>
    );
}

export default Annotations;
