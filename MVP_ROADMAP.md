# pyrust-check - MVP Roadmap

> **Goal**: Ship a working, functional type checker MVP in 2-3 weeks

> **Target**: Parse Python files and perform basic type checking with clear error messages

---

## 🎯 MVP Vision

Build a functional Python type checker that demonstrates:
1. ✅ **Parse Python Source**: Convert `.py` files to AST using RustPython parser
2. ✅ **Basic Symbol Resolution**: Track function and variable definitions in single files
3. ✅ **Simple Type Checking**: Check primitive types and function signatures
4. ✅ **Clear Error Reporting**: Show file location and error messages
5. ✅ **CLI Interface**: `pyrust-check <file.py>` just works

**MVP Success Metric**: A developer can run `pyrust-check myfile.py` and get clear type errors for basic type mismatches like `x: int = "hello"`.

---

## 📊 Progress Tracker

| Phase | Status | Completion |
|-------|--------|------------|
| **Phase 0: Foundation** | ✅ Complete | 100% |
| **Phase 1: Parser & AST** | 🏗️ In Progress | 0% |
| **Phase 2: Symbol Table** | ⏳ Pending | 0% |
| **Phase 3: Type System Foundation** | ⏳ Pending | 0% |
| **Phase 4: Basic Type Checking** | ⏳ Pending | 0% |
| **Phase 5: Error Reporting** | ⏳ Pending | 0% |
| **Phase 6: Testing & Polish** | ⏳ Pending | 0% |

**Overall Progress**: 14.28% (1/7 phases complete)

---

## Phase 0: Foundation ✅ COMPLETED

**Duration**: Week 0 (Days 1-2)

**Status**: ✅ Completed (100% complete)

### Goals
- Initialize Rust project structure
- Set up development tooling
- Configure dependencies
- Define core data structures

### Tasks

#### Day 1: Project Setup
- [x] Repository created with MIT license
- [x] Basic .gitignore configured
- [x] [Architectural design](./docs/reference/initial-architecture-design.md) documented
- [x] [Implementation plan](./docs/reference/guide.md) documented
- [x] MVP roadmap created (this document)
- [x] Initialize Cargo project
  ```bash
  cargo init --name pyrust-check
  ```
- [x] Create directory structure
  ```
  pyrust-check/
  ├── src/
  │   ├── main.rs
  │   ├── lib.rs
  │   ├── parser/
  │   ├── symbols/
  │   ├── types/
  │   ├── checker/
  │   ├── diagnostics/
  │   └── utils/
  ├── tests/
  └── benches/
  ```
- [x] Configure `rustfmt.toml`
- [x] Configure `clippy.toml`

#### Day 2: Dependencies & Core Types
- [x] **Add dependencies to `Cargo.toml`**
  - `rustpython-parser` - Python AST parsing
  - `clap` - CLI
  - `anyhow` - Error handling
  - `thiserror` - Custom errors
  - `colored` - Terminal colors
- [x] **Add dev dependencies**
  - `criterion` - Benchmarking
  - `pretty_assertions` - Better test output
- [x] **Create basic error types** (`src/diagnostics/error.rs`)
  - `ParseError`
  - `TypeError`
  - `DiagnosticError`
- [x] **Create `SourceSpan` type** (`src/utils/span.rs`)
  - Track file location (line, column, offset)
- [x] **Create basic CLI structure** (`src/main.rs`)
  - `pyrust-check <file>` command
  - Version flag
  - Help text
- [x] **Verify setup**
  - `cargo build` succeeds
  - `cargo test` runs (no tests yet)
  - `cargo clippy` passes

### Success Criteria
✅ Repository initialized with proper structure

✅ `cargo build` compiles successfully

✅ `cargo test` runs without errors

✅ `cargo clippy` passes with no warnings

✅ Basic CLI prints help text

### Deliverables
- Initialized Cargo project
- Configured development tools
- Basic error type hierarchy
- CLI skeleton
- Clean build with no warnings

---

## Phase 1: Parser & AST

**Duration**: Week 1 (Days 3-5)

**Status**: 🏗️ In Progress

**Focus**: Parse Python source files into AST

