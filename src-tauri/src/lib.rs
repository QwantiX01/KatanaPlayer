use crate::services::audio::AUDIO_SERVICE_INSTANCE;

mod services;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn play() {
    AUDIO_SERVICE_INSTANCE.play(String::from(
        "/run/media/yuriib/Skebob/Music/02 - Satan Hussein.flac",
    ));
    println!("play")
}

#[tauri::command]
fn pause() {
    AUDIO_SERVICE_INSTANCE.pause();
    println!("pause")
}

#[tauri::command]
fn set_volume(percentage: u8) {
    AUDIO_SERVICE_INSTANCE.set_volume(percentage);
    println!("pause")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![play, pause, set_volume, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
