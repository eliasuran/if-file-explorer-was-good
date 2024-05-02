use std::fs::FileType;

pub fn check_type(file: FileType) -> String {
    if file.is_dir() {
        return String::from("dir");
    };
    String::from("file")
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
