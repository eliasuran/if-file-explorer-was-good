// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs::read_dir, process::Command, time::SystemTime};

use rust_file_explorer::{
    check_dot, check_type, get_file_name, get_root_dir, is_hidden, is_node_module, is_onedrive,
};
use serde::Serialize;
use tauri::Manager;
use walkdir::WalkDir;

#[derive(Serialize)]
struct OpenDirReturn {
    current_path: String,
    file_data: Vec<FileData>,
}

#[derive(Serialize, Clone)]
struct FileData {
    name: String,
    full_path: String,
    file_type: String,
    is_dot_file: bool,
}

#[tauri::command]
fn open_root() -> Result<OpenDirReturn, String> {
    let user = whoami::username();
    let os = env::consts::OS;
    println!("user {} on os {}", user, os);

    let root_dir = get_root_dir(os, &user);

    match open_dir(root_dir) {
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

#[derive(Clone, Serialize)]
struct Payload {
    data: Vec<FileData>,
    done: bool,
}

// search command returning files as they are found
#[tauri::command]
async fn search_files(q: String, path: String, app: tauri::AppHandle) -> Result<(), String> {
    let start = SystemTime::now();

    let mut results = vec![];
    let mut walker = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && !is_node_module(e) && !is_onedrive(e));

    while let Some(entry) = walker.next() {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error getting entry: {}", e);
                continue;
            }
        };

        let full_path = entry.path().display().to_string();
        println!("{}", full_path);
        if full_path.contains(&q) {
            let name = get_file_name(&full_path);
            let file_type =
                check_type(entry.file_type(), &full_path).unwrap_or("unknown".to_string());

            let file_data = FileData {
                full_path,
                name,
                file_type,
                is_dot_file: false,
            };
            results.push(file_data.clone());

            app.emit_all(
                "incoming-data",
                Payload {
                    data: results.clone(),
                    done: false,
                },
            )
            .map_err(|e| String::from(format!("Error getting data: {}", e)))?
        }
    }
    app.emit_all(
        "incoming-data",
        Payload {
            data: results.clone(),
            done: true,
        },
    )
    .map_err(|e| String::from(format!("Error getting data: {}", e)))?;

    let done = SystemTime::now();

    let elapsed = done.duration_since(start).unwrap();

    println!("Done searching, elapsed time: {}", elapsed.as_secs_f64());


    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_root,
            open_dir,
            open_file,
            search_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
