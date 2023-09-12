use tokio::task;
use tokio::process::Command;
use tauri::{Window, Manager};
use crate::utils::*;

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window);

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
        .args(&["-i", format!(":{}.0+0,0", index).as_str()])
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
    let index = get_current_monitor_index(&window);
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

    let parts: Vec<&str> = area.split(',').collect();
        let x = parts[0].trim().parse::<i32>().unwrap();
        let y = parts[1].trim().parse::<i32>().unwrap();
        let width = parts[2].trim().parse::<i32>().unwrap();
        let height = parts[3].trim().parse::<i32>().unwrap();

    println!("{}, {}", width, height);


    let mut process = Command::new("ffmpeg")
        .arg("-y")
        .args(&["-f", "x11grab"])
        .args(&["-s", format!("{}x{}", width, height).as_str()])
        .args(&["-i", format!(":{}.0+{},{}", index, x.to_string(), y.to_string()).as_str()])
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
    let index = get_current_monitor_index(&window);
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
        .args(&["-i", format!(":{}.0+0,0", index).as_str()])
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
    let index = get_current_monitor_index(&window);
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

    let parts: Vec<&str> = area.split(',').collect();
    let x = parts[0].trim().parse::<i32>().unwrap();
    let y = parts[1].trim().parse::<i32>().unwrap();
    let width = parts[2].trim().parse::<i32>().unwrap();
    let height = parts[3].trim().parse::<i32>().unwrap();


    let mut process = Command::new("ffmpeg")
        .arg("-y")
        .args(&["-f", "x11grab"])
        .args(&["-s", format!("{}x{}", width, height).as_str()])
        .args(&["-i", format!(":{}.0+{},{}", index, x.to_string(), y.to_string()).as_str()])
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
