use std::fs::{metadata, read_link, FileType};

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
