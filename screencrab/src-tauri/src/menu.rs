use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, SystemTrayMenu, SystemTrayMenuItem};

pub fn create_context_menu() -> Menu {

    let about = Submenu::new("Screen Crab",
                             Menu::new()
                                 .add_native_item(MenuItem::Services)
                                 .add_native_item(MenuItem::Separator)
                                 .add_item(CustomMenuItem::new("toggle".to_string(), "Toggle Visibility").accelerator("CmdOrCtrl+H"))
                                 .add_native_item(MenuItem::HideOthers)
                                 .add_native_item(MenuItem::ShowAll)
                                 .add_native_item(MenuItem::Separator)
                                 .add_native_item(MenuItem::Quit));
    let capture = Submenu::new("Capture",
                              Menu::new().add_item(CustomMenuItem::new("capture_fullscreen".to_string(), "Fullscreen Capture").accelerator("CmdOrCtrl+F"))
                                  .add_item(CustomMenuItem::new("capture_custom".to_string(), "Custom Capture").accelerator("CmdOrCtrl+C"))
                                  .add_native_item(MenuItem::Separator)
                                  .add_item(CustomMenuItem::new("capture_mouse_pointer".to_string(), "Capture Mouse Pointer").accelerator("Option+M"))
                                  .add_item(CustomMenuItem::new("copy_to_clipboard".to_string(), "Copy To Clipboard").accelerator("Option+C"))
                                  .add_item(CustomMenuItem::new("open".to_string(), "Edit After Capture").accelerator("Option+E")));
    let record = Submenu::new("Record",
                              Menu::new().add_item(CustomMenuItem::new("record_fullscreen".to_string(), "Fullscreen Record").accelerator("CmdOrCtrl+Option+F"))
                                  .add_item(CustomMenuItem::new("record_custom".to_string(), "Custom Record").accelerator("CmdOrCtrl+Option+C"))
                                  .add_item(CustomMenuItem::new("stop_record".to_string(), "Stop Recording").accelerator("CmdOrCtrl+Option+S"))
                                  .add_native_item(MenuItem::Separator)
                                  .add_item(CustomMenuItem::new("record_external_audio".to_string(), "Record External Audio").accelerator("Option+A"))
                                  .add_item(CustomMenuItem::new("open".to_string(), "Open After Record").accelerator("Option+O")));
    let help = Submenu::new("Help",
                            Menu::new().add_item(CustomMenuItem::new("learn_more", "Learn More")));

    Menu::new()
        .add_submenu(about)
        .add_submenu(capture)
        .add_submenu(record)
        .add_submenu(help)

}

pub fn create_system_tray_menu() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide").accelerator("CmdOrCtrl+H"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q"))
}

