// AST node definitions
// Phase 1: AST definitions

use crate::utils::SourceSpan;
// use rustpython_parser::ast::{self, Stmt as RustPythonStmt, Expr as RustPythonExpr};

/// Simplified AST node types we care about for MVP
#[derive(Debug, Clone)]
pub enum Stmt {
    FunctionDef {
        name: String,
        args: Vec<Arg>,
        returns: Option<TypeAnnotation>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },
    AnnAssign {
        target: String,
        annotation: TypeAnnotation,
        value: Option<Expr>,
        span: SourceSpan,
    },
    Assign {
        targets: Vec<String>,
        value: Expr,
        span: SourceSpan,
    },
    Expr {
        value: Expr,
        span: SourceSpan,
    },
    // We'll add more as needed
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub annotation: Option<TypeAnnotation>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Name(String),
    // Will expand this later for generics, unions, etc.
}

#[derive(Debug, Clone)]
pub enum Expr {
    Name {
        id: String,
        span: SourceSpan,
    },
    Constant {
        value: Constant,
        span: SourceSpan,
    },
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
        span: SourceSpan,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
        span: SourceSpan,
    },
}

#[derive(Debug, Clone)]
pub enum Constant {
    Int(i64),
    Str(String),
    Bool(bool),
    Float(f64),
    None,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
}

// TODO: Implement conversion traits From<rustpython_ast::Stmt> for Stmt