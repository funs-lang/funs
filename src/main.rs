mod logger;

use logger::Logger;
use std::env;
use std::fs;
use std::path::PathBuf;

fn set_up_logger() {
    let pwd: PathBuf = env::current_dir().unwrap_or_else(|e| {
        panic!("Error getting current directory: {}", e);
    });
    let logger_file_path = pwd.join(".log").join("debug.log");
    let _logger = Logger::init(logger_file_path);
}

fn main() {
    set_up_logger();

    let usage_message: &str = "Usage: \n\
                               funs <file.fs>";
    let args: &[String] = &env::args().collect::<Vec<String>>()[1..];
    if args.len() != 1 {
        println!("{}", usage_message);
        return;
    }

    let file_path: &str = &args[0];
    let _content: String = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading file \"{}\": {}", file_path, e);
        }
    };
}
