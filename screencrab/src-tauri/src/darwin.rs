use tokio::process::Command as tokioCommand;
use std::process::Command as stdCommand;
use tauri::{Window, Manager};
use std::process::Stdio;
use crate::utils::*;

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    let filename1 = filename.to_string();
    let index = get_current_monitor_index(&window);

    let mut command = tokioCommand::new("screencapture");

    if pointer { command.arg("-C"); }
    if clipboard { command.arg("-c"); }

    command.args(&["-t", file_type]);
    command.args(&["-T", timer.to_string().as_str()]);
    command.args(&["-D", index.to_string().as_str()]);

    let process = command.arg(filename1.as_str()).spawn().map_err(|e| Response::new(None, Some(format!("Failed to take screenshot: {}", e))));
    let pid = process.as_ref().unwrap().id().unwrap();

    window.listen_global("kill", move |_event| {
            let _output = stdCommand::new("kill")
                .arg("-15")
                .arg(pid.to_string())
                .output();
    });

    let output = process.unwrap().wait().await.unwrap();
    if output.success() {
        if !clipboard && open_file {
            window.windows().get("main_window").unwrap().minimize().unwrap();
            window.windows().get("tools").unwrap().show().unwrap();
            window.emit_all("path", filename.to_string()).unwrap();
        }
        if clipboard {
            return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    let filename1 = filename.to_string();
    let index = get_current_monitor_index(&window);

    let mut command = tokioCommand::new("screencapture");

    if pointer { command.arg("-C"); }
    if clipboard { command.arg("-c"); }

    command.args(&["-R", area]);
    command.args(&["-t", file_type]);
    command.args(&["-T", timer.to_string().as_str()]);
    command.args(&["-D", index.to_string().as_str()]);

    let process = command.arg(filename1.as_str()).spawn().map_err(|e| Response::new(None,Some(format!("Failed to take screenshot: {}", e)) ));
    let pid = process.as_ref().unwrap().id().unwrap();

    window.listen_global("kill", move |_event| {
            let _output = stdCommand::new("kill")
                .arg("-15")
                .arg(pid.to_string())
                .output();
    });

    let output = process.unwrap().wait().await.unwrap();
    if output.success() {
        if !clipboard && open_file {
            window.windows().get("main_window").unwrap().minimize().unwrap();
            window.windows().get("tools").unwrap().show().unwrap();
            window.emit_all("path", filename.to_string()).unwrap();
        }
        if clipboard {
            return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}

pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let filename1 = filename.to_string();
    let filename2 = filename.to_string();
    let index = get_current_monitor_index(&window);

    let mut command = tokioCommand::new("screencapture");
    command.stdin(Stdio::piped());
    command.arg("-v");

    if audio { command.arg("-g"); }

    command.args(&["-T", timer.to_string().as_str()]);
    command.args(&["-D", index.to_string().as_str()]);

    let mut process = command.arg(filename1.as_str()).spawn().map_err(|e| Response::new(None, Some(format!("Failed to launch screen record: {}", e)) )).unwrap();
    let _stdin = process.stdin.take().unwrap();  //do not release process stdin at wait(), capture it to send SIGTERM to recording process
    let pid = process.id().unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    window.listen_global("kill", move |_event| {
            let _output = stdCommand::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output();
    });

    let window_ = window.clone();
    window.listen_global("stop", move |_event| {

            let _output = stdCommand::new("kill")
                .arg("-2")  //SIGTERM
                .arg(pid.to_string())
                .output();

        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    if output.success() {
        if open_file {

            let _open = stdCommand::new("open").arg(filename2.as_str()).output().unwrap();

        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename1.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}

pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let filename1 = filename.to_string();
    let filename2 = filename.to_string();
    let index = get_current_monitor_index(&window);

    let mut command = tokioCommand::new("screencapture");
    command.stdin(Stdio::piped());
    command.arg("-v");

    if audio { command.arg("-g"); }

    command.args(&["-T", timer.to_string().as_str()]);
    command.args(&["-R", area]);
    command.args(&["-D", index.to_string().as_str()]);

    let mut process = command.arg(filename1.as_str()).spawn().map_err(|e| Response::new(None, Some(format!("Failed to launch screen record: {}", e)) )).unwrap();
    let mut _stdin = process.stdin.take().unwrap();  //do not release process stdin at wait(), capture it to send SIGTERM to recording process
    let pid = process.id().unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    window.listen_global("kill", move |_event| {

            let _output = stdCommand::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output();

    });

    let window_ = window.clone();
    window.listen_global("stop", move |_event| {

            let _output = stdCommand::new("kill")
                .arg("-2")  //SIGTERM
                .arg(pid.to_string())
                .output();

        window_.menu_handle().get_item("stop_reording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    if output.success() {
        if open_file {

            let _open = stdCommand::new("open").arg(filename2.as_str()).output().unwrap();

        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename1.to_string())), None );
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}