### Goals
- Integrate RustPython parser
- Handle syntax errors gracefully
- Build AST traversal infrastructure
- Test on simple Python files

### Tasks

#### Day 3: Parser Integration
- [ ] **`src/parser/mod.rs`**
  - Wrap `rustpython_parser::parse_program()`
  - Convert parse errors to custom error types
  - Add source file path tracking
- [ ] **`src/parser/ast.rs`**
  - Create wrapper types for key AST nodes
  - Add `SourceSpan` to all node types
  - Focus on: Module, FunctionDef, Assign, AnnAssign, Expr
- [ ] **Write basic parser tests**
  - Parse simple function definition
  - Parse variable with type annotation
  - Handle syntax errors gracefully
  - Test empty file

#### Day 4-5: AST Visitor Pattern
- [ ] **`src/parser/visitor.rs`**
  - Define `AstVisitor` trait
  - Implement `walk_*` functions for each node type
  - Support pre-order and post-order traversal
- [ ] **Create example visitor**
  - `FunctionCollector` - collects all function names
  - Test visitor pattern works
- [ ] **Write comprehensive tests**
  - Parse multi-function file
  - Parse file with classes (basic structure)
  - Parse file with imports (structure only, no resolution)
  - Handle Unicode in strings and comments

### Success Criteria
✅ Parse valid Python 3.8+ files successfully

✅ Handle syntax errors without panicking

✅ Traverse AST and collect all function definitions

✅ All parser tests pass

### Deliverables
- Working Python parser
- AST wrapper types with spans
- Visitor pattern implementation
- Comprehensive parser tests
- Can parse 100+ line Python files

### Example After This Phase
```bash
$ pyrust-check parse example.py
Parsed successfully: example.py
Found 5 function definitions:
  - add (line 1)
  - subtract (line 5)
  - multiply (line 9)
  - divide (line 13)
  - main (line 17)
```

---

## Phase 2: Symbol Table

**Duration**: Week 1-2 (Days 6-10)

**Status**: ⏳ Pending

**Focus**: Build symbol table and resolve names in single files

### Goals
- Track all variable and function definitions
- Handle Python scoping rules (LEGB)
- Resolve name references
- No import resolution yet (single file only)

### Tasks

#### Day 6-7: Symbol Table Structure
- [ ] **`src/symbols/table.rs`**
  - `Symbol` struct (name, kind, type, location)
  - `SymbolKind` enum (Variable, Function, Parameter)
  - `Scope` struct (parent, children, symbols HashMap)
  - `ScopeKind` enum (Module, Function)
- [ ] **`src/symbols/scope.rs`**
  - Scope tree management
  - Enter/exit scope
  - Lookup symbol in current scope
  - Lookup with LEGB rules (Local, Enclosing, Global, Builtin)
- [ ] **Write symbol table tests**
  - Create scope
  - Add symbols
  - Lookup symbols
  - Test LEGB resolution

#### Day 8-9: Symbol Resolution Pass
- [ ] **`src/symbols/resolver.rs`**
  - Implement `SymbolResolver` visitor
  - First pass: collect all definitions
    - Function definitions
    - Variable assignments with annotations
    - Function parameters
  - Second pass: resolve all references
    - Variable uses
    - Function calls
  - Track undefined names
- [ ] **Handle Python scoping**
  - Function scope creates new scope
  - Nested functions (enclosing scope)
  - Module-level scope (global)
- [ ] **Write resolution tests**
  - Resolve simple variable
  - Resolve function reference
  - Detect undefined name
  - Handle nested function scopes

#### Day 10: Built-in Types
- [ ] **`src/symbols/builtins.rs`**
  - Define built-in scope
  - Add built-in types: `int`, `str`, `bool`, `float`, `None`
  - Add built-in functions: `print`, `len`, `range`
- [ ] **Test built-in resolution**
  - Resolve `int` type annotation
  - Resolve `print` function call
  - Built-ins don't shadow local definitions

### Success Criteria
✅ Build complete symbol table for single Python file

✅ Correctly resolve all names using LEGB rules

✅ Detect undefined variables

✅ Handle nested function scopes

