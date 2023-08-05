use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn create_context_menu() -> Menu {

    let about = Submenu::new("Screen Crab",
                             Menu::new()
                                 .add_native_item(MenuItem::Services)
                                 .add_native_item(MenuItem::Separator)
                                 .add_native_item(MenuItem::Hide)
                                 .add_native_item(MenuItem::HideOthers)
                                 .add_native_item(MenuItem::ShowAll)
                                 .add_native_item(MenuItem::Separator)
                                 .add_native_item(MenuItem::Quit));
    let file = Submenu::new("File",
                            Menu::new().add_native_item(MenuItem::CloseWindow)
                                .add_native_item(MenuItem::Quit));
    let edit = Submenu::new("Edit",
                            Menu::new()
                                .add_native_item(MenuItem::Undo)
                                .add_native_item(MenuItem::Redo)
                                .add_native_item(MenuItem::Separator)
                                .add_native_item(MenuItem::Cut)
                                .add_native_item(MenuItem::Copy)
                                .add_native_item(MenuItem::Paste));
    let captur = Submenu::new("Capture",
                              Menu::new().add_item(CustomMenuItem::new("capture_fullscreen".to_string(), "Fullscreen Capture").accelerator("Control+F"))
                                  .add_item(CustomMenuItem::new("capture_custom".to_string(), "Custom Capture").accelerator("Control+C"))
                                  .add_native_item(MenuItem::Separator)
                                  .add_item(CustomMenuItem::new("capture_mouse_pointer", "Capture Mouse Pointer").accelerator("CmdOrCtrl+M"))
                                  .add_item(CustomMenuItem::new("capture_to_clipboard", "Copy To Clipboard").accelerator("CmdOrCtrl+P"))
                                  .add_item(CustomMenuItem::new("edit_after_capture", "Edit After Capture").selected().accelerator("CmdOrCtrl+E")));
    let record = Submenu::new("Record",
                              Menu::new().add_item(CustomMenuItem::new("record_fullscreen".to_string(), "Fullscreen Record").accelerator("Control+Option+F"))
                                  .add_item(CustomMenuItem::new("record_custom".to_string(), "Custom Record").accelerator("Control+Option+C"))
                                  .add_item(CustomMenuItem::new("stop_record".to_string(), "Stop Recording").accelerator("Control+Option+S").disabled())
                                  .add_native_item(MenuItem::Separator)
                                  .add_item(CustomMenuItem::new("record_mouse_pointer", "Record Mouse Pointer").accelerator("CmdOrCtrl+Option+M"))
                                  .add_item(CustomMenuItem::new("open_after_record", "Open After Record").selected().accelerator("CmdOrCtrl+O")));
    let help = Submenu::new("Help",
                            Menu::new().add_item(CustomMenuItem::new("learn_more", "Learn More")));
    Menu::new()
        .add_submenu(about)
        .add_submenu(file)
        .add_submenu(edit)
        .add_submenu(captur)
        .add_submenu(record)
        .add_submenu(help)

}

