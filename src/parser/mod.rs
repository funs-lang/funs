use crate::lexer::token::Literal;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use serde::Deserialize;
use serde::Serialize;
use std::cell::Cell;
use tracing::error;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Tree {
    kind: TreeKind,
    children: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum TreeKind {
    ErrorTree,
    File,
    StmtVarDecl,
    TypeExpr,
    StmtExpr,
    ExprLiteral,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Child {
    Tree(Tree),
    Token(Token),
}

#[derive(Debug)]
enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
}

struct MarkOpened {
    index: usize,
}

// Grammar:
//
// File = (Stmt | Comment)*
//
// Stmt =
//   StmtVarDecl
// | StmtFunDecl
// | StmtExpr
//
// StmtExpr = Expr "\n"
// StmtDeclVar = Ident: Type "=" Expr
// Comment = "#" [^\n]*
//
// Expr =
//   Ident
// | ExprLiteral
// | ExprBinary
// | ExprUnary
// | ExprParen
// | ExprFunCall
//
// ExprLiteral = Int | Float | Bool | Str
// ExprBinary = Expr ("+" | "-" | "*" | "/") Expr
// ExprUnary = ("+" | "-") Expr
// ExprParen = "(" Expr ")"
//
// Ident = [a-zA-Z_][a-zA-Z0-9_]*
// Int = [0-9]+
// Float = [0-9]+\.[0-9]+
// Bool = "true" | "false"
// Str = "\"" [^\n]* "\""
// Type =
//   Ident
// | "[" Type "]"
// | "(" Type ("," Type)* ")"
//
// ExprFunCall = Ident Expr*
//
// --- TODO ---
// DeclFun = Ident ":" ParamList "->" Type = (Ident) "->" (Expr | Block) ";"
// TypeParamList = "(" ((Type | "unit") ("," Type)*)? ")"

const INITIAL_FUEL: u32 = 256;
pub struct Parser {
    /// The tokens that the parser is consuming.
    tokens: Vec<Token>,
    /// The current fuel of the parser.
    /// The parser will stop parsing if the fuel reaches 0 in order to prevent infinite loops.
    fuel: Cell<u32>,
    /// The current position in the event list.
    pos: usize,
    /// The events that the parser has generated in the first pass.
    events: Vec<Event>,
}

impl Parser {
    pub fn new(lexer: impl IntoIterator<Item = Token>) -> Self {
        Parser {
            tokens: lexer.into_iter().collect(),
            fuel: Cell::new(INITIAL_FUEL),
            pos: 0,
            events: Vec::new(),
        }
    }

    // This function is used to open a new tree in the event list.
    //
    // It will mark the current position as an `TokenKind::ErrorTree` and return a `MarkOpened`
    // that can be used to close the tree later.
    fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: TreeKind::ErrorTree,
        });
        mark
    }

    /// This function is used to close a tree that was opened with `open`.
    ///
    /// The `mark` argument indicates the position of the `open` call in the event list.
    /// The `kind` argument indicates the kind of the tree that is being closed, replacing
    /// the `TokenKind::ErrorTree` that was used when the tree was opened.
    fn close(&mut self, mark: MarkOpened, kind: TreeKind) {
        self.events[mark.index] = Event::Open { kind };
        self.events.push(Event::Close);
    }

    /// This function is used to advance the parser to the next token.
    ///
    /// It will set the fuel to `INITIAL_FUEL` in order to prevent infinite loops.
    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(INITIAL_FUEL);
        self.events.push(Event::Advance);
        self.pos += 1;
    }

    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> TokenKind {
        if self.fuel.get() == 0 {
            error!("The parser has run out of fuel");
            panic!("The parser has run out of fuel");
        }

        self.fuel.set(self.fuel.get() - 1);
        self.tokens
            .get(self.pos + lookahead)
            .map_or(TokenKind::TokenEOF, |it| it.kind.clone())
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.nth(0) == kind
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expext(&mut self, kind: TokenKind) {
        if self.eat(kind.clone()) {
            return;
        }

        // TODO: Error reporting
        eprintln!("Expected {kind:?}");
    }

    fn advance_with_error(&mut self, error: &str) {
        let m = self.open();

        // TODO: Error reporting
        eprintln!("{error}");
        error!("{error}");
        self.advance();
        self.close(m, TreeKind::ErrorTree);
    }

    fn build_tree(self) -> Tree {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut stack = Vec::<Tree>::new();

        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                // Open a new tree.
                // Push an empty tree to the stack.
                Event::Open { kind } => stack.push(Tree {
                    kind,
                    children: Vec::new(),
                }),
                // A tree is done.
                // Pop it off the stack and append to a new current tree.
                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack
                        .last_mut()
                        // If we don't pop the last `Close` before this loop,
                        // this unwrap would trigger for it.
                        .unwrap()
                        .children
                        .push(Child::Tree(tree));
                }
                // Advance to the next token.
                // Append the token to the current tree.
                Event::Advance => {
                    let token = tokens.next().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
            }
        }

        // The parser will guarantee that all trees are closed and all tokens are consumed.
        assert!(stack.len() == 1);
        assert!(tokens.next().is_none());

        stack.pop().unwrap()
    }

    pub fn parse(mut self) -> Tree {
        self.parse_file();
        self.build_tree()
    }

    // File = (Stmt | Comment)*
    //
    // Stmt =
    //   StmtVarDecl
    // | StmtFunDecl
    // | Expr
    fn parse_file(&mut self) {
        let m = self.open();
        while !self.eof() {
            match self.nth(0) {
                TokenKind::TokenEOF => self.advance(),
                TokenKind::TokenComment => self.parse_comment(),
                TokenKind::TokenIdentifier => {
                    if self.nth(1) == TokenKind::TokenColon {
                        if self.nth(2) == TokenKind::TokenOpenParen {
                            self.parse_fun_decl();
                        } else {
                            self.parse_var_decl();
                        }
                    } else {
                        self.parse_expr();
                    }
                }
                _ => self.advance_with_error("Expected statement"),
            }
        }
        self.close(m, TreeKind::File);
    }

    // StmtDeclVar = Ident: Type "=" StmtExpr
    fn parse_var_decl(&mut self) {
        assert!(self.at(TokenKind::TokenIdentifier));
        let m = self.open();

        self.expext(TokenKind::TokenIdentifier);
        self.expext(TokenKind::TokenColon);
        self.parse_type();
        self.expext(TokenKind::TokenAssign);
        self.parse_stmt_expr();

        self.close(m, TreeKind::StmtVarDecl);
    }

    // Type =
    //   Ident
    // | "[" Type "]"
    // | "(" Type ("," Type)* ")"
    fn parse_type(&mut self) {
        let m = self.open();

        match self.nth(0) {
            TokenKind::TokenIdentifier => self.expext(TokenKind::TokenIdentifier),
            TokenKind::TokenOpenBracket => {
                self.expext(TokenKind::TokenOpenBracket);
                self.parse_type();
                self.expext(TokenKind::TokenCloseBracket);
            }
            TokenKind::TokenOpenParen => {
                self.expext(TokenKind::TokenOpenParen);
                self.parse_type();
                while self.eat(TokenKind::TokenComma) {
                    self.parse_type();
                }
                self.expext(TokenKind::TokenCloseParen);
            }
            _ => self.advance_with_error("Expected type"),
        }

        self.close(m, TreeKind::TypeExpr);
    }

    // StmtExpr = Expr "\n"
    fn parse_stmt_expr(&mut self) {
        let m = self.open();
        self.parse_expr();
        self.expext(TokenKind::TokenNewLine);
        self.close(m, TreeKind::StmtExpr);
    }

    // Expr =
    //   Ident
    // | ExprLiteral
    // | ExprBinary
    // | ExprUnary
    // | ExprParen
    // | ExprFunCall
    fn parse_expr(&mut self) {
        let m = self.open();

        match self.nth(0) {
            TokenKind::TokenLiteral(Literal::Int)
            | TokenKind::TokenLiteral(Literal::Float)
            | TokenKind::TokenLiteral(Literal::Bool)
            | TokenKind::TokenLiteral(Literal::Str) => {
                self.advance();
                self.close(m, TreeKind::ExprLiteral);
            }
            _ => unimplemented!(),
        }
    }

    fn parse_comment(&mut self) {}
    fn parse_fun_decl(&mut self) {}
}

#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::Lexer, parser::Parser, source::Source, utils::file_handler::collect_fs_files,
    };
    use tracing::info;

    #[test]
    fn test_parser_native_types() {
        let fs_files = collect_fs_files("./testdata/native_types", true);
        assert_eq!(fs_files.len(), 15);

        let fs_files = fs_files.iter().filter(|p| p.ends_with("id_int_assign.fs"));

        for path in fs_files {
            info!("file -> {:?}", path);
            eprintln!("file -> {:?}", path);
            let input = std::fs::File::open(path.clone()).unwrap();
            let content = std::io::read_to_string(input).unwrap();
            #[cfg(target_os = "windows")]
            let content = content.replace("\r\n", "\n");
            let source = Source::from(content);
            let fs_file = path.to_str().unwrap();

            let output_ast = Parser::new(Lexer::new(&source)).parse();
            let ast_file = fs_file.to_string().replace(".fs", ".ast.json");
            let json_ast = std::fs::File::open(ast_file).unwrap();
            // println!("{}", serde_json::to_string(&output_tree).unwrap());
            let expected_ast = serde_json::from_reader(json_ast).unwrap();
            assert_eq!(output_ast, expected_ast);
        }
    }
}
