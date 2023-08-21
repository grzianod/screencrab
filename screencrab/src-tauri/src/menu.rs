use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Hotkeys {
    pub capture_fullscreen: String,
    pub capture_custom: String,
    pub capture_mouse_pointer: String,
    pub copy_to_clipboard: String,
    pub open_capture: String,
    pub record_fullscreen: String,
    pub record_custom: String,
    pub stop_record: String,
    pub record_external_audio: String,
    pub open_record : String
}

pub fn create_context_menu(hotkeys: &Hotkeys) -> Menu {

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
                              Menu::new().add_item(CustomMenuItem::new("capture_fullscreen".to_string(), "Fullscreen Capture").accelerator(&hotkeys.capture_fullscreen))
                                  .add_item(CustomMenuItem::new("capture_custom".to_string(), "Custom Capture").accelerator(&hotkeys.capture_custom))
                                  .add_native_item(MenuItem::Separator)
                                  .add_item(CustomMenuItem::new("capture_mouse_pointer".to_string(), "Capture Mouse Pointer").accelerator(&hotkeys.capture_mouse_pointer))
                                  .add_item(CustomMenuItem::new("copy_to_clipboard".to_string(), "Copy To Clipboard").accelerator(&hotkeys.copy_to_clipboard))
                                  .add_item(CustomMenuItem::new("open_capture".to_string(), "Edit After Capture").accelerator(&hotkeys.open_capture)));
    let record = Submenu::new("Record",
                              Menu::new().add_item(CustomMenuItem::new("record_fullscreen".to_string(), "Fullscreen Record").accelerator(&hotkeys.record_fullscreen))
                                  .add_item(CustomMenuItem::new("record_custom".to_string(), "Custom Record").accelerator(&hotkeys.record_custom))
                                  .add_item(CustomMenuItem::new("stop_record".to_string(), "Stop Recording").accelerator(&hotkeys.stop_record))
                                  .add_native_item(MenuItem::Separator)
                                  .add_item(CustomMenuItem::new("record_external_audio".to_string(), "Record External Audio").accelerator(&hotkeys.record_external_audio))
                                  .add_item(CustomMenuItem::new("open_record".to_string(), "Open After Record").accelerator(&hotkeys.open_record)));
    let settings = Submenu::new("Settings",
    Menu::new().add_item(CustomMenuItem::new("change_hotkeys".to_string(), "Change Shortcut Keys")));
    let help = Submenu::new("Help",
                            Menu::new().add_item(CustomMenuItem::new("learn_more", "Learn More")));

    Menu::new()
    .add_submenu(about)
    .add_submenu(capture)
    .add_submenu(record)
    .add_submenu(settings)
    .add_submenu(help)


}

