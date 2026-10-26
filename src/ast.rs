use crate::language::BinaryOp;
use miette::SourceSpan;

impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        SourceSpan::new(span.start.into(), span.end - span.start)
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub alias: Option<String>,
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Import(Import),
    Struct(Struct),
    Function(Function),
    // Reserved for future top-level executable statements; the parser currently rejects them.
    #[allow(dead_code)]
    Statement(Stmt),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub span: Span,
    pub type_span: Span,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub intrinsic: Option<String>,
    pub span: Span,
    pub return_type_span: Option<Span>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub span: Span,
    pub type_span: Span,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    Number(String),
    String(String),
    Bool(bool),
    Identifier(String),
    Wildcard,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        ty: Option<Type>,
        lifetime_ms: Option<u64>,
        value: Expr,
        span: Span,
    },

    Destructure {
        struct_name: String,
        mutable: bool,
        bindings: Vec<(String, String)>,
        value: Expr,
        span: Span,
    },

    Return(Option<Expr>, Span),

    Stop(Option<Expr>, Span),

    Skip(Span),

    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
        span: Span,
    },

    Loop {
        kind: LoopKind,
        binding: Option<String>,
        body: Block,
        span: Span,
    },

    Expr(Expr, Span),

    Match {
        expression: Expr,
        arms: Vec<MatchArm>,
        span: Span,
    },
    When {
        condition: Expr,
        body: Block,
        span: Span,
    },
}

#[derive(Debug, Clone)]
pub enum LoopKind {
    Infinite,
    Repeat(Expr),
    While(Expr),

    // Retained in the AST/IR model for iterable-loop support, which is not parsed yet.
    #[allow(dead_code)]
    For {
        variable: String,
        iterable: Expr,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(String, Span),
    String(String, Span),
    Interpolation(Vec<String>, Span),
    Bool(bool, Span),

    Identifier(String, Span),
    Previous(String, Span),

    StructLiteral {
        name: String,
        fields: Vec<(String, Expr)>,
        span: Span,
    },

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },

    Pipe {
        input: Box<Expr>,
        target: Box<Expr>,
        span: Span,
    },

    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        span: Span,
    },

    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        span: Span,
    },

    Member {
        object: Box<Expr>,
        member: String,
        span: Span,
    },

    Shell {
        command: String,
        span: Span,
    },

    // Intrinsics are currently lowered from function declarations, not parsed as expressions.
    #[allow(dead_code)]
    Intrinsic {
        name: String,
        arguments: Vec<Expr>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Number(_, span) => *span,
            Expr::String(_, span) => *span,
            Expr::Interpolation(_, span) => *span,
            Expr::Bool(_, span) => *span,
            Expr::Identifier(_, span) => *span,
            Expr::Previous(_, span) => *span,
            Expr::StructLiteral { span, .. } => *span,
            Expr::Binary { span, .. } => *span,
            Expr::Pipe { span, .. } => *span,
            Expr::Unary { span, .. } => *span,
            Expr::Call { span, .. } => *span,
            Expr::Member { span, .. } => *span,
            Expr::Shell { span, .. } => *span,
            Expr::Intrinsic { span, .. } => *span,
        }
    }
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Let { span, .. } => *span,
            Stmt::Destructure { span, .. } => *span,
            Stmt::Return(_, span) => *span,
            Stmt::Stop(_, span) => *span,
            Stmt::Skip(span) => *span,
            Stmt::If { span, .. } => *span,
            Stmt::Loop { span, .. } => *span,
            Stmt::Expr(_, span) => *span,
            Stmt::Match { span, .. } => *span,
            Stmt::When { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Shell,
    ShellResult,
    Void,
    Named(String),
    /// A parameterized type such as `Array<Int>` or `Result<User, DbError>`.
    Applied(String, Vec<Type>),
    /// A heterogeneous fixed-size product type such as `(Int, String)`.
    Tuple(Vec<Type>),
    /// A callable type such as `fn(Int, String) -> Bool`.
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}
