pub mod ast;
pub mod old_parser; // TODO: Remove

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
    FunctionDecl,
    StmtDecl,
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

const INITIAL_FUEL: u32 = 256;

struct Parser {
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
    fn new(lexer: impl IntoIterator<Item = Token, IntoIter = Vec<Token>>) -> Self {
        Parser {
            tokens: lexer.into_iter(),
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

    fn advence_with_error(&mut self, error: &str) {
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
}
