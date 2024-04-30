// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ops::Deref, path::PathBuf};

use tauri::api::dir::read_dir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn read_fs() -> Vec<String> {
    let path = PathBuf::from(r"/Users/elura001/");

    let mut root: Vec<String> = vec![String::from("xdd")];

    for entry in read_dir(path, false).unwrap() {
        let item = entry.path.deref();
        root.push(item.to_str().unwrap_or("NOT_FOUND").to_string());
    }
    root
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_fs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
