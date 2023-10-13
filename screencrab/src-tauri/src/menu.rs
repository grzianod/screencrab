use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use crate::utils::*;

pub fn create_context_menu() -> Menu {

    let content = hotkeys();
    let hotkeys: Hotkeys = serde_json::from_str(&content).unwrap();

    let about = Submenu::new("Screen Crab",
                             Menu::new()
                                 .add_native_item(MenuItem::Services)
                                 .add_native_item(MenuItem::Separator)
                                 .add_native_item(MenuItem::Hide)
                                 .add_native_item(MenuItem::HideOthers)
                                 .add_native_item(MenuItem::ShowAll)
                                 .add_native_item(MenuItem::Separator)
                                 .add_native_item(MenuItem::Quit));
    let capture = Submenu::new("Capture",
                               Menu::new().add_item(CustomMenuItem::new("fullscreen_capture".to_string(), "Fullscreen Capture").accelerator(&hotkeys.fullscreen_capture))
                                   .add_item(CustomMenuItem::new("custom_capture".to_string(), "Custom Capture").accelerator(&hotkeys.custom_capture))
                                   .add_native_item(MenuItem::Separator)
                                   .add_item(CustomMenuItem::new("capture_mouse_pointer".to_string(), "Capture Mouse Pointer").accelerator(&hotkeys.capture_mouse_pointer))
                                   .add_item(CustomMenuItem::new("copy_to_clipboard".to_string(), "Copy To Clipboard").accelerator(&hotkeys.copy_to_clipboard))
                                   .add_item(CustomMenuItem::new("edit_after_capture".to_string(), "Edit After Capture").selected().accelerator(&hotkeys.edit_after_capture)));
    let record = Submenu::new("Record",
                              Menu::new().add_item(CustomMenuItem::new("fullscreen_record".to_string(), "Fullscreen Record").accelerator(&hotkeys.fullscreen_record))
                                  .add_item(CustomMenuItem::new("custom_record".to_string(), "Custom Record").accelerator(&hotkeys.custom_record))
                                  .add_item(CustomMenuItem::new("stop_recording".to_string(), "Stop Recording").disabled().accelerator(&hotkeys.stop_recording))
                                  .add_native_item(MenuItem::Separator)
                                  .add_item(CustomMenuItem::new("record_external_audio".to_string(), "Record External Audio").accelerator(&hotkeys.record_external_audio))
                                  .add_item(CustomMenuItem::new("open_after_record".to_string(), "Open After Record").selected().accelerator(&hotkeys.open_after_record)));
    let settings = Submenu::new("Settings",
                                Menu::new().add_item(CustomMenuItem::new("change_hotkeys".to_string(), "Shortcut Keys")));
    let help = Submenu::new("Help",
                            Menu::new().add_item(CustomMenuItem::new("learn_more", "Learn More")));

    #[cfg(not(target_os = "macos"))] {
        Menu::new()
            .add_submenu(capture)
            .add_submenu(record)
            .add_submenu(settings)
            .add_submenu(help)
    }

    #[cfg(target_os="macos")] {
        Menu::new()
            .add_submenu(about)
            .add_submenu(capture)
            .add_submenu(record)
            .add_submenu(settings)
            .add_submenu(help)
    }
}


