// File: src/lib.rs

use std::process::Command;
use std::io::{Result, Error, ErrorKind};
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Response {
    response: Option<String>,
    error: Option<String>,
}

pub fn cwd() -> String {
    let response = match env::current_dir() {
        Ok(current_dir) => Response { response: current_dir.to_str().map(String::from), error: None },
        Err(err) => Response { response: None, error: Some(err.to_string()) },
    };
    let json_response = serde_json::to_string_pretty(&response).expect("Failed to serialize JSON");

    json_response
}


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

fn main() {}