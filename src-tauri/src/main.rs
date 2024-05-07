// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::read_dir, process::Command};

use rust_file_explorer::{check_dot, check_type, get_file_name, is_hidden, is_node_module};
use serde::Serialize;
use walkdir::WalkDir;

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
    let user = whoami::username();
    match open_dir(format!("/Users/{}", user)) {
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

        let name = get_file_name(&full_path);

        let file_type = match item.file_type() {
            Ok(v) => check_type(v, &full_path)?,
            Err(e) => return Err(String::from(format!("Error reading file type: {}", e))),
        };

        let is_dot_file = check_dot(&name);

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

#[tauri::command]
fn open_file(path: String) -> Result<String, String> {
    match Command::new("open").args([path]).output() {
        Ok(_) => Ok(String::from("Successfully opened file")),
        Err(e) => Err(String::from(format!("Error opening file: {}", e))),
    }
}

#[tauri::command]
fn search(q: String, path: String) -> Vec<FileData> {
    let mut files = vec![];

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !is_node_module(e))
    {
        let file = match entry {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                continue;
            }
        };
        let full_path = file.path().display().to_string();
        let name = get_file_name(&full_path);
        let file_type = check_type(file.file_type(), &full_path).unwrap_or("unknown".to_string());
        println!("{}", full_path);
        if full_path.contains(&q) {
            let file_data = FileData {
                full_path,
                name,
                is_dot_file: false,
                file_type,
            };
            files.push(file_data);
            return files;
        }
    }
    let file_data = FileData {
        full_path: String::from(""),
        file_type: String::from("unknown"),
        is_dot_file: false,
        name: String::from("No files found"),
    };
    files.push(file_data);
    files
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_root, open_dir, open_file, search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
