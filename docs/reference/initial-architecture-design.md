# pyrust-check - Initial Architecture Design

## Core Architectural Principles

1. **Correctness First**: Reduce false positives over catching every edge case
2. **Performance by Design**: Built in Rust for speed, designed for parallelism
3. **Modular Pipeline**: Clear separation of concerns (parse → resolve → check → report)
4. **Incremental Complexity**: Start simple, expand systematically
5. **Zero-Cost Abstractions**: Fast by default, no runtime overhead

---

## System Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                           │
│                 (main.rs - User Interface)                  │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│                   Type Checker Core                         │
│        (checker/ - Orchestration & Analysis)                │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  Expression  │  │  Statement   │  │  Constraint  │       │
│  │   Checker    │  │   Checker    │  │   Solver     │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────┬────────────────────┬───────────────────┬──────────┘
          │                    │                   │
┌─────────▼─────────┐ ┌────────▼────────┐ ┌────────▼────────┐
│   Parser Layer    │ │  Symbol Layer   │ │   Type Layer    │
│                   │ │                 │ │                 │
│ ┌───────────────┐ │ │ ┌─────────────┐ │ │ ┌─────────────┐ │
│ │  RustPython   │ │ │ │Symbol Table │ │ │ │ Primitives  │ │
│ │    Parser     │ │ │ │   Scopes    │ │ │ │  Functions  │ │
│ │   Wrapper     │ │ │ │  Resolver   │ │ │ │  (Future:   │ │
│ │               │ │ │ │  Built-ins  │ │ │ │  Generics)  │ │
│ └───────────────┘ │ │ └─────────────┘ │ │ └─────────────┘ │
└───────────────────┘ └─────────────────┘ └─────────────────┘
                              │
                    ┌─────────▼──────────┐
                    │ Diagnostics Layer  │
                    │                    │
                    │ ┌────────────────┐ │
                    │ │ Error Types    │ │
                    │ │ Reporter       │ │
                    │ │ Source Context │ │
                    │ └────────────────┘ │
                    └────────────────────┘
```

---

## Core Abstractions

### 1. Parser Layer

**Purpose**: Convert Python source code to AST using RustPython

```rust
// src/parser/parser_impl.rs
use rustpython_parser::{parse_program, ParseError};
use std::path::Path;

pub struct PythonParser;

impl PythonParser {
    /// Parse a Python file into an AST
    pub fn parse_file(path: &Path) -> Result<rustpython_parser::ast::Suite> {
        let source = std::fs::read_to_string(path)?;
        Self::parse_source(&source, path)
    }

    /// Parse Python source code
    pub fn parse_source(
        source: &str,
        path: &Path,
    ) -> Result<rustpython_parser::ast::Suite> {
        parse_program(source, path.to_str().unwrap_or("<unknown>"))
            .map_err(|e| Self::convert_parse_error(e, path))
    }

    fn convert_parse_error(error: ParseError, path: &Path) -> PyRustError {
        PyRustError::ParseError {
            location: SourceLocation {
                file: path.to_path_buf(),
                line: error.location.row(),
                column: error.location.column(),
            },
            message: error.error.to_string(),
        }
    }
}
```

**Key Design Decisions:**
- Leverage RustPython's battle-tested parser (don't reinvent the wheel)
- Thin wrapper layer for error conversion
- Keep original AST structure for MVP, simplify later if needed

---

### 2. Symbol Layer

**Purpose**: Track all names, scopes, and their types

#### Symbol Representation

```rust
// src/symbols/symbol.rs

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub typ: Option<Type>,  // Type if known
    pub span: SourceSpan,   // Where defined
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
    // Future: Class, Method, Module
}
```

#### Scope Management

```rust
// src/symbols/table.rs

pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: ScopeId,
}

impl SymbolTable {
    /// Create new symbol table with module scope
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        scopes.push(Scope::new(ScopeKind::Module, None));
        Self {
            scopes,
            current_scope: ScopeId(0),
        }
    }

    /// Enter a new scope (function, class, etc.)
    pub fn enter_scope(&mut self, kind: ScopeKind) -> ScopeId {
        let parent = self.current_scope;
        let scope_id = ScopeId(self.scopes.len());
        
        let scope = Scope::new(kind, Some(parent));
        self.scopes.push(scope);
        self.scopes[parent.0].children.push(scope_id);
        
        self.current_scope = scope_id;
        scope_id
    }

    /// Look up a name using LEGB rules
    /// (Local, Enclosing, Global, Built-in)
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.lookup_in_scope(name, self.current_scope)
    }

    fn lookup_in_scope(&self, name: &str, scope_id: ScopeId) -> Option<&Symbol> {
        let scope = &self.scopes[scope_id.0];
        
        // Look in current scope first
        if let Some(symbol) = scope.lookup(name) {
            return Some(symbol);
        }
        
        // Recurse to parent (LEGB)
        if let Some(parent_id) = scope.parent {
            return self.lookup_in_scope(name, parent_id);
        }
        
        None
    }
}
```

**Key Design Decisions:**
- Tree structure for scopes (parent/child relationships)
- LEGB (Local, Enclosing, Global, Built-in) resolution order
- Separate scope for each function
- Built-ins in special root scope

---

### 3. Type Layer

**Purpose**: Represent Python's type system

#### Type Representation

```rust
// src/types/primitives.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Primitive types
    Int,
    Str,
    Bool,
    Float,
    None,
    
    /// Function type
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    
    /// Type not yet inferred
    Unknown,
    
    /// typing.Any - compatible with everything
    Any,
    
    // Future:
    // Class { name: String, fields: HashMap<String, Type> },
    // Generic { base: Box<Type>, params: Vec<Type> },
    // Union { types: Vec<Type> },
    // Protocol { name: String, methods: Vec<Method> },
}

