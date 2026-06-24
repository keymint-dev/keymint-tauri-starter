#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod license;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            license::activate_license,
            license::deactivate_license,
            license::is_activated,
            license::get_license_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
