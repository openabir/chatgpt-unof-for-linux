// Prevents extra console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager};
use tauri::webview::WebviewWindowBuilder;
use tauri::WebviewUrl;

fn main() {
    tauri::Builder::default()
        .setup(|app| {

            // Create a second window that loads ChatGPT
            let _chatgpt_window = WebviewWindowBuilder::new(
                app,
                "chatgpt_hidden",
                WebviewUrl::External("https://chat.openai.com".parse().unwrap()),
            )
            .visible(true)   // set to false later
            .title("ChatGPT Hidden")
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}