impl Type {
    /// Check if two types are compatible
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            // Exact match
            (a, b) if a == b => true,
            
            // Any is compatible with everything
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Unknown during inference
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            
            // Function compatibility (future: covariance/contravariance)
            (Type::Function { .. }, Type::Function { .. }) => {
                // Simplified for MVP
                true
            }
            
            _ => false,
        }
    }

    /// Parse type from annotation name
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
```

**Type System Phases:**

**MVP (Current):**
- Primitives: int, str, bool, float, None
- Function signatures
- Type annotations only (no inference without annotations)

**Phase 2 (Classes):**
- Class types with attributes
- Inheritance tracking
- Method signatures

**Phase 3 (Advanced):**
- Generics: List[T], Dict[K, V]
- Unions: int | str
- Protocols: Structural typing

**Phase 4 (Complex):**
- Constraint-based inference
- Variance (covariance/contravariance)
- Advanced MRO handling

---

### 4. Type Checker Core

**Purpose**: Perform type analysis and error detection

#### Check Context

```rust
// src/checker/context.rs

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
}
```

#### Type Checking Algorithm

```rust
// src/checker/checker_impl.rs

pub struct TypeChecker;

impl TypeChecker {
    /// Main entry point: check a file
    pub fn check_file(path: &Path) -> Result<CheckContext> {
        // 1. Parse
        let ast = PythonParser::parse_file(path)?;
        
        // 2. Initialize context
        let mut ctx = CheckContext::new(path.to_path_buf());
        
        // 3. Build symbol table and check types
        Self::check_module(&ast, &mut ctx);
        
        // 4. Return results
        Ok(ctx)
    }

    /// Check a module (list of statements)
    fn check_module(stmts: &[ast::Stmt], ctx: &mut CheckContext) {
        for stmt in stmts {
            Self::check_stmt(stmt, ctx);
        }
    }

    /// Check a single statement
    fn check_stmt(stmt: &ast::Stmt, ctx: &mut CheckContext) {
        match stmt {
            ast::Stmt::FunctionDef(func) => {
                Self::check_function_def(func, ctx);
            }
            ast::Stmt::AnnAssign(ann_assign) => {
                // x: int = 5
                Self::check_ann_assign(ann_assign, ctx);
            }
            ast::Stmt::Assign(assign) => {
                // x = 5 (infer type from value)
                Self::check_assign(assign, ctx);
            }
            // ... other statement types
            _ => {}
        }
    }

    /// Infer type of an expression
    fn infer_expr_type(expr: &ast::Expr, ctx: &CheckContext) -> Type {
        match expr {
            ast::Expr::Constant(constant) => {
                // 5 -> int, "hello" -> str, etc.
                Self::infer_constant_type(&constant.value)
            }
            ast::Expr::Name(name) => {
                // Look up variable in symbol table
                ctx.symbol_table.lookup(&name.id)
                    .and_then(|sym| sym.typ.clone())
                    .unwrap_or(Type::Unknown)
            }
            ast::Expr::BinOp(binop) => {
                // a + b: infer types of both sides
                let left = Self::infer_expr_type(&binop.left, ctx);
                let right = Self::infer_expr_type(&binop.right, ctx);
                
                // Simple inference: int + int -> int
                if left == Type::Int && right == Type::Int {
                    Type::Int
                } else {
                    Type::Unknown
                }
            }
            ast::Expr::Call(call) => {
                // Function call: return type of function
                Self::infer_call_return_type(call, ctx)
            }
            _ => Type::Unknown,
        }
    }
}
```

**Type Checking Strategy:**

1. **Two-Pass Approach:**
   - Pass 1: Collect all definitions (functions, variables)
   - Pass 2: Resolve types and check compatibility

2. **For MVP (Single Pass):**
   - Process statements in order
   - Build symbol table as we go
   - Check types immediately when we have annotations
   - Forward references handled in post-MVP

3. **Error Collection:**
   - Don't stop at first error
   - Collect all errors
   - Report at end

---

### 5. Diagnostics Layer

**Purpose**: Beautiful, actionable error messages

#### Error Types

```rust
// src/diagnostics/error.rs

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
```

#### Error Reporter

```rust
// src/diagnostics/reporter.rs

