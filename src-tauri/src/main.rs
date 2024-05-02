// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::read_dir, path::PathBuf};

use rust_file_explorer::{check_dot, check_type};
use serde::Serialize;

#[derive(Serialize)]
struct FileData {
    name: String,
    full_path: String,
    file_type: String,
    is_dot_file: bool,
}

#[tauri::command]
fn read_fs() -> Result<Vec<FileData>, String> {
    let path_buf = PathBuf::from(r"/Users/elura001/");

    let mut all_files: Vec<FileData> = vec![];

    let dirs = match read_dir(path_buf) {
        Ok(v) => v,
        Err(e) => return Err(String::from(format!("Error reading root {}", e))),
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

        let check_file_type = match item.file_type() {
            Ok(v) => v,
            Err(e) => return Err(String::from(format!("Error readinf file type: {}", e))),
        };
        let file_type = check_type(check_file_type);

        let is_dot_file = check_dot(&full_path);

        let file = FileData {
            name,
            full_path,
            file_type,
            is_dot_file,
        };
        all_files.push(file);
    }
    Ok(all_files)
}

#[tauri::command]
fn open_dir(full_path: String) -> Result<Vec<FileData>, String> {
    let mut files = vec![];

    let dirs = match read_dir(full_path) {
        Ok(v) => v,
        Err(e) => return Err(String::from(format!("Error reading root {}", e))),
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

        let check_file_type = match item.file_type() {
            Ok(v) => v,
            Err(e) => return Err(String::from(format!("Error readinf file type: {}", e))),
        };
        let file_type = check_type(check_file_type);

        let is_dot_file = check_dot(&full_path);

        let file = FileData {
            name,
            full_path,
            file_type,
            is_dot_file,
        };
        files.push(file);
    }
    Ok(files)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_fs, open_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
