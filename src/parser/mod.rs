pub mod ast;
use crate::{
    lexer::token::{Keyword, Literal, Token, TokenKind},
    source::Source,
};
use tracing::info;

pub struct Parser<I: IntoIterator> {
    lexer: I::IntoIter,
    curr_token: Option<Token>,
    source: Source,
}

impl<I: IntoIterator<Item = Token>> Parser<I> {
    pub fn new(source: Source, lexer: I) -> Parser<I> {
        let mut lexer = lexer.into_iter();
        let source = source.clone();
        info!("Created Parser");
        let curr_token = lexer.next();
        Parser {
            lexer,
            curr_token,
            source,
        }
    }

    fn consume(&mut self) {
        self.curr_token = self.lexer.next();
    }

    pub fn parse(&mut self) -> ast::Ast {
        let source = self.source.clone();
        let ast::Block { stmts } = self.parse_block().unwrap();
        let root = ast::Block { stmts };
        ast::Ast::new(source, root)
    }

    fn parse_block(&mut self) -> Option<ast::Block> {
        let mut stmts = Vec::new();
        loop {
            match self.curr_token {
                Some(Token {
                    kind: TokenKind::TokenEOF,
                    ..
                }) => break,
                Some(_) => {
                    let stmt = self.parse_stmt();
                    match stmt {
                        Some(stmt) => stmts.push(stmt),
                        None => break,
                    }
                }
                _ => (),
            }
        }
        Some(ast::Block {
            stmts: stmts.into_boxed_slice(),
        })
    }

