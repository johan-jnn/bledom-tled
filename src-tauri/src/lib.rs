use tauri::Manager;
use tokio::sync::Mutex;

use crate::tled::device::manager::BleDeviceManager;

pub mod tled;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tled::device::device_init,
            tled::device::device_get,
            tled::device::device_toggle,
            tled::device::device_change_only,
            tled::device::device_change_all,
            tled::device::device_set_effect,
            tled::device::device_use_audio,
            tled::device::device_default_audio_configuration
        ])
        .setup(|app| {
            app.manage(Mutex::new(BleDeviceManager::empty()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
