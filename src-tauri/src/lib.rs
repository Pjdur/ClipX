use std::{thread, time::Duration};
use tauri::Emitter;
use arboard::Clipboard;

#[tauri::command]
fn set_clipboard(text: String) {
    if let Ok(mut clipboard) = Clipboard::new() {
        let _ = clipboard.set_text(text);
    }
}

fn start_clipboard_watcher(app: tauri::AppHandle) {
    thread::spawn(move || {
        let mut clipboard = Clipboard::new().unwrap();
        let mut last = String::new();

        loop {
            if let Ok(text) = clipboard.get_text() {
                if text != last && !text.trim().is_empty() {
                    last = text.clone();
                    let _ = app.emit("clipboard-update", text);
                }
            }
            thread::sleep(Duration::from_millis(300));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_clipboard])
        .setup(|app| {
            // Clone the handle for the background thread
            start_clipboard_watcher(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}