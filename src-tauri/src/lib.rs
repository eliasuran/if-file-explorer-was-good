use std::fs::{read_dir, read_link, FileType};

pub fn check_type(file: FileType, path: &str) -> Result<String, String> {
    if file.is_dir() {
        Ok(String::from("dir"))
    } else if file.is_file() {
        Ok(String::from("file"))
    } else if file.is_symlink() {
        match read_link(path) {
            Ok(v) => {
                let file = read_dir(path);
                Ok("".to_string())
            }
            Err(e) => return Err(String::from(format!("Unable to read symlink: {}", e))),
        }
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
