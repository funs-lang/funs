use crate::{
    lexer::token::{Literal, Token, TokenKind, TokenLocation},
    source::Source,
};
use serde::{Deserialize, Serialize};
use tracing::error;
use tracing::info;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum AstError {
    UnexpectedToken { token: Token },
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AstError::UnexpectedToken { token } => {
                write!(f, "Unexpected token: {}", token)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Ast {
    pub source: Source,
    pub root: Block,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Block {
    pub stmts: Box<[Stmt]>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Stmt {
    Assign {
        lhs: Expr,
        type_: Type,
        rhs: Result<Expr, AstError>,
    },
    Expr(Expr),
    Comment {
        comment: String,
        location: TokenLocation,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Expr {
    Literal {
        literal: TypeLiteral,
        location: TokenLocation,
    },
    Identifier {
        name: String,
        location: TokenLocation,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum TypeLiteral {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl Ast {
    pub fn new(source: Source, root: Block) -> Ast {
        Ast { source, root }
    }
}

pub struct Parser<I: IntoIterator> {
    lexer: I::IntoIter,
    curr_token: Option<Token>,
    source: Source,
}

impl<I: IntoIterator<Item = Token>> Parser<I> {
    pub fn new(source: Source, lexer: I) -> Parser<I> {
        let mut lexer = lexer.into_iter();
        let source = source.clone();
        let curr_token = lexer.next();
        info!("Created Parser");
        Parser {
            lexer,
            curr_token,
            source,
        }
    }

    fn consume(&mut self) {
        self.curr_token = self.lexer.next();
    }

    pub fn parse(&mut self) -> Ast {
        let source = self.source.clone();
        let Block { stmts } = self.parse_block();
        let root = Block { stmts };
        Ast::new(source, root)
    }

    fn parse_block(&mut self) -> Block {
        let mut stmts = Vec::new();
        loop {
            match self.curr_token {
                Some(Token {
                    kind: TokenKind::TokenEOF,
                    ..
                }) => break,
                Some(_) => {
                    let stmt = self.parse_stmt();
                    stmts.push(stmt);
                }
                _ => (),
            }
        }
        Block {
            stmts: stmts.into_boxed_slice(),
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match &self.curr_token {
            Some(Token {
                kind: TokenKind::TokenIdentifier,
                ..
            }) => {
                let stms = self.parse_identifier_stmt();
                info!("Parsed identifier - {:?}", stms);
                stms
            }
            Some(Token {
                kind: TokenKind::TokenComment,
                ..
            }) => {
                let comment = self.parse_comment_stmt();
                info!("Parsed comment - {:?}", comment);
                comment
            }
            c => todo!("{:?}", c),
        }
    }

    fn parse_identifier_stmt(&mut self) -> Stmt {
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

    fn parse_assign_stmt(&mut self, lhs: Token) -> Stmt {
        let lhs = Expr::Identifier {
            name: lhs.lexeme,
            location: lhs.location,
        };
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
                        Stmt::Assign {
                            lhs,
                            type_,
                            rhs: Ok(rhs),
                        }
                    }
                    _ => todo!(),
                }
            }
            Some(Token {
                kind: TokenKind::TokenUnknown,
                ..
            }) => {
                let err = AstError::UnexpectedToken {
                    token: self.curr_token.clone().unwrap(),
                };
                error!("{}", err);

                self.consume_until_new_statement();

                Stmt::Assign {
                    lhs,
                    type_,
                    rhs: Err(err),
                }
            }
            _ => todo!(),
        }
    }

    fn parse_comment_stmt(&mut self) -> Stmt {
        let comment = self.curr_token.clone().unwrap(); // Safe to unwrap
        self.consume();
        match self.curr_token {
            Some(Token {
                kind: TokenKind::TokenNewLine,
                ..
            }) => {
                self.consume();
                Stmt::Comment {
                    comment: comment.lexeme,
                    location: comment.location,
                }
            }
            _ => todo!(),
        }
    }

    fn parse_type(&mut self) -> Type {
        match &self.curr_token {
            Some(Token {
                kind: TokenKind::TokenIdentifier,
                lexeme,
                ..
            }) => match lexeme.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "bool" => Type::Bool,
                "str" => Type::Str,
                _ => todo!(), // Error of invalid type
            },
            _ => todo!(), // Error of unexpected token
        }
    }

    fn parse_expr(&mut self) -> Expr {
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

    fn parse_literal_expr(&mut self) -> Expr {
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
                    Expr::Literal {
                        literal: TypeLiteral::Int(int),
                        location: token.location,
                    }
                }
                Literal::Float => {
                    let float = match token.lexeme.parse::<f64>() {
                        Ok(float) => float,
                        Err(_) => todo!(), // Error of invalid float
                    };
                    Expr::Literal {
                        literal: TypeLiteral::Float(float),
                        location: token.location,
                    }
                }
                Literal::Bool => {
                    let bool_ = match token.lexeme.parse::<bool>() {
                        Ok(bool_) => bool_,
                        Err(_) => todo!(), // Error of invalid bool
                    };
                    Expr::Literal {
                        literal: TypeLiteral::Bool(bool_),
                        location: token.location,
                    }
                }
                Literal::Str => {
                    let str_ = token.lexeme.clone();
                    Expr::Literal {
                        literal: TypeLiteral::Str(str_),
                        location: token.location,
                    }
                }
            },
            _ => todo!(),
        }
    }

    fn consume_until_new_statement(&mut self) {
        // Consume all tokens until a newline token is found
        while self.curr_token.is_some() {
            if let Some(Token {
                kind: TokenKind::TokenNewLine,
                ..
            }) = self.curr_token
            {
                self.consume();
                break;
            }
            self.consume();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::Lexer, parser::old_parser::Parser, source::Source,
        utils::file_handler::collect_fs_files,
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
                || p.ends_with("comment.fs")
                || p.ends_with("comment_and_id_int.fs")
                || p.ends_with("id_int_assign_with_len_one.fs")
                || p.ends_with("id_int_assign_with_spaces.fs")
                || p.ends_with("id_float_assign.fs")
                || p.ends_with("id_bool_true_assign.fs")
                || p.ends_with("id_bool_false_assign.fs")
                || p.ends_with("id_str_assign.fs")
                || p.ends_with("id_str_assign_multiple_words.fs")
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
    fn test_parser_errors() {
        let fs_files = collect_fs_files("./testdata/errors", true);
        assert_eq!(fs_files.len(), 2);

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
}