    fn parse_stmt(&mut self) -> Option<ast::Stmt> {
        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenIdentifier,
                ..
            }) => {
                let stms = self.parse_identifier_stmt();
                info!("Parsed identifier - {:?}", stms);
                Some(stms)
            }
            Some(Token {
                kind: TokenKind::TokenComment,
                ..
            }) => {
                let comment = self.parse_comment_stmt();
                info!("Parsed comment - {:?}", comment);
                Some(comment)
            }
            _ => todo!(),
        }
    }

    fn parse_identifier_stmt(&mut self) -> ast::Stmt {
        let lhs = self.curr_token.clone().unwrap(); // Safe to unwrap
        self.consume();

        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenColon,
                ..
            }) => {
                self.consume();
                match self.curr_token {
                    Some(Token {
                        kind: TokenKind::TokenKeyword(_),
                        ..
                    }) => self.parse_assign_stmt(lhs),
                    _ => todo!(), // Match `(` and parse a function
                }
            }
            _ => todo!(),
        }
    }

    fn parse_assign_stmt(&mut self, lhs: Token) -> ast::Stmt {
        let type_ = self.parse_type();
        info!("Parsed type - {:?}", type_);
        self.consume();
        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenAssign,
                ..
            }) => {
                self.consume();
                let rhs = self.parse_expr();
                info!("Parsed expr - {:?}", rhs);
                self.consume();
                match self.curr_token {
                    Some(Token {
                        kind: TokenKind::TokenNewLine,
                        ..
                    }) => {
                        self.consume();
                        ast::Stmt::Assign {
                            lhs: ast::Expr::Identifier {
                                name: lhs.lexeme,
                                location: lhs.location,
                            },
                            type_,
                            rhs,
                        }
                    }
                    _ => todo!(),
                }
            }
            Some(Token {
                kind: TokenKind::TokenUnknown,
                ..
            }) => panic!("Unexpected token {:?}", self.curr_token),
            _ => todo!(),
        }
    }

    fn parse_comment_stmt(&mut self) -> ast::Stmt {
        let comment = self.curr_token.clone().unwrap(); // Safe to unwrap
        self.consume();
        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenNewLine,
                ..
            }) => {
                self.consume();
                ast::Stmt::Comment {
                    comment: comment.lexeme,
                    location: comment.location,
                }
            }
            _ => todo!(),
        }
    }

    fn parse_type(&mut self) -> ast::Type {
        match &self.curr_token {
            Some(Token {
                kind: TokenKind::TokenKeyword(keyword),
                ..
            }) => match keyword {
                Keyword::IntType => ast::Type::Int,
                Keyword::FloatType => ast::Type::Float,
                Keyword::BoolType => ast::Type::Bool,
                Keyword::StrType => ast::Type::Str,
                _ => todo!(), // Error of invalid type
            },
            _ => todo!(), // Error of unexpected token
        }
    }

    fn parse_expr(&mut self) -> ast::Expr {
        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenLiteral(_),
                ..
            }) => self.parse_literal_expr(),
            // Some(Token {
            //     kind: TokenKind::TokenIdentifier,
            //     ..
            // }) => self.parse_identifier_expr(),
            _ => todo!(),
        }
    }

    fn parse_literal_expr(&mut self) -> ast::Expr {
        let token = self.curr_token.clone().unwrap(); // Safe to unwrap
        match &self.curr_token {
            Some(Token {
                kind: TokenKind::TokenLiteral(literal),
                ..
            }) => match literal {
                Literal::Int => {
                    let int = match token.lexeme.parse::<i64>() {
                        Ok(int) => int,
                        Err(_) => todo!(), // Error of invalid integer
                    };
                    ast::Expr::Literal {
                        literal: ast::Literal::Int(int),
                        location: token.location,
                    }
                }
                Literal::Float => {
                    let float = match token.lexeme.parse::<f64>() {
                        Ok(float) => float,
                        Err(_) => todo!(), // Error of invalid float
                    };
                    ast::Expr::Literal {
                        literal: ast::Literal::Float(float),
                        location: token.location,
                    }
                }
                Literal::Bool => {
                    let bool_ = match token.lexeme.parse::<bool>() {
                        Ok(bool_) => bool_,
                        Err(_) => todo!(), // Error of invalid bool
                    };
                    ast::Expr::Literal {
                        literal: ast::Literal::Bool(bool_),
                        location: token.location,
                    }
                }
                Literal::Str => {
                    let str_ = token.lexeme.clone();
                    ast::Expr::Literal {
                        literal: ast::Literal::Str(str_),
                        location: token.location,
                    }
                }
            },
            _ => todo!(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::Lexer, parser::Parser, source::Source, utils::file_handler::collect_fs_files,
    };
    use pretty_assertions::assert_eq;
    use tracing::info;

    #[test]
    fn test_parser_native_types() {
        let fs_files = collect_fs_files("./testdata/native_types", true);
        assert_eq!(fs_files.len(), 15);

        let fs_files = fs_files.iter().filter(|p| {
            p.ends_with("id_int_assign.fs")
                || p.ends_with("id_int_assign_2.fs")
                || p.ends_with("id_int_assign_with_len_one.fs")
                || p.ends_with("id_int_assign_with_spaces.fs")
                || p.ends_with("id_float_assign.fs")
                || p.ends_with("id_bool_true_assign.fs")
                || p.ends_with("id_bool_false_assign.fs")
                || p.ends_with("id_str_assign.fs")
                || p.ends_with("id_str_assign_multiple_words.fs")
                || p.ends_with("comment.fs")
                || p.ends_with("comment_and_id_int.fs")
        });

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            #[cfg(target_os = "windows")]
            let content = content.replace("\r\n", "\n");
            let source = Source::from(content);

            let fs_file = path.to_str().unwrap();

            let output_ast = Parser::new(source.clone(), Lexer::new(&source)).parse();
            let ast_file = fs_file.to_string().replace(".fs", ".ast.json");
            let ast = std::fs::File::open(ast_file).unwrap();
            println!("{}", serde_json::to_string(&output_ast.root).unwrap());
            let expected_ast = serde_json::from_reader(ast).unwrap();
            assert_eq!(output_ast.root, expected_ast);
        }
    }

    #[test]
    #[should_panic]
    fn test_parser_panics() {
        let fs_files = collect_fs_files("./testdata/panics", true);
        assert_eq!(fs_files.len(), 1);

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            #[cfg(target_os = "windows")]
            let content = content.replace("\r\n", "\n");
            let source = Source::from(content);

            let _fs_file = path.to_str().unwrap();

            let _output_ast = Parser::new(source.clone(), Lexer::new(&source)).parse();
            // let ast_file = fs_file.to_string().replace(".fs", ".ast.json");
            // let ast = std::fs::File::open(ast_file).unwrap();
            // println!("{}", serde_json::to_string(&output_ast.root).unwrap());
            // let expected_ast = serde_json::from_reader(ast).unwrap();
            // assert_eq!(output_ast.root, expected_ast);
        }
    }
}
