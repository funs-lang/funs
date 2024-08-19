use std::fs::File;
use std::io::Write;
use std::path::Path;

#[cfg(test)]
/// Collect all fs files in the given path.
/// This is util function for testing.
pub fn collect_fs_files(path: &str, set_logger: bool) -> Vec<std::path::PathBuf> {
    if set_logger {
        let subscriber = tracing_subscriber::fmt()
            // filter spans/events with level TRACE or higher.
            .with_max_level(tracing::Level::TRACE)
            // build but do not install the subscriber.
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber)
            .map_err(|_err| eprintln!("Unable to set global default subscriber"));
    }

    std::fs::read_dir(path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if let Some(extension) = path.extension() {
                if extension == "fs" {
                    return Some(path);
                }
            }
            None
        })
        .collect()
}

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
