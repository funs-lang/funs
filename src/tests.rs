#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::{token::Token, Lexer},
        parser::Parser,
        source::Source,
    };
    use pretty_assertions::assert_eq;
    use tracing::info;

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

    #[test]
    fn native_types() {
        let fs_files = collect_fs_files("./testdata/native_types", true);
        assert_eq!(fs_files.len(), 16);

        let fs_files = fs_files.iter().filter(|p| p.ends_with("id_int_assign.fs"));

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let mut lexer = Lexer::new(&source);
            let output_tokens = (&mut lexer).collect::<Vec<Token>>();

            let tokens_file = path.to_str().unwrap();
            let tokens_file = tokens_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file.clone()).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);

            let output_ast = Parser::new(Lexer::new(&source)).parse();
            let ast_file = tokens_file.to_string().replace(".tokens.json", ".ast.json");
            let ast = std::fs::File::open(ast_file).unwrap();
            // println!("{}", serde_json::to_string(&output_ast).unwrap());
            let expected_ast = serde_json::from_reader(ast).unwrap();
            assert_eq!(output_ast, expected_ast);
        }
    }

    #[test]
    fn functions() {
        let fs_files = collect_fs_files("./testdata/functions", true);
        assert_eq!(fs_files.len(), 9);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let tokens_file = path.to_str().unwrap();
            let tokens_file = tokens_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn lists() {
        let fs_files = collect_fs_files("./testdata/lists", true);
        assert_eq!(fs_files.len(), 3);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let tokens_file = path.to_str().unwrap();
            let tokens_file = tokens_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn tuples() {
        let fs_files = collect_fs_files("./testdata/tuples", true);
        assert_eq!(fs_files.len(), 3);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let tokens_file = path.to_str().unwrap();
            let tokens_file = tokens_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn records() {
        let fs_files = collect_fs_files("./testdata/records", true);
        assert_eq!(fs_files.len(), 3);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let tokens_file = path.to_str().unwrap();
            let tokens_file = tokens_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }

    #[test]
    fn variants() {
        let fs_files = collect_fs_files("./testdata/variants", true);
        assert_eq!(fs_files.len(), 1);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            let source = Source::from(content);
            let lexer = Lexer::new(&source);
            let output_tokens = lexer.collect::<Vec<Token>>();

            let tokens_file = path.to_str().unwrap();
            let tokens_file = tokens_file.to_string().replace(".fs", ".tokens.json");
            let tokens = std::fs::File::open(tokens_file).unwrap();
            let expected_tokens: Vec<Token> = serde_json::from_reader(tokens).unwrap();
            assert_eq!(output_tokens, expected_tokens);
        }
    }
}
