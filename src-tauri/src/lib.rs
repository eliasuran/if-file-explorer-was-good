use std::fs::FileType;

pub fn check_type(file: FileType) -> String {
    if file.is_dir() {
        return String::from("dir");
    };
    String::from("file")
}

pub fn check_dot(file: &str) -> bool {
    let split_file_path = file.split("/").collect::<Vec<&str>>();
    if split_file_path[split_file_path.len() - 1]
        .chars()
        .next()
        .unwrap()
        == '.'
    {
        return true;
    }
    false
}
