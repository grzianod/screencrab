use tokio::task;
use tokio::process::Command;
use tauri::{Window, Manager};
use std::process::Stdio;
use crate::utils::*;

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    return Response::new(Some(format!("Screen Crab taken!")),None );
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    return Response::new(Some(format!("Screen Crab taken!")),None );
}

pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    return Response::new(Some(format!("Screen Crab taken!")),None );
}

pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    return Response::new(Some(format!("Screen Crab taken!")),None );
}