pub struct ErrorReporter;

impl ErrorReporter {
    pub fn report_error(error: &PyRustError) {
        match error {
            PyRustError::TypeError { location, message } => {
                println!("\n{}: {}", 
                    "error".red().bold(), 
                    "Type mismatch".bold()
                );
                println!("  {} {}:{}:{}", 
                    "-->".blue().bold(),
                    location.file.display(),
                    location.line,
                    location.column
                );
                
                // Show source context
                Self::print_source_context(location);
                
                println!("  {}", message);
            }
            // ... other error types
        }
    }

    fn print_source_context(location: &SourceLocation) {
        if let Ok(source) = std::fs::read_to_string(&location.file) {
            let lines: Vec<&str> = source.lines().collect();
            if location.line > 0 && location.line <= lines.len() {
                let line_idx = location.line - 1;
                
                println!("   {}", "|".blue());
                println!(" {} {} {}", 
                    format!("{:>3}", location.line).blue(),
                    "|".blue(),
                    lines[line_idx]
                );
                
                // Print error pointer
                let spaces = " ".repeat(location.column + 6);
                println!("   {} {}{}", 
                    "|".blue(),
                    spaces,
                    "^".repeat(3).red().bold()
                );
            }
        }
    }
}
```

**Error Message Example:**
```
error: Type mismatch
  --> test.py:5:10
   |
 5 | x: int = "hello"
   |          ^^^^^^^ expected `int`, found `str`
```

---

### 6. CLI Layer

**Purpose**: User-friendly command-line interface

```rust
// src/main.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pyrust-check")]
#[command(version, about = "Fast Python type checker", long_about = None)]
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
    Check { path: PathBuf },
    
    /// Parse and display AST (debug)
    Parse { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Check { path }) | None => {
            let path = path.or(cli.path).expect("Path required");
            check_command(&path);
        }
        Some(Commands::Parse { path }) => {
            parse_command(&path);
        }
    }
}

fn check_command(path: &PathBuf) {
    use pyrust_check::checker::TypeChecker;
    use pyrust_check::diagnostics::reporter::ErrorReporter;
    
    match TypeChecker::check_file(path) {
        Ok(ctx) => {
            if ctx.has_errors() {
                ErrorReporter::report_errors(&ctx.errors);
                println!("\n✗ {} error(s) found", ctx.errors.len());
                std::process::exit(1);
            } else {
                println!("✓ No type errors found");
            }
        }
        Err(e) => {
            ErrorReporter::report_error(&e);
            std::process::exit(1);
        }
    }
}
```

---

## Usage Examples

### 1. Basic Usage (CLI)

```bash
# Check a single file
pyrust-check myfile.py

# Check with explicit command
pyrust-check check myfile.py

# Parse only (debug)
pyrust-check parse myfile.py
```

### 2. Python Code Example

**Input: `example.py`**
```python
def add(a: int, b: int) -> int:
    return a + b

x: int = 5
y: str = "hello"
z: int = add(x, 10)      # OK
w: str = add(x, 10)      # Error: type mismatch
```

**Output:**
```
Checking: example.py

error: Type mismatch
  --> example.py:8:10
   |
 8 | w: str = add(x, 10)
   |          ^^^^^^^^^^ expected `str`, found `int`

✗ 1 error(s) found
```

### 3. Library Usage (Future)

```rust
use pyrust_check::TypeChecker;

