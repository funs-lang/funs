use crate::source::Source;

pub struct Ast {
    pub source: Source,
    pub root: Block,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Box<[Stmt]>,
}

#[derive(Debug)]
pub enum Stmt {
    Assign {
        ident: Identifier,
        type_: Type,
        expr: Expr,
    },
    Expr(Expr),
}

#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

impl Ast {
    pub fn new(source: Source, root: Block) -> Ast {
        Ast { source, root }
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Block [{}]", self.root)
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for stmt in self.stmts.iter() {
            write!(f, " {} ", stmt)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stmt::Assign { ident, type_, expr } => write!(
                f,
                "Stmt::Assign {{ ident: {}, type: {}, expr: {} }}",
                ident, type_, expr
            ),
            Stmt::Expr(expr) => write!(f, "Stmt::Expr({})", expr),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Str => write!(f, "str"),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Identifier(ident) => write!(f, "{}", ident),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Int(int) => write!(f, "{}", int),
            Literal::Float(float) => write!(f, "{}", float),
            Literal::Bool(bool) => write!(f, "{}", bool),
            Literal::Str(string) => write!(f, "{}", string),
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
