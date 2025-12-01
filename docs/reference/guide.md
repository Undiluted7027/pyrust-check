# pyrust-check - MVP Implementation Guide

> **Goal**: Build a working Python type checker in Rust in 2-3 weeks
>
> **Target**: A CLI tool that parses Python files and performs basic type checking with clear error messages

---

## Table of Contents

1. [Week 1: Foundation & Parser](#week-1-foundation--parser)
2. [Week 2: Symbol Table & Types](#week-2-symbol-table--types)
3. [Week 3: Type Checking & Polish](#week-3-type-checking--polish)

---

## Week 1: Foundation & Parser

### Day 1-2: Project Setup & Dependencies

#### 1.1 Initialize Cargo Project

```bash
# Initialize project (if not done)
cargo init --name pyrust-check

# Verify setup
cargo build
cargo test
```

#### 1.2 Configure `Cargo.toml`

```toml
[package]
name = "pyrust-check"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
description = "A fast Python type checker built in Rust"
repository = "https://github.com/Undiluted7027/pyrust-check"
keywords = ["python", "type-checker", "static-analysis", "rust"]
categories = ["development-tools", "command-line-utilities"]

[dependencies]
rustpython-parser = "0.3"
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
colored = "2.1"

[dev-dependencies]
criterion = "0.5"
pretty_assertions = "1.4"

[[bench]]
name = "parser_bench"
harness = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

#### 1.3 Create Project Structure

```bash
mkdir -p src/{parser,symbols,types,checker,diagnostics,utils}
mkdir -p tests/{integration,fixtures}
mkdir -p benches
```

#### 1.4 Configure Development Tools

**Create `rustfmt.toml`:**
```toml
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
edition = "2021"
```

**Create `clippy.toml`:**
```toml
# Clippy configuration
warn-on-all-wildcard-imports = true
```

**Create `.gitignore` (if not exists):**
```
/target
Cargo.lock
**/*.rs.bk
*.pdb
.DS_Store
.idea/
*.swp
*.swo
```

#### 1.5 Create Basic Error Types

**Create `src/diagnostics/error.rs`:**
```rust
use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum PyRustError {
    #[error("Parse error at {location}: {message}")]
    ParseError {
        location: SourceLocation,
        message: String,
    },

    #[error("Type error at {location}: {message}")]
    TypeError {
        location: SourceLocation,
        message: String,
    },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Undefined name '{name}' at {location}")]
    UndefinedName {
        name: String,
        location: SourceLocation,
    },
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

pub type Result<T> = std::result::Result<T, PyRustError>;
```

**Create `src/diagnostics/mod.rs`:**
```rust
pub mod error;
pub mod reporter;

pub use error::{PyRustError, Result, SourceLocation};
```

#### 1.6 Create Source Span Type

**Create `src/utils/span.rs`:**
```rust
use std::path::PathBuf;

/// Represents a span of source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpan {
    pub file: PathBuf,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl SourceSpan {
    pub fn new(
        file: PathBuf,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            file,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    pub fn unknown() -> Self {
        Self {
            file: PathBuf::from("<unknown>"),
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
        }
    }
}

impl std::fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.file.display(),
            self.start_line,
            self.start_col
        )
    }
}
```

**Create `src/utils/mod.rs`:**
```rust
pub mod span;

pub use span::SourceSpan;
```

#### 1.7 Create Basic CLI Structure

**Create `src/main.rs`:**
```rust
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pyrust-check")]
#[command(version, about = "A fast Python type checker built in Rust", long_about = None)]
struct Cli {
    /// Python file or directory to check
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check Python files for type errors
    Check {
        /// Path to check
        path: PathBuf,
    },
    /// Parse and display AST (debug)
    Parse {
        /// Path to parse
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Check { path }) => {
            println!("Checking: {}", path.display());
            // TODO: Implement type checking
        }
        Some(Commands::Parse { path }) => {
            println!("Parsing: {}", path.display());
            // TODO: Implement parsing
        }
        None => {
            if let Some(path) = cli.path {
                println!("Checking: {}", path.display());
                // TODO: Implement type checking
            } else {
                eprintln!("Error: Please provide a path to check");
                std::process::exit(1);
            }
        }
    }
}
```

**Create `src/lib.rs`:**
```rust
pub mod diagnostics;
pub mod parser;
pub mod symbols;
pub mod types;
pub mod checker;
pub mod utils;

pub use diagnostics::{PyRustError, Result};
```

#### 1.8 Verify Setup

```bash
# Build project
cargo build

# Run clippy
cargo clippy

# Format code
cargo fmt

# Run tests (empty for now)
cargo test

# Try CLI
cargo run -- --help
```

**Expected output:**
```
A fast Python type checker built in Rust

Usage: pyrust-check [PATH] [COMMAND]

Commands:
  check  Check Python files for type errors
  parse  Parse and display AST (debug)
  help   Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  Python file or directory to check
```

---

### Day 3-5: Parser & AST

#### 1.9 Create Parser Module

**Create `src/parser/mod.rs`:**
```rust
pub mod ast;
mod parser_impl;

pub use parser_impl::PythonParser;
pub use ast::*;
```

**Create `src/parser/parser_impl.rs`:**
```rust
use rustpython_parser::{parse_program, ParseError};
use std::fs;
use std::path::Path;
use crate::diagnostics::{PyRustError, Result, SourceLocation};

pub struct PythonParser;

impl PythonParser {
    pub fn parse_file(path: &Path) -> Result<rustpython_parser::ast::Suite> {
        let source = fs::read_to_string(path)?;
        Self::parse_source(&source, path)
    }

    pub fn parse_source(
        source: &str,
        path: &Path,
    ) -> Result<rustpython_parser::ast::Suite> {
        parse_program(source, path.to_str().unwrap_or("<unknown>"))
            .map_err(|e| Self::convert_parse_error(e, path))
    }

    fn convert_parse_error(error: ParseError, path: &Path) -> PyRustError {
        let location = SourceLocation {
            file: path.to_path_buf(),
            line: error.location.row(),
            column: error.location.column(),
        };
        PyRustError::ParseError {
            location,
            message: error.error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_simple_function() {
        let source = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
        let result = PythonParser::parse_source(source, Path::new("test.py"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_error() {
        let source = "def invalid syntax";
        let result = PythonParser::parse_source(source, Path::new("test.py"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_type_annotations() {
        let source = r#"
x: int = 5
y: str = "hello"

def greet(name: str) -> str:
    return f"Hello, {name}"
"#;
        let result = PythonParser::parse_source(source, Path::new("test.py"));
        assert!(result.is_ok());
    }
}
```

#### 1.10 Create AST Wrapper Types

**Create `src/parser/ast.rs`:**
```rust
use rustpython_parser::ast;
use crate::utils::SourceSpan;

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
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub annotation: Option<TypeAnnotation>,
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
    None,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
}

// Conversion utilities from RustPython AST to our simplified AST
// We'll implement this as needed
```

#### 1.11 Update Main to Use Parser

**Update `src/main.rs`:**
```rust
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use pyrust_check::parser::PythonParser;
use colored::*;

#[derive(Parser)]
#[command(name = "pyrust-check")]
#[command(version, about = "A fast Python type checker built in Rust", long_about = None)]
struct Cli {
    /// Python file or directory to check
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check Python files for type errors
    Check {
        /// Path to check
        path: PathBuf,
    },
    /// Parse and display AST (debug)
    Parse {
        /// Path to parse
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Parse { path }) => {
            parse_command(&path);
        }
        Some(Commands::Check { path }) | None => {
            let path = path.or(cli.path).unwrap_or_else(|| {
                eprintln!("{}", "Error: Please provide a path to check".red());
                std::process::exit(1);
            });
            check_command(&path);
        }
    }
}

fn parse_command(path: &PathBuf) {
    println!("{} {}", "Parsing:".blue(), path.display());
    
    match PythonParser::parse_file(path) {
        Ok(ast) => {
            println!("{}", "✓ Parsed successfully".green());
            println!("\nAST has {} statements", ast.len());
            // TODO: Print AST details
        }
        Err(e) => {
            eprintln!("{} {}", "✗ Parse error:".red(), e);
            std::process::exit(1);
        }
    }
}

fn check_command(path: &PathBuf) {
    println!("{} {}", "Checking:".blue(), path.display());
    
    match PythonParser::parse_file(path) {
        Ok(_ast) => {
            println!("{}", "✓ Parsed successfully".green());
            // TODO: Perform type checking
            println!("{}", "Type checking not yet implemented".yellow());
        }
        Err(e) => {
            eprintln!("{} {}", "✗ Parse error:".red(), e);
            std::process::exit(1);
        }
    }
}
```

#### 1.12 Test Parser

Create test fixture:

**Create `tests/fixtures/basic.py`:**
```python
def add(a: int, b: int) -> int:
    return a + b

x: int = 5
y: str = "hello"
z: int = add(x, 10)
```

**Test it:**
```bash
cargo run -- parse tests/fixtures/basic.py
```

---

## Week 2: Symbol Table & Types

### Day 6-7: Symbol Table Structure

#### 2.1 Create Symbol Types

**Create `src/symbols/symbol.rs`:**
```rust
use crate::types::Type;
use crate::utils::SourceSpan;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub typ: Option<Type>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
}

#[derive(Debug)]
pub struct Scope {
    pub kind: ScopeKind,
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<ScopeId>,
    pub children: Vec<ScopeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Module,
    Function,
}

impl Scope {
    pub fn new(kind: ScopeKind, parent: Option<ScopeId>) -> Self {
        Self {
            kind,
            symbols: HashMap::new(),
            parent,
            children: Vec::new(),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}
```

**Create `src/symbols/table.rs`:**
```rust
use super::symbol::{Scope, ScopeId, ScopeKind, Symbol};
use std::collections::HashMap;

pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: ScopeId,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        let module_scope = Scope::new(ScopeKind::Module, None);
        scopes.push(module_scope);
        
        Self {
            scopes,
            current_scope: ScopeId(0),
        }
    }

    pub fn current_scope_id(&self) -> ScopeId {
        self.current_scope
    }

    pub fn enter_scope(&mut self, kind: ScopeKind) -> ScopeId {
        let parent = self.current_scope;
        let scope_id = ScopeId(self.scopes.len());
        
        let scope = Scope::new(kind, Some(parent));
        self.scopes.push(scope);
        
        // Add as child to parent
        self.scopes[parent.0].children.push(scope_id);
        
        self.current_scope = scope_id;
        scope_id
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope.0].parent {
            self.current_scope = parent;
        }
    }

    pub fn insert_symbol(&mut self, symbol: Symbol) {
        self.scopes[self.current_scope.0].insert(symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.lookup_in_scope(name, self.current_scope)
    }

    pub fn lookup_in_scope(&self, name: &str, scope_id: ScopeId) -> Option<&Symbol> {
        let scope = &self.scopes[scope_id.0];
        
        // Look in current scope
        if let Some(symbol) = scope.lookup(name) {
            return Some(symbol);
        }
        
        // Look in parent scope (LEGB)
        if let Some(parent_id) = scope.parent {
            return self.lookup_in_scope(name, parent_id);
        }
        
        None
    }

    pub fn get_scope(&self, scope_id: ScopeId) -> &Scope {
        &self.scopes[scope_id.0]
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
```

**Create `src/symbols/mod.rs`:**
```rust
pub mod symbol;
pub mod table;
pub mod builtins;

pub use symbol::{Symbol, SymbolKind, Scope, ScopeId, ScopeKind};
pub use table::SymbolTable;
pub use builtins::create_builtin_scope;
```

### Day 8-9: Type System Foundation

#### 2.2 Create Type Representation

**Create `src/types/mod.rs`:**
```rust
pub mod primitives;

pub use primitives::Type;
```

**Create `src/types/primitives.rs`:**
```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// int
    Int,
    /// str
    Str,
    /// bool
    Bool,
    /// float
    Float,
    /// None type
    None,
    /// Function type
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    /// Type not yet inferred
    Unknown,
    /// typing.Any
    Any,
}

impl Type {
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            // Exact match
            (a, b) if a == b => true,
            
            // Any is compatible with everything
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Unknown is compatible during inference
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            
            _ => false,
        }
    }

    pub fn from_annotation(name: &str) -> Option<Self> {
        match name {
            "int" => Some(Type::Int),
            "str" => Some(Type::Str),
            "bool" => Some(Type::Bool),
            "float" => Some(Type::Float),
            "None" => Some(Type::None),
            _ => None,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Str => write!(f, "str"),
            Type::Bool => write!(f, "bool"),
            Type::Float => write!(f, "float"),
            Type::None => write!(f, "None"),
            Type::Function { params, return_type } => {
                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::Unknown => write!(f, "Unknown"),
            Type::Any => write!(f, "Any"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_equality() {
        assert_eq!(Type::Int, Type::Int);
        assert_ne!(Type::Int, Type::Str);
    }

    #[test]
    fn test_compatibility() {
        assert!(Type::Int.is_compatible_with(&Type::Int));
        assert!(Type::Int.is_compatible_with(&Type::Any));
        assert!(!Type::Int.is_compatible_with(&Type::Str));
    }

    #[test]
    fn test_from_annotation() {
        assert_eq!(Type::from_annotation("int"), Some(Type::Int));
        assert_eq!(Type::from_annotation("str"), Some(Type::Str));
        assert_eq!(Type::from_annotation("unknown"), None);
    }
}
```

### Day 10: Built-in Types

#### 2.3 Create Built-in Scope

**Create `src/symbols/builtins.rs`:**
```rust
use super::{Symbol, SymbolKind, Scope, ScopeKind};
use crate::types::Type;
use crate::utils::SourceSpan;

pub fn create_builtin_scope() -> Scope {
    let mut scope = Scope::new(ScopeKind::Module, None);
    
    // Built-in types
    let builtin_types = vec![
        ("int", Type::Int),
        ("str", Type::Str),
        ("bool", Type::Bool),
        ("float", Type::Float),
    ];
    
    for (name, typ) in builtin_types {
        scope.insert(Symbol {
            name: name.to_string(),
            kind: SymbolKind::Variable,
            typ: Some(typ),
            span: SourceSpan::unknown(),
        });
    }
    
    // Built-in functions
    let print_fn = Type::Function {
        params: vec![Type::Any], // Simplified: print takes anything
        return_type: Box::new(Type::None),
    };
    
    scope.insert(Symbol {
        name: "print".to_string(),
        kind: SymbolKind::Function,
        typ: Some(print_fn),
        span: SourceSpan::unknown(),
    });
    
    scope
}
```

---

## Week 3: Type Checking & Polish

### Day 11-13: Basic Type Checker

#### 3.1 Create Type Checker Context

**Create `src/checker/context.rs`:**
```rust
use crate::symbols::SymbolTable;
use crate::diagnostics::PyRustError;
use std::path::PathBuf;

pub struct CheckContext {
    pub symbol_table: SymbolTable,
    pub errors: Vec<PyRustError>,
    pub file: PathBuf,
}

impl CheckContext {
    pub fn new(file: PathBuf) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            file,
        }
    }

    pub fn add_error(&mut self, error: PyRustError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
```

**Create `src/checker/mod.rs`:**
```rust
pub mod context;
pub mod checker_impl;

pub use context::CheckContext;
pub use checker_impl::TypeChecker;
```

#### 3.2 Implement Basic Type Checker

**Create `src/checker/checker_impl.rs`:**
```rust
use rustpython_parser::ast;
use crate::diagnostics::{PyRustError, SourceLocation, Result};
use crate::types::Type;
use crate::symbols::{Symbol, SymbolKind, ScopeKind};
use super::context::CheckContext;
use std::path::Path;

pub struct TypeChecker;

impl TypeChecker {
    pub fn check_file(path: &Path) -> Result<CheckContext> {
        let ast = crate::parser::PythonParser::parse_file(path)?;
        let mut ctx = CheckContext::new(path.to_path_buf());
        
        Self::check_module(&ast, &mut ctx);
        
        Ok(ctx)
    }

    fn check_module(stmts: &[ast::Stmt], ctx: &mut CheckContext) {
        for stmt in stmts {
            Self::check_stmt(stmt, ctx);
        }
    }

    fn check_stmt(stmt: &ast::Stmt, ctx: &mut CheckContext) {
        match stmt {
            ast::Stmt::FunctionDef(func) => {
                Self::check_function_def(func, ctx);
            }
            ast::Stmt::AnnAssign(ann_assign) => {
                Self::check_ann_assign(ann_assign, ctx);
            }
            ast::Stmt::Assign(assign) => {
                Self::check_assign(assign, ctx);
            }
            ast::Stmt::Expr(expr) => {
                Self::infer_expr_type(&expr.value, ctx);
            }
            _ => {
                // Skip other statement types for MVP
            }
        }
    }

    fn check_function_def(func: &ast::StmtFunctionDef, ctx: &mut CheckContext) {
        // Store function in symbol table
        let return_type = func.returns.as_ref()
            .and_then(|r| Self::resolve_type_annotation(r, ctx))
            .unwrap_or(Type::Unknown);
        
        let param_types: Vec<Type> = func.args.args.iter()
            .map(|arg| {
                arg.annotation.as_ref()
                    .and_then(|ann| Self::resolve_type_annotation(ann, ctx))
                    .unwrap_or(Type::Unknown)
            })
            .collect();
        
        let func_type = Type::Function {
            params: param_types,
            return_type: Box::new(return_type),
        };
        
        let symbol = Symbol {
            name: func.name.to_string(),
            kind: SymbolKind::Function,
            typ: Some(func_type),
            span: crate::utils::SourceSpan::unknown(), // TODO: Extract from AST
        };
        
        ctx.symbol_table.insert_symbol(symbol);
        
        // Enter function scope
        ctx.symbol_table.enter_scope(ScopeKind::Function);
        
        // Add parameters to scope
        for (arg, param_type) in func.args.args.iter().zip(param_types) {
            let param_symbol = Symbol {
                name: arg.arg.to_string(),
                kind: SymbolKind::Parameter,
                typ: Some(param_type),
                span: crate::utils::SourceSpan::unknown(),
            };
            ctx.symbol_table.insert_symbol(param_symbol);
        }
        
        // Check function body
        Self::check_module(&func.body, ctx);
        
        // Exit function scope
        ctx.symbol_table.exit_scope();
    }

    fn check_ann_assign(ann_assign: &ast::StmtAnnAssign, ctx: &mut CheckContext) {
        // Get variable name
        let var_name = match &*ann_assign.target {
            ast::Expr::Name(name) => name.id.to_string(),
            _ => return, // Skip complex targets for MVP
        };
        
        // Resolve annotation type
        let annotated_type = Self::resolve_type_annotation(&ann_assign.annotation, ctx)
            .unwrap_or(Type::Unknown);
        
        // If there's a value, check it matches the annotation
        if let Some(value) = &ann_assign.value {
            let value_type = Self::infer_expr_type(value, ctx);
            
            if !value_type.is_compatible_with(&annotated_type) {
                let error = PyRustError::TypeError {
                    location: SourceLocation {
                        file: ctx.file.clone(),
                        line: value.start_location().unwrap().row(),
                        column: value.start_location().unwrap().column(),
                    },
                    message: format!(
                        "Type mismatch: expected `{}`, found `{}`",
                        annotated_type, value_type
                    ),
                };
                ctx.add_error(error);
            }
        }
        
        // Add to symbol table
        let symbol = Symbol {
            name: var_name,
            kind: SymbolKind::Variable,
            typ: Some(annotated_type),
            span: crate::utils::SourceSpan::unknown(),
        };
        ctx.symbol_table.insert_symbol(symbol);
    }

    fn check_assign(assign: &ast::StmtAssign, ctx: &mut CheckContext) {
        // Infer type from value
        let value_type = Self::infer_expr_type(&assign.value, ctx);
        
        // Add variables to symbol table
        for target in &assign.targets {
            if let ast::Expr::Name(name) = target {
                let symbol = Symbol {
                    name: name.id.to_string(),
                    kind: SymbolKind::Variable,
                    typ: Some(value_type.clone()),
                    span: crate::utils::SourceSpan::unknown(),
                };
                ctx.symbol_table.insert_symbol(symbol);
            }
        }
    }

    fn infer_expr_type(expr: &ast::Expr, ctx: &CheckContext) -> Type {
        match expr {
            ast::Expr::Constant(constant) => {
                Self::infer_constant_type(&constant.value)
            }
            ast::Expr::Name(name) => {
                // Look up in symbol table
                ctx.symbol_table.lookup(&name.id)
                    .and_then(|sym| sym.typ.clone())
                    .unwrap_or(Type::Unknown)
            }
            ast::Expr::BinOp(binop) => {
                let left_type = Self::infer_expr_type(&binop.left, ctx);
                let right_type = Self::infer_expr_type(&binop.right, ctx);
                
                // Simple inference: int + int -> int, etc.
                if left_type == Type::Int && right_type == Type::Int {
                    Type::Int
                } else if left_type == Type::Str || right_type == Type::Str {
                    Type::Str
                } else {
                    Type::Unknown
                }
            }
            ast::Expr::Call(call) => {
                // Look up function type
                if let ast::Expr::Name(func_name) = &*call.func {
                    if let Some(symbol) = ctx.symbol_table.lookup(&func_name.id) {
                        if let Some(Type::Function { return_type, .. }) = &symbol.typ {
                            return *return_type.clone();
                        }
                    }
                }
                Type::Unknown
            }
            _ => Type::Unknown,
        }
    }

    fn infer_constant_type(constant: &ast::Constant) -> Type {
        match constant {
            ast::Constant::Int(_) => Type::Int,
            ast::Constant::Str(_) => Type::Str,
            ast::Constant::Bool(_) => Type::Bool,
            ast::Constant::Float(_) => Type::Float,
            ast::Constant::None => Type::None,
            _ => Type::Unknown,
        }
    }

    fn resolve_type_annotation(expr: &ast::Expr, _ctx: &CheckContext) -> Option<Type> {
        match expr {
            ast::Expr::Name(name) => Type::from_annotation(&name.id),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_simple_type_check() {
        let source = r#"
x: int = 5
y: str = "hello"
"#;
        let path = PathBuf::from("test.py");
        std::fs::write(&path, source).unwrap();
        
        let result = TypeChecker::check_file(&path);
        assert!(result.is_ok());
        
        std::fs::remove_file(&path).ok();
    }
}
```

### Day 14-16: Error Reporting

#### 3.3 Create Error Reporter

**Create `src/diagnostics/reporter.rs`:**
```rust
use super::PyRustError;
use colored::*;
use std::fs;

pub struct ErrorReporter;

impl ErrorReporter {
    pub fn report_errors(errors: &[PyRustError]) {
        for error in errors {
            Self::report_error(error);
        }
    }

    pub fn report_error(error: &PyRustError) {
        match error {
            PyRustError::TypeError { location, message } => {
                println!("\n{}: {}", "error".red().bold(), "Type mismatch".bold());
                println!("  {} {}:{}:{}", 
                    "-->".blue().bold(),
                    location.file.display(),
                    location.line,
                    location.column
                );
                
                // Try to show source context
                if let Ok(source) = fs::read_to_string(&location.file) {
                    Self::print_source_context(&source, location.line, location.column);
                }
                
                println!("  {}", message);
            }
            PyRustError::ParseError { location, message } => {
                println!("\n{}: {}", "error".red().bold(), "Parse error".bold());
                println!("  {} {}:{}:{}", 
                    "-->".blue().bold(),
                    location.file.display(),
                    location.line,
                    location.column
                );
                println!("  {}", message);
            }
            PyRustError::UndefinedName { name, location } => {
                println!("\n{}: {}", "error".red().bold(), "Undefined name".bold());
                println!("  {} {}:{}:{}", 
                    "-->".blue().bold(),
                    location.file.display(),
                    location.line,
                    location.column
                );
                println!("  Undefined name: {}", name.yellow());
            }
            _ => {
                println!("{}: {}", "error".red().bold(), error);
            }
        }
    }

    fn print_source_context(source: &str, line: usize, column: usize) {
        let lines: Vec<&str> = source.lines().collect();
        if line > 0 && line <= lines.len() {
            let line_idx = line - 1;
            
            // Print line number and source
            println!("   {}", "|".blue());
            println!(" {} {} {}", 
                format!("{:>3}", line).blue(),
                "|".blue(),
                lines[line_idx]
            );
            
            // Print error pointer
            let spaces = " ".repeat(column + 6); // Account for line number padding
            println!("   {} {}{}", 
                "|".blue(),
                spaces,
                "^".repeat(1).red().bold()
            );
        }
    }
}
```

#### 3.4 Update Main to Use Type Checker

**Update `src/main.rs` check_command:**
```rust
fn check_command(path: &PathBuf) {
    use pyrust_check::checker::TypeChecker;
    use pyrust_check::diagnostics::reporter::ErrorReporter;
    
    println!("{} {}", "Checking:".blue(), path.display());
    
    match TypeChecker::check_file(path) {
        Ok(ctx) => {
            if ctx.has_errors() {
                ErrorReporter::report_errors(&ctx.errors);
                println!("\n{} {} error(s) found", 
                    "✗".red().bold(),
                    ctx.errors.len()
                );
                std::process::exit(1);
            } else {
                println!("{} {}", "✓".green().bold(), "No type errors found".green());
            }
        }
        Err(e) => {
            ErrorReporter::report_error(&e);
            std::process::exit(1);
        }
    }
}
```

### Day 17-19: Testing & Polish

#### 3.5 Create Comprehensive Tests

**Create `tests/integration/type_check_tests.rs`:**
```rust
use std::fs;
use std::path::PathBuf;
use pyrust_check::checker::TypeChecker;

fn create_test_file(content: &str, name: &str) -> PathBuf {
    let path = PathBuf::from(format!("tests/fixtures/{}", name));
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn test_valid_type_annotations() {
    let source = r#"
x: int = 5
y: str = "hello"
z: bool = True
"#;
    let path = create_test_file(source, "test_valid.py");
    
    let result = TypeChecker::check_file(&path);
    assert!(result.is_ok());
    let ctx = result.unwrap();
    assert!(!ctx.has_errors());
    
    fs::remove_file(&path).ok();
}

#[test]
fn test_type_mismatch() {
    let source = r#"
x: int = "hello"
"#;
    let path = create_test_file(source, "test_mismatch.py");
    
    let result = TypeChecker::check_file(&path);
    assert!(result.is_ok());
    let ctx = result.unwrap();
    assert!(ctx.has_errors());
    assert_eq!(ctx.errors.len(), 1);
    
    fs::remove_file(&path).ok();
}

#[test]
fn test_function_type_check() {
    let source = r#"
def add(a: int, b: int) -> int:
    return a + b

x: int = 5
result: int = add(x, 10)
"#;
    let path = create_test_file(source, "test_function.py");
    
    let result = TypeChecker::check_file(&path);
    assert!(result.is_ok());
    let ctx = result.unwrap();
    assert!(!ctx.has_errors());
    
    fs::remove_file(&path).ok();
}
```

#### 3.6 Create README

**Create comprehensive README with examples and usage instructions.**

#### 3.7 Performance Benchmarks

**Create `benches/parser_bench.rs`:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pyrust_check::parser::PythonParser;
use std::path::Path;

fn benchmark_parsing(c: &mut Criterion) {
    let source = r#"
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

x: int = 10
result: int = fibonacci(x)
"#;
    
    c.bench_function("parse_small_file", |b| {
        b.iter(|| {
            PythonParser::parse_source(black_box(source), Path::new("bench.py"))
        })
    });
}

criterion_group!(benches, benchmark_parsing);
criterion_main!(benches);
```

---

## Testing Checklist

- [ ] All unit tests pass (`cargo test`)
- [ ] Integration tests cover main scenarios
- [ ] Parser handles syntax errors gracefully
- [ ] Type checker detects basic type mismatches
- [ ] Error messages are clear and helpful
- [ ] CLI works for single files
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)

## Pre-Release Checklist

- [ ] README is comprehensive with examples
- [ ] CONTRIBUTING.md exists
- [ ] All tests pass
- [ ] Documentation is complete
- [ ] Benchmarks show reasonable performance
- [ ] Examples in `tests/fixtures/` work
- [ ] Error messages are user-friendly
- [ ] Code coverage >70%

---

## Success Metrics

**Week 1**: Parser working, can parse 100+ line files  
**Week 2**: Symbol table complete, types represented  
**Week 3**: Type checking working, beautiful errors  

**MVP Success**: Can run `pyrust-check test.py` and catch type mismatches like `x: int = "hello"`

---

## Tips for Success

1. **Start Simple**: Get parsing working first before anything else
2. **Test Early**: Write tests as you implement each module
3. **Use RustPython**: Don't reinvent the wheel for parsing
4. **Beautiful Errors**: Invest time in error formatting
5. **Iterate**: Ship MVP, then add features

---

## Need Help?

During implementation, refer to:
- Initial architecture design document
- RustPython parser documentation: https://docs.rs/rustpython-parser/
- Rust Book: https://doc.rust-lang.org/book/
- Type system resources: PEP 484, PEP 526

Remember: **The goal is a working MVP in 2-3 weeks**, not perfection. Focus on the core loop: parse → check → report errors.