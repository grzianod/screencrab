// File: src/lib.rs

use std::process::Command;
use std::io::{Result, Error, ErrorKind};

pub fn capture_screen(filename: &str) -> Result<()> {
    let output = Command::new("screencapture")
        .args(&["-i", filename])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, format!("Failed to take a screenshot: {:?}", output)))
    }
}

