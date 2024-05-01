// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::read_dir, path::PathBuf};

use serde::Serialize;

#[derive(Serialize)]
struct FileData {
    file_path: String,
    file_type: String,
}

#[tauri::command]
fn read_fs() -> Result<Vec<FileData>, String> {
    let path_buf = PathBuf::from(r"/Users/elura001/");

    let mut root: Vec<FileData> = vec![];

    let dirs = match read_dir(path_buf) {
        Ok(v) => v,
        Err(e) => return Err(String::from(format!("Error reading root {}", e))),
    };

    for entry in dirs {
        let item = match entry {
            Ok(v) => v,
            Err(e) => return Err(String::from(format!("Error getting entry in dir: {}", e))),
        };
        let file_path = item.path().to_str().unwrap_or("ERROR").to_string();
        let file_type: String;
        match item.file_type() {
            Ok(v) => {
                if v.is_dir() {
                    file_type = String::from("dir")
                } else {
                    file_type = String::from("file")
                }
            }
            Err(e) => return Err(String::from(format!("Error readinf file type: {}", e))),
        }
        let file = FileData {
            file_path,
            file_type,
        };
        root.push(file);
    }
    Ok(root)
}

#[tauri::command]
fn open_dir() {}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_fs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
