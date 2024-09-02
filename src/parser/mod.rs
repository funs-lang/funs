// pub mod old_parser;
pub mod ast;

use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use std::cell::Cell;
use tracing::error;

struct Tree {
    kind: TreeKind,
    children: Vec<Child>,
}

enum TreeKind {
    ErrorTree,
    File,
    Block,
    StmtVarDecl,
    TypeExpr,
}

enum Child {
    Tree(Tree),
    Token(Token),
}

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
        self.pos == self.events.len()
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

    pub fn parse_module(&mut self) {
        let m = self.open();
        self.parse_block();
        self.close(m, TreeKind::File);
    }

    // File = (Stmt | Comment)*
    //
    // Stmt =
    //   StmtVarDecl
    // | StmtFunDecl
    // | Expr
    fn parse_block(&mut self) {
        let m = self.open();
        while !self.eof() {
            match self.nth(0) {
                TokenKind::TokenEOF => break,
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

    fn parse_stmt_expr(&mut self) {

    }

    fn parse_comment(&mut self) {}
    fn parse_fun_decl(&mut self) {}
}
