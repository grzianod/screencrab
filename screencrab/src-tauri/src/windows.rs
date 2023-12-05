use tokio::task;
use tauri::api::process::Command;
//use tauri::api::process::CommandEvent;
use tokio::process::Command as tokioCommand;
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::winbase::DETACHED_PROCESS;
//use std::os::windows::io::AsHandle;
use std::os::windows::process::CommandExt;
use std::process::Command as stdCommand;
use tauri::{Window, Manager};
//use std::process::{Command as StdCommand};
use crate::utils::*;
use winapi::um::wincon::GenerateConsoleCtrlEvent;
use winapi::um::wincon::CTRL_BREAK_EVENT;
//use winapi::um::wincon::{CTRL_C_EVENT, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT};
use std::fs;
use cpal::traits::{HostTrait,DeviceTrait};

pub async fn capture_fullscreen(window: Window, filename: &str, _file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;
    let position = get_monitor_position(&window, index);
    let monitor = window.current_monitor().unwrap().unwrap(); // Store the monitor in a variable
    let size = monitor.size();
    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    let status = Command::new_sidecar("ffmpeg")
        .unwrap()
        .args([
            "-f", "gdigrab",
            "-offset_x", format!("{}", position.x).as_str(),
            "-offset_y", format!("{}", position.y).as_str(),
            "-video_size", format!("{}x{}", size.width, size.height).as_str(),
            "-i", "desktop",
            "-draw_mouse", if pointer { "1" } else { "0" },
            "-frames:v", "1",
            &filename.to_string()
        ])
        .status()
        .unwrap();

    let filename1 = filename.to_string();
    if status.success() {
        if !clipboard && open_file {
            let _open_task = task::spawn(async move {
                let _open = tokioCommand::new("cmd").arg("/C").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
        }
        if clipboard {
            if let Err(_err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            } else {
                let _ = fs::remove_file(filename.to_string());
                return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
            }
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, _file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    let parts: Vec<&str> = area.split(',').collect();
    let x = parts[0].trim().parse::<i32>().unwrap();
    let y = parts[1].trim().parse::<i32>().unwrap();
    let width = parts[2].trim().parse::<i32>().unwrap();
    let height = parts[3].trim().parse::<i32>().unwrap();


    let status = Command::new_sidecar("ffmpeg")
        .unwrap()
        .args([
            "-f", "gdigrab",
            "-framerate", "30",
            "-offset_x", format!("{}", x).as_str(),
            "-offset_y", format!("{}", y).as_str(),
            "-video_size", format!("{}x{}", width, height).as_str(),
            "-i", "desktop",
            "-draw_mouse", if pointer { "1" } else { "0" },
            "-frames:v", "1",
            &filename.to_string()
        ])
        .status()
        .unwrap();


    let filename1 = filename.to_string();
    if status.success() {
        if !clipboard && open_file {
            let _open_task = task::spawn(async move {
                let _open = tokioCommand::new("cmd").arg("/C").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
        }
        if clipboard {
            if let Err(_err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            } else {
                let _ = fs::remove_file(filename.to_string());
                return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
            }
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}

pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;
    let position = get_monitor_position(&window, index);
    let monitor = window.current_monitor().unwrap().unwrap(); // Store the monitor in a variable
    let size = monitor.size();

    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    let mut selected_audio_device = String::new();

    if audio {
        // Run FFmpeg command to list devices
        let output = std::process::Command::new("ffmpeg")
            .args(["-list_devices", "true", "-f", "dshow", "-i", "dummy"])
            .output();

        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);

                // Extract audio device name, for example
                if let Some(start) = output_str.find("DirectShow audio devices") {
                    if let Some(end) = output_str[start..].find("Alternative name") {
                        let audio_device_line = &output_str[start..start+end];
                        // Extract the actual device name from audio_device_line
                        // For example, the line might contain the name "Microfono (3- Samson Meteor Mic)"
                        selected_audio_device = audio_device_line.to_string();
                    }
                }
            },
            Err(e) => {
                // Handle the error if FFmpeg command fails to execute
                return Response::new(None, Some(format!("Failed to execute FFmpeg command: {}", e)));
            }
        }
        
    }


    // Modify the FFmpeg command to include the selected audio device
    let _audio_device_arg = if !selected_audio_device.is_empty() {
        format!("-f dshow -i audio=\"{}\"", selected_audio_device)
    } else {
        String::new()
    };
    
    let filename_extension = std::path::Path::new(filename)
    .extension()
    .unwrap_or_default()
    .to_str()
    .unwrap_or_default()
    .to_lowercase();

    let (_video_codec, _format_option) = match filename_extension.as_str() {
        "mp4" => ("-c:v", "libx264"),
        "mov" => ("-c:v", "qtrle"),
        "avi" => ("-c:v", "mpeg4"),
        // Add more formats as needed
        _ => return Response::new(None, Some(format!("Unsupported file format: {}", filename_extension))),
    };

    let mut command = stdCommand::from(Command::new_sidecar("ffmpeg").unwrap()
        .args(["-f", "gdigrab"])
        .args(["-framerate", "30"])
        .args(["-offset_x", format!("{}", position.x).as_str()])
        .args(["-offset_y", format!("{}", position.y).as_str()])
        .args(["-video_size", format!("{}x{}", size.width, size.height).as_str()])
        .args(["-i", "desktop"])
    );
    if audio {
        let mic_name = cpal::default_host().default_input_device().unwrap().name().unwrap();
        let audio_string = format!("audio=\"{}\"", mic_name);
        command.raw_arg("-f");
        command.raw_arg("dshow");
        command.raw_arg("-i");
        command.raw_arg(audio_string.as_str());
    }

    command.args([&filename.to_string()]);


    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();
    
    let process = command.creation_flags(0x00000200).spawn().unwrap();
    let pid = process.id();
    let window_ = window.clone();
    let filename1 = filename.to_string();
    window.listen_global("stop", move |_event| {
        unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });


    let status = process.wait_with_output().unwrap().status;
    if status.code().unwrap() == 255 {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = stdCommand::new("cmd").arg("/C").arg(filename1.as_str()).output().map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}


pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    let parts: Vec<&str> = area.split(',').collect();
    let x = parts[0].trim().parse::<i32>().unwrap();
    let y = parts[1].trim().parse::<i32>().unwrap();
    let width = parts[2].trim().parse::<i32>().unwrap();
    let height = parts[3].trim().parse::<i32>().unwrap();

    let mut command = stdCommand::from(Command::new_sidecar("ffmpeg")
        .unwrap()
        .args(["-f", "gdigrab"])
        .args(["-framerate", "30"])
        .args(["-offset_x", format!("{}", x).as_str()])
        .args(["-offset_y", format!("{}", y).as_str()])
        .args(["-video_size", format!("{}x{}", width, height).as_str()])
        .args(["-i", "desktop"]));
        //.args(["-show_region", "1"]));

        if audio {
            let mic_name = cpal::default_host().default_input_device().unwrap().name().unwrap();
            let audio_string = format!("audio=\"{}\"", mic_name);
            //command.raw_arg(["-f", "dshow", "-i", audio_string.as_str()]);
            command.raw_arg("-f");
            command.raw_arg("dshow");
            command.raw_arg("-i");
            command.raw_arg(audio_string.as_str());
        }
    command.args(["-show_region", "1",&filename.to_string()]);

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let process = command.creation_flags(0x00000200).spawn().unwrap();
    let pid = process.id();
    let window_ = window.clone();
    let filename1 = filename.to_string();
    window.listen_global("stop", move |_event| {
        unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let status = process.wait_with_output().unwrap().status;
    if status.code().unwrap() == 255 {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = tokioCommand::new("cmd").arg("/C").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}