✅ Built-in types and functions available

### Deliverables
- Symbol table implementation
- Name resolution working
- Built-in types defined
- Comprehensive resolution tests
- Can resolve names in 200+ line files

### Example After This Phase
```bash
$ pyrust-check symbols example.py
Symbol Table for example.py:
Module scope:
  - add: Function (line 1)
  - x: int (line 10)
  - result: Unknown (line 11)
  
Undefined references:
  - undefined_var at line 15:5
```

---

## Phase 3: Type System Foundation

**Duration**: Week 2 (Days 11-13)

**Status**: ⏳ Pending

**Focus**: Represent basic Python types in Rust

### Goals
- Define type representation
- Implement primitive types
- Handle function signatures
- Basic type equality checking

### Tasks

#### Day 11-12: Type Representation
- [ ] **`src/types/mod.rs`**
  - `Type` enum with variants:
    - `Int`, `Str`, `Bool`, `Float`, `None`
    - `Function(FunctionType)`
    - `Unknown`
    - `Any`
  - Type equality (`PartialEq`, `Eq`)
  - Type display (`Display`)
- [ ] **`src/types/primitives.rs`**
  - Primitive type definitions
  - Conversions from AST type annotations
- [ ] **`src/types/functions.rs`**
  - `FunctionType` struct
    - Parameters: Vec<(String, Type)>
    - Return type: Type
  - Function type equality
- [ ] **Write type tests**
  - Create types
  - Compare types for equality
  - Display types as strings

#### Day 13: Type Annotations
- [ ] **`src/types/annotations.rs`**
  - Parse type annotations from AST
  - Convert AST annotation → Type
  - Handle:
    - Simple types: `int`, `str`
    - Built-in generics (basic): `list`, `dict` (no type params yet)
- [ ] **Test annotation parsing**
  - Parse `x: int`
  - Parse `def f(a: str) -> bool`
  - Handle missing annotations (→ Unknown)

### Success Criteria
✅ Represent primitive types

✅ Represent function signatures

✅ Parse type annotations from Python source

✅ Type equality works correctly

### Deliverables
- Type system with primitives
- Function type representation
- Annotation parser
- Type tests
- Foundation for type checking

---

## Phase 4: Basic Type Checking

**Duration**: Week 2-3 (Days 14-17)

**Status**: ⏳ Pending

**Focus**: Check types and detect mismatches

### Goals
- Infer types for literals and variables
- Check variable assignments
- Check function calls
- Detect type errors

### Tasks

#### Day 14: Type Inference Context
- [ ] **`src/checker/context.rs`**
  - `CheckContext` struct
    - Symbol table reference
    - Current scope
    - Error collector
  - Methods for type lookup
  - Methods for reporting errors
- [ ] **`src/checker/mod.rs`**
  - Main type checker struct
  - Initialize context
  - Run type checking pass

#### Day 15-16: Expression Type Checking
- [ ] **`src/checker/expressions.rs`**
  - Implement expression type inference
  - Literals: `5` → int, `"hello"` → str, `True` → bool
  - Variables: lookup in symbol table
  - Binary operations: `int + int` → int
  - Function calls: check against function signature
- [ ] **Handle type mismatches**
  - `int + str` → error
  - Wrong number of arguments → error
  - Wrong argument type → error
- [ ] **Write expression tests**
  - Infer literal types
  - Infer variable types
  - Check binary operations
  - Check function calls

#### Day 17: Statement Type Checking
- [ ] **`src/checker/statements.rs`**
  - Check variable assignments
    - `x: int = 5` → OK
    - `x: int = "hello"` → Error
  - Check function definitions
    - Store function signature
    - Check return type (basic)
  - Check function parameters
- [ ] **Write statement tests**
  - Check valid assignments
  - Detect type mismatches
  - Check function definitions

### Success Criteria
✅ Infer types for literals and variables

✅ Check variable assignments against annotations

✅ Check function calls for arity and argument types

✅ Detect and report type errors

✅ All type checking tests pass

### Deliverables
- Type inference for expressions
- Type checking for statements
- Error detection
- Comprehensive type checking tests
- Can type check 100+ line programs