fn main() {
    let path = Path::new("myfile.py");
    let result = TypeChecker::check_file(path);
    
    match result {
        Ok(ctx) => {
            if ctx.has_errors() {
                for error in &ctx.errors {
                    println!("{}", error);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

---

## Data Flow

### Type Checking Pipeline

```
1. SOURCE CODE
   ↓
2. PARSE (RustPython)
   → Result<AST, ParseError>
   ↓
3. SYMBOL RESOLUTION
   → Build symbol table
   → Track scopes
   → Add built-ins
   ↓
4. TYPE CHECKING
   → Check statements
   → Infer expression types
   → Validate compatibility
   → Collect errors
   ↓
5. ERROR REPORTING
   → Format errors
   → Show source context
   → Display to user
```

### Information Flow

```
┌──────────────┐
│ Source File  │
└──────┬───────┘
       │
       ▼
┌──────────────┐     ┌─────────────┐
│     AST      │────→│Symbol Table │
└──────┬───────┘     └──────┬──────┘
       │                    │
       │  ┌─────────────────┘
       │  │
       ▼  ▼
┌──────────────┐     ┌─────────────┐
│Type Checker  │────→│   Errors    │
└──────────────┘     └──────┬──────┘
                            │
                            ▼
                     ┌─────────────┐
                     │  Reporter   │
                     └──────┬──────┘
                            │
                            ▼
                         Output
```

---

## Key Architectural Benefits

### ✅ Modularity
- Each layer has clear responsibilities
- Easy to test components in isolation
- Can swap implementations (e.g., different parsers)

### ✅ Performance
- Rust's zero-cost abstractions
- No GC pauses
- Ready for parallelization (Phase 5)
- Efficient memory usage

### ✅ Correctness
- Strong typing throughout
- Exhaustive pattern matching
- Compiler-enforced error handling
- No null pointer exceptions

### ✅ Maintainability
- Clear architecture documented from day 1
- Consistent error handling
- Separation of concerns
- Easy to extend

### ✅ Testability
- Pure functions for core logic
- Dependency injection for context
- Mock-friendly interfaces
- Integration test framework

---

## Design Patterns

### 1. Visitor Pattern (Implicit)
Type checker visits each AST node and processes it.

### 2. Context Object
`CheckContext` carries state through the checking process.

### 3. Registry Pattern (Future)
Type registry, symbol registry for extensibility.

### 4. Builder Pattern
Symbol table builds incrementally as we traverse AST.

### 5. Error Accumulation
Collect all errors instead of failing fast.

---

## Performance Considerations

### MVP (Current)
- **Goal**: <100ms for 100-line files
- **Strategy**: Keep it simple, no premature optimization

### Phase 5 (Performance)
- **Parallel file processing**: Use Rayon for data parallelism
- **String interning**: Reduce memory for repeated identifiers
- **Arena allocation**: Reduce allocation overhead for AST
- **Caching**: Cache parsed ASTs and type results
- **Incremental checking**: Only re-check changed files

### Benchmarking Strategy
```rust
// benches/parser_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parsing(c: &mut Criterion) {
    c.bench_function("parse_100_lines", |b| {
        b.iter(|| {
            // Benchmark code
        })
    });
}
```

---

## Future Architecture Enhancements

### Phase 2: Classes
```rust
pub enum Type {
    // ... existing types
    Class {
        name: String,
        bases: Vec<Type>,  // Parent classes
        methods: HashMap<String, Type>,
        attributes: HashMap<String, Type>,
    },
}
```

### Phase 3: Generics
```rust
pub enum Type {
    // ... existing types
    Generic {
        base: Box<Type>,      // List, Dict, etc.
        params: Vec<Type>,    // [int], [str, int], etc.
    },
}
```

### Phase 4: Constraints
```rust
pub struct Constraint {
    kind: ConstraintKind,
    left: Type,
    right: Type,
}

pub enum ConstraintKind {
    Equality,      // T1 = T2
    Subtype,       // T1 <: T2
    HasAttribute,  // T has attr
}
```

### Phase 5: LSP
```rust
pub struct LspServer {
    checker: TypeChecker,
    file_cache: HashMap<PathBuf, ParsedFile>,
    // Real-time type checking
}
```

---

## Testing Architecture

### Unit Tests
- Test each module independently
- Mock dependencies where needed
- Property-based testing for type system

### Integration Tests
- End-to-end file checking
- Real Python files as fixtures
- Compare with expected errors

### Performance Tests
- Benchmark suite with Criterion
- Track performance over time
- Prevent regressions

### Test Organization
```
tests/
├── integration/
│   ├── type_check_tests.rs
│   └── parser_tests.rs
├── fixtures/
│   ├── basic.py
│   ├── functions.py
│   └── errors.py
└── benches/
    └── parser_bench.rs
```

---

## Comparison with Existing Tools

### pyrust-check vs mypy

| Feature | pyrust-check (MVP) | mypy |
|---------|-------------------|------|
| Language | Rust | Python |
| Speed | Fast (goal: 3-5x) | Baseline |
| Inheritance | Simple | Complex |
| False Positives | Lower (goal) | Higher on complex OOP |
| Generics | Future | Full support |
| Protocols | Future | Full support |

**pyrust-check Advantages:**
- Performance (Rust's speed)
- Better handling of complex inheritance (Phase 4 goal)
- Clearer error messages
- Modern codebase

**mypy Advantages:**
- Mature ecosystem
- Complete type system support
- Years of production usage
- Large community

---

## Conclusion

This architecture provides:
1. **Clear foundation** for MVP implementation
2. **Path to expansion** through defined phases
3. **Performance by design** with Rust
4. **Maintainable structure** with separation of concerns
5. **Testable components** at every layer

The key insight: Start simple (MVP), validate the approach, then systematically expand capabilities. Each phase builds on the previous one without major refactoring.