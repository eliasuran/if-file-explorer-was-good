// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::read_dir;

use rust_file_explorer::{check_dot, check_type};
use serde::Serialize;

#[derive(Serialize)]
struct OpenDirReturn {
    current_path: String,
    file_data: Vec<FileData>,
}

#[derive(Serialize)]
struct FileData {
    name: String,
    full_path: String,
    file_type: String,
    is_dot_file: bool,
}

#[tauri::command]
fn open_root() -> Result<OpenDirReturn, String> {
    match open_dir("/Users/elura001/".to_string()) {
        Ok(v) => Ok(v),
        Err(e) => {
            return Err(String::from(format!(
                "Error reading root file system: {}",
                e
            )))
        }
    }
}

#[tauri::command]
fn open_dir(full_path: String) -> Result<OpenDirReturn, String> {
    let mut files = vec![];

    let dirs = match read_dir(&full_path) {
        Ok(v) => v,
        Err(e) => return Err(String::from(format!("Error reading directory {}", e))),
    };

    for entry in dirs {
        let item = match entry {
            Ok(v) => v,
            Err(e) => return Err(String::from(format!("Error getting entry in dir: {}", e))),
        };
        let full_path = item.path().to_str().unwrap_or("ERROR").to_string();

        let name = full_path
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();

        let file_type = match item.file_type() {
            Ok(v) => check_type(v, &full_path)?,
            Err(e) => return Err(String::from(format!("Error readinf file type: {}", e))),
        };

        let is_dot_file = check_dot(&full_path);

        let file = FileData {
            name,
            full_path,
            file_type,
            is_dot_file,
        };
        files.push(file);
    }

    let return_value = OpenDirReturn {
        current_path: full_path,
        file_data: files,
    };
    Ok(return_value)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_root, open_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
