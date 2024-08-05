use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn create_tmp_file(file_path: &str, content: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}

pub fn remove_tmp_file(file_path: &str) {
    let path = Path::new(file_path);
    std::fs::remove_file(path).expect("Failed to remove file");
}
