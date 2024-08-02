use std::env;

fn main() {
    let usage_message: &str = "Usage: \n\
                              funs <file.fs>";
    let args: &[String] = &env::args().collect::<Vec<String>>()[1..];
    assert!(args.len() == 1, "{}", usage_message);

    let file_path: &str = &args[0];
}