### Example After This Phase
```python
# test.py
def add(a: int, b: int) -> int:
    return a + b

x: int = 5
y: str = "hello"
z: int = add(x, 10)      # OK
w: str = add(x, 10)      # Error!
```

```bash
$ pyrust-check test.py
Error: Type mismatch
  --> test.py:8:10
   |
 8 | w: str = add(x, 10)
   |          ^^^^^^^^^^ expected `str`, found `int`
```

---

## Phase 5: Error Reporting

**Duration**: Week 3 (Days 18-19)

**Status**: ⏳ Pending

**Focus**: Beautiful, actionable error messages

### Goals
- Show source location with line numbers
- Highlight problematic code
- Explain errors clearly
- Use colors for readability

### Tasks

#### Day 18: Error Formatting
- [ ] **`src/diagnostics/reporter.rs`**
  - Format errors with source context
  - Show line numbers
  - Highlight error span with `^` or color
  - Use `colored` crate for terminal colors
- [ ] **Error message structure**
  ```
  error: Type mismatch
    --> file.py:10:5
     |
  10 |     x: int = "hello"
     |              ^^^^^^^ expected `int`, found `str`
  ```
- [ ] **Test error formatting**
  - Format basic error
  - Test with multi-line spans
  - Test color output (manually)

#### Day 19: Error Messages
- [ ] **Improve error messages**
  - Add helpful context
  - Suggest fixes where obvious
  - Show type information
- [ ] **Handle multiple errors**
  - Collect all errors before reporting
  - Don't stop at first error
  - Limit total errors shown (e.g., max 10)

### Success Criteria
✅ Errors show source location

✅ Error messages are clear and helpful

✅ Colors make errors easy to scan

✅ Multiple errors reported together

### Deliverables
- Beautiful error formatting
- Clear error messages
- Color support
- Multiple error handling
- User-friendly output

---

## Phase 6: Testing & Polish

**Duration**: Week 3 (Days 20-21)

**Status**: ⏳ Pending

**Focus**: Comprehensive testing and final polish

### Goals
- Comprehensive test suite
- Integration tests with real Python files
- Documentation
- CLI polish

### Tasks

#### Day 20: Testing
- [ ] **Unit tests**
  - Review and complete unit tests for all modules
  - Target: >70% code coverage
- [ ] **Integration tests**
  - Create `tests/integration/` directory
  - Create fixture Python files in `tests/fixtures/`
  - Test end-to-end: parse → check → report
  - Test files:
    - `basic_types.py` - simple type checking
    - `functions.py` - function signatures
    - `errors.py` - expected to have type errors
    - `valid.py` - no errors expected
- [ ] **Run tests**
  ```bash
  cargo test
  cargo test --release
  ```
- [ ] **Coverage report**
  ```bash
  cargo tarpaulin --out Html
  ```

#### Day 21: Polish & Documentation
- [ ] **CLI improvements**
  - Better help text
  - Progress indicator for multiple files
  - Summary statistics (X files checked, Y errors found)
  - Exit codes: 0 (no errors), 1 (type errors found), 2 (parse error)
- [ ] **README.md**
  - Installation instructions
  - Quick start guide
  - Examples
  - Feature list
  - Known limitations
- [ ] **CONTRIBUTING.md**
  - Development setup
  - Running tests
  - Code style
  - PR process
- [ ] **Add examples**
  - Create `examples/` directory
  - Example Python files that demonstrate MVP

### Success Criteria
✅ >70% test coverage

✅ All tests pass

✅ Integration tests with real Python files

✅ Documentation complete

✅ CLI polished and user-friendly

### Deliverables
- Comprehensive test suite
- Integration tests
- README and CONTRIBUTING docs
- Polished CLI
- Example files
- Ready for initial release

---

## 🎯 MVP Feature Checklist

### Core Features
- [ ] **Parser**
  - [ ] Parse Python 3.8+ syntax
  - [ ] Handle syntax errors gracefully
  - [ ] Track source locations

- [ ] **Symbol Table**
  - [ ] Single-file scope resolution
  - [ ] LEGB scoping rules
  - [ ] Built-in types
  - [ ] Undefined name detection

