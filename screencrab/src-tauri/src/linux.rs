use tokio::task;
use tokio::process::Command;
use tauri::{Window, Manager};
use crate::utils::Response;

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {

    let mut sleep_command = Command::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await;
        });
    });
    let output = sleep_command.wait_with_output().await.unwrap();
    if !output.status.success() {
        return Response::new(None, Some(format!("Screen Crab cancelled")));
    }

    let mut process = Command::new("ffmpeg")
        .arg("-y")
        .args(&["-f", "x11grab"])
        .args(&["-i", ":0.0+0,0"])
        .args(&["-draw_mouse", if pointer { "true" } else { "false" }])
        .args(&["-frames:v", "1"])
        .arg(&filename.to_string())
        .spawn()
        .map_err(|e| Response::new(None, Some(format!("Failed to take screenshot: {}", e))))
        .unwrap();

    let output = process.wait().await.unwrap();
    let filename1 = filename.to_string();
    if output.success() {
        if !clipboard && open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("xdg-open").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
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
    let mut sleep_command = Command::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await;
        });
    });
    let output = sleep_command.wait_with_output().await.unwrap();
    if !output.status.success() {
        return Response::new(None, Some(format!("Screen Crab cancelled")));
    }

    let mut process = Command::new("ffmpeg")
        .arg("-y")
        .args(&["-f", "x11grab"])
        .args(&["-i", ":0.0+0,0"])
        .args(&["-draw_mouse", if pointer { "true" } else { "false" }])
        .args(&["-frames:v", "1"])
        .arg(&filename.to_string())
        .spawn()
        .map_err(|e| Response::new(None, Some(format!("Failed to take screenshot: {}", e))))
        .unwrap();

    let output = process.wait().await.unwrap();
    let filename1 = filename.to_string();
    if output.success() {
        if !clipboard && open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("xdg-open").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
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
    let mut sleep_command = Command::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await;
        });
    });
    let output = sleep_command.wait_with_output().await.unwrap();
    if !output.status.success() {
        return Response::new(None, Some(format!("Screen Crab cancelled")));
    }

    let mut process = Command::new("ffmpeg")
        .arg("-y")
        .args(&["-f", "x11grab"])
        .args(&["-i", ":0.0+0,0"])
        .arg(&filename.to_string())
        .spawn()
        .map_err(|e| Response::new(None, Some(format!("Failed to take screenshot: {}", e))))
        .unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let pid = process.id().unwrap();

    let window_ = window.clone();
    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-2")  //SIGTERM
                .arg(pid.to_string())
                .output()
                .await;
        });
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    let filename1 = filename.to_string();
    if output.success() {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("xdg-open").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e)) ));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}

pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let mut sleep_command = Command::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await;
        });
    });
    let output = sleep_command.wait_with_output().await.unwrap();
    if !output.status.success() {
        return Response::new(None, Some(format!("Screen Crab cancelled")));
    }

    let mut process = Command::new("ffmpeg")
        .arg("-y")
        .args(&["-f", "x11grab"])
        .args(&["-i", ":0.0+0,0"])
        .arg(&filename.to_string())
        .spawn()
        .map_err(|e| Response::new(None, Some(format!("Failed to take screenshot: {}", e))))
        .unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let pid = process.id().unwrap();

    let window_ = window.clone();
    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-2")  //SIGTERM
                .arg(pid.to_string())
                .output()
                .await;
        });
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    let filename1 = filename.to_string();
    if output.success() {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("xdg-open").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e)) ));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}
