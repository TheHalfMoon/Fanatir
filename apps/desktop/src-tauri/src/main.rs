//! Fanatir T031 minimal Tauri 2 shell.
//!
//! WebView is untrusted presentation only. No invoke handlers, IPC contracts,
//! filesystem/network/shell commands, auth, Supabase, or PHI surfaces.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running Fanatir desktop shell");
}