- [ ] **Type System**
  - [ ] Primitive types (int, str, bool, float, None)
  - [ ] Function signatures
  - [ ] Type annotations
  - [ ] Type equality checking

- [ ] **Type Checking**
  - [ ] Variable assignment checking
  - [ ] Function call checking
  - [ ] Literal type inference
  - [ ] Binary operation type checking

- [ ] **Error Reporting**
  - [ ] Source location in errors
  - [ ] Colored output
  - [ ] Multiple error reporting
  - [ ] Clear error messages

- [ ] **CLI**
  - [ ] Check single file
  - [ ] Check directory (multiple files)
  - [ ] Help and version flags
  - [ ] Proper exit codes

- [ ] **Testing**
  - [ ] Unit tests for all modules
  - [ ] Integration tests
  - [ ] >70% coverage

- [ ] **Documentation**
  - [ ] README with examples
  - [ ] Installation instructions
  - [ ] Contributing guide

---

## 📊 Success Metrics

### Technical Metrics
- **Test Coverage**: >70%
- **Check Speed**: <100ms for 100-line file
- **Error Accuracy**: Detect all obvious type errors
- **False Positives**: Minimal on MVP scope

### User Metrics
- **Time to First Check**: <5 minutes from clone to check
- **Error Clarity**: Clear enough to fix without external help
- **Documentation**: Users can self-serve from README

### Project Metrics
- **Codebase**: ~1,500-2,000 lines of Rust
- **Completion Time**: 2-3 weeks
- **Demonstrates**: Core competency in Rust and type systems

---

## 🚧 Known Limitations (MVP)

The MVP intentionally excludes:
- ❌ Complex inheritance (A→B→C case) - **Post-MVP**
- ❌ Classes and methods
- ❌ Generics (List[int], Dict[str, int])
- ❌ Union types (int | str)
- ❌ Protocols and ABCs
- ❌ Import resolution (single file only)
- ❌ Type inference without annotations
- ❌ Control flow analysis
- ❌ Type narrowing
- ❌ Performance optimization
- ❌ Parallel file processing
- ❌ LSP server

These are documented in [ROADMAP.md](./ROADMAP.md) for post-MVP development.

---

## 🎓 Learning Resources

As you implement each phase, refer to:
- **Rust Book**: https://doc.rust-lang.org/book/
- **RustPython Parser**: https://docs.rs/rustpython-parser/
- **Python Type System PEPs**: 
  - PEP 484 (Type Hints)
  - PEP 526 (Variable Annotations)
- **Clap CLI**: https://docs.rs/clap/
- **Type Systems**: "Types and Programming Languages" by Pierce (chapters 1-9)

---

## 🆘 Getting Help

If you get stuck:
1. Check the Implementation Plan document
2. Read RustPython parser examples
3. Review mypy's approach (for comparison)
4. Use `cargo doc --open` for dependency docs
5. Read Rust compiler errors carefully - they're excellent teachers

---

## 🎉 Celebrating Milestones

- **Phase 0 Complete**: Foundation is solid! 🎯
- **Phase 1 Complete**: Parser working! 📝
- **Phase 2 Complete**: Symbols resolved! 🔍
- **Phase 3 Complete**: Types represented! 📊
- **Phase 4 Complete**: Type checking operational! ✅
- **Phase 5 Complete**: Errors beautiful! 🎨
- **Phase 6 Complete**: MVP SHIPPED! 🚀

---

## 📝 Daily Log Template

Keep a development log to track progress:
```markdown
## Day X - [Date]

### Phase
[Current phase]

### Completed
- [ ] Task 1
- [ ] Task 2

### In Progress
- [ ] Task 3

### Blockers
- Issue 1: Description and solution attempted

### Tomorrow
- [ ] Next task

### Notes
- Learnings
- Questions
- Resources used
```
---

<div align="center">

**Let's build something amazing! 💪**

Progress: ██░░░░░░░░░░░░░░░░░░░░░ 14.28%

[Back to README](./README.md) • [Full Roadmap](./ROADMAP.md) • [Implementation Plan](./docs/reference/guide.md)

</div>