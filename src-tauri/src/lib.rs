use std::fs::{metadata, read_link, FileType};

use walkdir::DirEntry;

pub fn get_root_dir(os: &str, user: &str) -> String {
    if os == "macos" {
        return format!("/Users/{}", user);
    } else if os == "windows" {
        return r"C:\".to_string();
    }
    "/".to_string()
}

pub fn get_file_name(full_path: &str) -> String {
    let name = full_path
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();
    name.to_string()
}

pub fn check_type(file: FileType, path: &str) -> Result<String, String> {
    if file.is_dir() {
        Ok(String::from("dir"))
    } else if file.is_file() {
        Ok(String::from("file"))
    } else if file.is_symlink() {
        let link_path = match read_link(path) {
            Ok(v) => v,
            Err(e) => return Err(String::from(format!("Unable to read symlink: {}", e))),
        };

        let metadata = match metadata(&link_path) {
            Ok(v) => v.file_type(),
            Err(e) => return Err(String::from(format!("Error getting metadata: {}", e))),
        };

        match check_type(metadata, link_path.to_str().unwrap()) {
            Ok(v) => return Ok(v),
            Err(e) => {
                return Err(String::from(format!(
                    "Error checking type of symlink: {}",
                    e
                )))
            }
        };
    } else {
        Ok(String::from("unknown"))
    }
}

pub fn check_dot(file: &str) -> bool {
    if file
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string()
        .chars()
        .next()
        .unwrap()
        == '.'
    {
        return true;
    }
    false
}

// for walkdir
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn is_node_module(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s == "node_modules")
        .unwrap_or(false)
}

pub fn is_onedrive(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s == "OneDrive-Osloskolen")
        .unwrap_or(false)
}
