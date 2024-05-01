// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::read_dir, path::PathBuf};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn read_fs() -> Vec<(String, String)> {
    let path_buf = PathBuf::from(r"/Users/elura001/");

    let mut root: Vec<(String, String)> = vec![];

    for entry in read_dir(path_buf).unwrap() {
        let item = entry.unwrap();
        let file_path = item.path().to_str().unwrap_or("ERROR").to_string();
        let file_type: String;
        if item.file_type().unwrap().is_dir() {
            file_type = String::from("dir")
        } else {
            file_type = String::from("file")
        }
        root.push((file_path, file_type));
    }
    root
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_fs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
