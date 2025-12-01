# pyrust-check - Complete Roadmap

> **Vision**: The most accurate and performant Python type checker, built in Rust

This document outlines pyrust-check's development roadmap from MVP through advanced capabilities. We're building in the open and focused on solving real type checking pain points.

---

## 🗺️ Roadmap Overview

| Phase | Focus | Timeline | Status |
|-------|-------|----------|--------|
| **Phase 1: MVP** | Core type checking | Weeks 1-3 | 🏗️ 3% Complete |
| **Phase 2: Classes & Inheritance** | OOP support | Weeks 4-7 | 📋 Planned |
| **Phase 3: Advanced Types** | Generics, unions, protocols | Weeks 8-12 | 📋 Planned |
| **Phase 4: The Differentiator** | Complex inheritance solution | Weeks 13-17 | 🔮 Future |
| **Phase 5: Performance** | Optimization & parallelization | Weeks 18-20 | 🔮 Future |
| **Phase 6: Production Ready** | LSP, IDE integration | Weeks 21-26 | 🔮 Future |

---

## Phase 1: MVP 🏗️

**Timeline**: Weeks 1-3
**Status**: 3% Complete (Phase 0 in progress)
**Goal**: Working CLI type checker for basic Python

See [MVP_ROADMAP.md](./MVP_ROADMAP.md) for detailed breakdown.

### Delivered Features (Target)
- ⏳ Parse Python 3.8+ source files
- ⏳ Basic symbol table and name resolution
- ⏳ Primitive type checking (int, str, bool, float, None)
- ⏳ Function signature checking
- ⏳ Clear error messages with source locations
- ⏳ CLI interface

**Completion Target**: End of Week 3

**Current Status**: Phase 0 (Foundation) in progress

### MVP Known Limitations

For a comprehensive list of intentionally excluded features, see the
[Known Limitations section in MVP_ROADMAP.md](./MVP_ROADMAP.md#-known-limitations-mvp).

These limitations are by design and are addressed in subsequent phases of this roadmap.

---

## Phase 2: Classes & Inheritance

**Timeline**: Weeks 4-7 (4 weeks)

**Status**: 📋 Planned

**Goal**: Add object-oriented programming support

### 2.1 Basic Classes (Week 4)

**Motivation**: Most Python code uses classes

#### Class Type System
- [ ] **`src/types/classes.rs`**
  - `ClassType` struct
  - Store class name and location
  - Track class attributes
  - Track method signatures
  - Class type equality
- [ ] **Class definitions**
  - Parse class definitions
  - Store in symbol table
  - Track `__init__` method
- [ ] **Instance creation**
  - Type check `MyClass()` calls
  - Check `__init__` parameters
- [ ] **Attribute access**
  - Check `obj.attr` access
  - Resolve attribute types from class

**Impact**: Can type check basic OOP code

```python
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
    
    def greet(self) -> str:
        return f"Hello, I'm {self.name}"

p: Person = Person("Alice", 30)  # OK
x: int = p.greet()               # Error: expected int, got str
```

### 2.2 Simple Inheritance (Week 5)

**Motivation**: Support basic inheritance chains

#### Single Inheritance
- [ ] **Inheritance tracking**
  - Parse `class B(A):`
  - Store base class in `ClassType`
  - Build inheritance chain
- [ ] **Method resolution**
  - Lookup methods in class hierarchy
  - Simple MRO (depth-first)
- [ ] **Subtype checking**
  - `Child` is subtype of `Parent`
  - Works in function calls
  - Works in assignments

**Impact**: Handle single inheritance chains

```python
class Animal:
    def speak(self) -> str:
        return "..."

class Dog(Animal):
    def speak(self) -> str:
        return "Woof!"

def make_speak(animal: Animal) -> str:
    return animal.speak()

dog: Dog = Dog()
result: str = make_speak(dog)  # OK: Dog is subtype of Animal
```

### 2.3 Method Override Checking (Week 6)

**Motivation**: Catch signature mismatches in overrides

#### Override Validation
- [ ] **`src/checker/overrides.rs`**
  - Check method signature compatibility
  - Parameter types (contravariance)
  - Return types (covariance)
  - Detect violations
- [ ] **Liskov Substitution Principle**
  - Warn when override breaks contract
  - Allow explicit overrides with different types
- [ ] **`@override` decorator support**
  - Recognize explicit override intent
  - Require override for abstract methods

**Impact**: Find bugs in inheritance hierarchies early

```python
class Base:
    def process(self, x: int) -> str:
        return str(x)

class Derived(Base):
    def process(self, x: int) -> int:  # Warning: return type changed
        return x * 2
```

### 2.4 Abstract Base Classes (Week 7)

**Motivation**: Support Python's ABC pattern

#### ABC Support
- [ ] **ABC detection**
  - Recognize `ABC` base class
  - Parse `@abstractmethod` decorator
- [ ] **Abstract method tracking**
  - Mark methods as abstract
  - Check implementations in subclasses
  - Error if abstract method not implemented
- [ ] **Abstract class instantiation**
  - Error on `AbstractClass()` call
  - OK for concrete subclasses

**Impact**: Support common Python patterns

```python
from abc import ABC, abstractmethod

class DataStore(ABC):
    @abstractmethod
    def save(self, data: str) -> bool:
        pass

class FileStore(DataStore):
    def save(self, data: str) -> bool:  # Must implement
        return True

store: DataStore = FileStore()  # OK
direct: DataStore = DataStore()  # Error: cannot instantiate ABC
```

---

## Phase 3: Advanced Types

**Timeline**: Weeks 8-12 (5 weeks)
**Status**: 📋 Planned
**Goal**: Support modern Python type system features

### 3.1 Generic Types (Weeks 8-9)

**Motivation**: Most real code uses List[int], Dict[str, int], etc.

#### Built-in Generics
- [ ] **`src/types/generics.rs`**
  - `GenericType` with type parameters
  - Type variable representation
  - Type substitution
- [ ] **Common generics**
  - `List[T]`
  - `Dict[K, V]`
  - `Tuple[T1, T2, ...]`
  - `Optional[T]` (same as `T | None`)
  - `Set[T]`
- [ ] **Generic instantiation**
  - `List[int]` creates specific type
  - Check element types on list operations
  - Check key/value types for dicts
- [ ] **Generic functions**
  - Type variables in function signatures
  - Basic inference for generics

**Impact**: Type check modern Python code

```python
from typing import List, Dict

def sum_list(numbers: List[int]) -> int:
    return sum(numbers)

def get_name(users: Dict[int, str], id: int) -> str:
    return users[id]

nums: List[int] = [1, 2, 3]
result: int = sum_list(nums)        # OK
bad: int = sum_list(["a", "b"])     # Error: List[str] not List[int]
```

### 3.2 Union Types (Week 10)

**Motivation**: Handle `int | str`, `Optional[T]`

#### Union Support
- [ ] **`src/types/unions.rs`**
  - Union type representation
  - Union normalization (flatten nested)
  - Union simplification
- [ ] **Union type checking**
  - Value matches if matches any variant
  - Check all variants for operations
- [ ] **Type narrowing** (basic)
  - `isinstance()` checks narrow type
  - `is None` checks narrow type
  - Track narrowing in control flow

**Impact**: Handle `None` and multiple return types

```python
def find_user(id: int) -> str | None:
    if id > 0:
        return "User"
    return None

result: str | None = find_user(5)
if result is not None:
    length: int = len(result)  # OK: narrowed to str
```

### 3.3 Protocols (Week 11)

**Motivation**: Support structural typing (duck typing)

#### Protocol Support
- [ ] **`src/types/protocols.rs`**
  - Protocol type representation
  - Store required methods/attributes
- [ ] **Protocol checking**
  - Structural subtype checking
  - Check if class implements protocol
  - Work without explicit inheritance
- [ ] **Built-in protocols**
  - `Iterable`, `Iterator`
  - `Sized`, `Container`
  - `Callable`

**Impact**: Type check duck-typed code

```python
from typing import Protocol

class Drawable(Protocol):
    def draw(self) -> None: ...

class Circle:
    def draw(self) -> None:
        print("Drawing circle")

def render(obj: Drawable) -> None:
    obj.draw()

circle = Circle()
render(circle)  # OK: Circle implements Drawable (structural)
```

### 3.4 Advanced Annotations (Week 12)

**Motivation**: Support all `typing` module features

#### Type System Features
- [ ] **Literal types**
  - `Literal["GET", "POST"]`
  - Check exact value matches
- [ ] **Final**
  - Mark variables as immutable
  - Error on reassignment
- [ ] **ClassVar**
  - Class variables vs instance variables
- [ ] **TypeAlias**
  - Type alias definitions
  - Resolve aliases during checking
- [ ] **Callable types**
  - Function types in annotations
  - Check callback signatures

**Impact**: Support full Python type system

```python
from typing import Literal, Final, Callable

Mode = Literal["read", "write"]

def open_file(path: str, mode: Mode) -> None:
    pass

open_file("file.txt", "read")    # OK
open_file("file.txt", "append")  # Error: not a valid literal

MAX_SIZE: Final[int] = 100
MAX_SIZE = 200  # Error: cannot reassign Final

callback: Callable[[int], str] = str
```

---

## Phase 4: The Differentiator

**Timeline**: Weeks 13-17 (5 weeks)
**Status**: 🔮 Future
**Goal**: Solve the complex inheritance problem better than mypy/Pyright

### 4.1 Multi-Level Inheritance Analysis (Weeks 13-14)

**Motivation**: THIS IS YOUR UNIQUE VALUE PROPOSITION

#### Advanced MRO
- [ ] **`src/checker/inheritance.rs`**
  - C3 linearization algorithm
  - Complete MRO calculation
  - Diamond inheritance patterns
  - Multiple inheritance support
- [ ] **MRO caching**
  - Cache MRO calculations
  - Invalidate on changes
  - Optimize lookup performance

**Impact**: Handle Python's complex inheritance correctly

### 4.2 The A→B→C Problem (Weeks 15-16)

**Motivation**: Your specific pain point

#### Smart Override Detection
- [ ] **`src/checker/smart_overrides.rs`**
  - Track intentional type changes in hierarchies
  - Distinguish bugs from intentional breaks
  - Context-aware override checking
- [ ] **Heuristics for false positives**
  - Pattern recognition for common cases
  - Abstract → Concrete → Specialized pattern
  - Runtime vs static type differences
- [ ] **Configurable strictness**
  - Strict mode: LSP violations are errors
  - Lenient mode: warnings for known patterns
  - Custom rules per project

**Your Test Case:**
```python
class A(ABC):
    @abstractmethod
    def method(self) -> int: ...

class B(A):
    def method(self) -> int: ...
    
class C(B):
    def method(self) -> str:  # Intentional change
        return "hello"

# pyrust-check should:
# 1. Detect this pattern
# 2. Warn but not error (it's intentional)
# 3. Provide context about the inheritance chain
# 4. Explain the LSP violation clearly
```

**Impact**: 30-50% reduction in false positives on your codebase

### 4.3 Constraint-Based Inference (Week 17)

**Motivation**: Better handle complex type relationships

#### Constraint Solver
- [ ] **`src/checker/constraints.rs`**
  - Generate constraints during checking
  - Equality constraints: `T1 = T2`
  - Subtype constraints: `T1 <: T2`
  - Member constraints: `T has attr`
- [ ] **Unification algorithm**
  - Solve constraint sets
  - Find most general types
  - Detect unsolvable constraints
- [ ] **Bidirectional checking**
  - Infer from usage
  - Check against annotations
  - Combine both approaches

**Impact**: Handle cases mypy/Pyright struggle with

---

## Phase 5: Performance

**Timeline**: Weeks 18-20 (3 weeks)
**Status**: 🔮 Future
**Goal**: 3-5x faster than mypy

### 5.1 Profiling & Baseline (Week 18)

**Motivation**: Know where to optimize

#### Performance Analysis
- [ ] **Benchmark suite**
  - Create test projects of various sizes
  - 100 lines, 1K lines, 10K lines, 100K lines
  - Measure baseline performance
- [ ] **Profiling**
  - Use `cargo flamegraph`
  - Identify hot paths
  - Find memory bottlenecks
- [ ] **Comparison with mypy**
  - Run same projects through mypy
  - Record timing data
  - Set performance targets

**Impact**: Data-driven optimization

### 5.2 Parallel Processing (Week 19)

**Motivation**: Utilize all CPU cores

#### Parallelization
- [ ] **`src/parallel/mod.rs`**
  - Parse multiple files concurrently
  - Use Rayon for data parallelism
  - Thread-safe symbol table (DashMap)
- [ ] **Dependency analysis**
  - Build file dependency graph
  - Process independent files in parallel
  - Merge results safely
- [ ] **Progress reporting**
  - Show files being processed
  - Display progress bar
  - Real-time statistics

**Impact**: Near-linear speedup with core count

### 5.3 Optimization (Week 20)

**Motivation**: Squeeze every bit of performance

#### Optimization Techniques
- [ ] **String interning**
  - Intern all identifiers
  - Reduce memory usage
  - Faster string comparisons
- [ ] **Arena allocation**
  - Allocate AST nodes in arena
  - Reduce allocation overhead
  - Better cache locality
- [ ] **Type caching**
  - Cache resolved types
  - Avoid redundant work
  - Incremental checking
- [ ] **Fast paths**
  - Optimize common operations
  - Inline hot functions
  - SIMD where applicable

**Target**: 3-5x faster than mypy on 10K+ line projects

---

## Phase 6: Production Ready

**Timeline**: Weeks 21-26 (6 weeks)
**Status**: 🔮 Future
**Goal**: Professional tool ready for teams

### 6.1 Configuration System (Week 21)

**Motivation**: Make tool configurable

#### Config Files
- [ ] **`pyproject.toml` support**
  ```toml
  [tool.pyrust-check]
  strict = true
  ignore-patterns = ["build/", "venv/"]
  max-errors = 50
  ```
- [ ] **`.pyrust-check.toml`**
  - Project-specific settings
  - Per-directory configs
  - Config inheritance
- [ ] **CLI overrides**
  - Command-line flags override config
  - `--strict`, `--ignore`, etc.

**Impact**: Teams can customize behavior

### 6.2 Incremental Checking (Week 22)

**Motivation**: Fast feedback on changes

#### Caching System
- [ ] **`src/cache/mod.rs`**
  - Cache parsed ASTs
  - Cache type checking results
  - Detect file changes
  - Invalidate stale caches
- [ ] **Disk cache**
  - Serialize cache to disk
  - Fast startup on re-check
  - Cache versioning
- [ ] **Watch mode**
  - Monitor file changes
  - Re-check on save
  - Instant feedback

**Impact**: Sub-second re-checks on large projects

### 6.3 LSP Server (Weeks 23-24)

**Motivation**: Editor integration

#### Language Server Protocol
- [ ] **`src/lsp/server.rs`**
  - Use `tower-lsp` crate
  - Implement core LSP features
- [ ] **Features**
  - `textDocument/didOpen`
  - `textDocument/didChange`
  - `textDocument/publishDiagnostics`
  - `textDocument/hover` (show types)
  - `textDocument/definition` (go to def)
- [ ] **Real-time checking**
  - Check as you type
  - Show errors inline
  - Type information on hover

**Impact**: IDE integration

### 6.4 Editor Plugins (Week 25)

**Motivation**: Easy installation

#### VS Code Extension
- [ ] **Create extension**
  - Package LSP server
  - Configuration UI
  - Settings integration
- [ ] **Publish to marketplace**
  - Create publisher account
  - Package extension
  - Write extension docs
- [ ] **Other editors**
  - Neovim setup guide
  - Sublime Text guide
  - Emacs guide

**Impact**: Professional developer experience

### 6.5 Documentation & Release (Week 26)

**Motivation**: Polish for v1.0

#### Documentation
- [ ] **User guide**
  - Installation
  - Configuration
  - Usage examples
  - Editor setup
- [ ] **API documentation**
  - Library usage
  - Extending the checker
  - Custom rules
- [ ] **Architecture docs**
  - Design decisions
  - Type system spec
  - Contributing guide
- [ ] **Blog post**
  - Announce v1.0
  - Explain unique features
  - Show benchmarks

**Impact**: Ready for production use

---

## 🎯 Feature Prioritization

We prioritize features based on:

1. **Correctness**: Does it reduce false positives?
2. **Performance**: Does it make checking faster?
3. **Usability**: Does it improve developer experience?
4. **Uniqueness**: Does it differentiate from competitors?
5. **Feasibility**: Can we implement it well?

### High Priority (Months 1-4)
1. ✅ MVP with basic type checking
2. ✅ Classes and inheritance
3. ✅ Generics and unions
4. ✅ The A→B→C solution (differentiator)
5. ✅ Performance optimization

### Medium Priority (Months 5-6)
1. LSP server
2. Editor plugins
3. Incremental checking
4. Configuration system
5. Documentation

### Low Priority (Future)
1. Advanced features (more type system features)
2. Plugin system for custom rules
3. Integration with CI/CD
4. Web interface
5. Language interop (check from Python)

---

## 📊 Success Metrics by Phase

### Phase 1 (MVP)
- **Lines Checked**: Up to 1K lines
- **Types Supported**: Primitives, functions
- **Speed**: <100ms per 100-line file
- **Accuracy**: Detects basic type errors

### Phase 2 (Classes)
- **Lines Checked**: Up to 5K lines
- **Types Supported**: Classes, simple inheritance
- **Speed**: <500ms per 1K-line file
- **Accuracy**: Handles basic OOP

### Phase 3 (Advanced Types)
- **Lines Checked**: Up to 10K lines
- **Types Supported**: Generics, unions, protocols
- **Speed**: <1s per 1K-line file
- **Accuracy**: Covers 70% of Python type system

### Phase 4 (Differentiator)
- **Lines Checked**: Up to 50K lines
- **False Positives**: 30-50% reduction vs mypy
- **Unique Value**: Solves A→B→C case
- **Accuracy**: Better than existing tools on complex inheritance

### Phase 5 (Performance)
- **Lines Checked**: 100K+ lines
- **Speed**: 3-5x faster than mypy
- **Parallel**: Scales with CPU cores
- **Memory**: Efficient caching

### Phase 6 (Production)
- **Editor Support**: VS Code, Neovim, etc.
- **Features**: LSP, incremental, real-time
- **Documentation**: Complete
- **Adoption**: Ready for teams

---

## 🤝 Community Involvement

We're building pyrust-check in the open! Here's how you can help:

### Immediate Needs
- 🐛 **Bug reports**: Test on your codebases
- 🧪 **Test cases**: Contribute Python files that expose bugs
- 📖 **Documentation**: Improve guides
- 💡 **Ideas**: Suggest features

### Ongoing Needs
- 🔧 **False positive reports**: Help us improve accuracy
- 🛠️ **Benchmark suites**: Share large codebases for testing
- 📝 **Content**: Write about your experience
- 🎨 **Design**: Help with error message formatting

### How to Contribute
1. Check [Issues](https://github.com/Undiluted7027/pyrust-check/issues) for open tasks
2. Read [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines
3. Submit PRs with your improvements
4. Share feedback on accuracy

---

## 🔄 Roadmap Updates

This roadmap is a living document. We update it:
- **Weekly**: Based on implementation progress
- **Monthly**: Based on learnings and challenges
- **After major phases**: Based on user feedback

### How to Influence the Roadmap
1. 👍 Open issues for feature requests
2. 💬 Comment on roadmap issues
3. 📊 Share your type checking pain points
4. 🗳️ Vote on features you need

---

## 📅 Release Schedule

### Version Strategy
- **v0.1**: MVP with basic checking
- **v0.2**: Classes and inheritance
- **v0.3**: Generics and advanced types
- **v0.4**: Complex inheritance solution
- **v0.5**: Performance optimized
- **v1.0**: Production-ready with LSP

### Release Cadence
- **MVP**: Week 3
- **Minor versions**: Every 3-4 weeks
- **Major version** (v1.0): Month 6
- **Patch versions**: As needed

---

## 🎓 Learning from Others

We're inspired by and learning from:

- **mypy**: Comprehensive type system, but slow on large codebases
- **Pyright**: Fast, but same false positive issues in inheritance
- **Pyre**: Good incremental checking, written in OCaml
- **rust-analyzer**: Best-in-class LSP implementation
- **Ruff**: Shows what's possible with Rust rewrites
- **pyflakes/pylint**: Simpler but practical checking

**Key Insight**: Rust's performance + careful type system design = better tool

---

## 🚀 Long-term Vision (1-2 Years)

### The Future of pyrust-check

**Vision**: pyrust-check becomes the **go-to** type checker for Python projects with complex OOP

1. **Better Accuracy**
   - Significantly fewer false positives than mypy/Pyright
   - Smart handling of complex inheritance patterns
   - Understands common Python idioms

2. **Superior Performance**
   - 5-10x faster than mypy on large codebases
   - Instant feedback in editors
   - Scales to hundreds of thousands of lines

3. **Excellent Developer Experience**
   - Beautiful error messages
   - Helpful suggestions
   - Seamless editor integration
   - Great documentation

4. **Open Source Success**
   - Active community
   - Regular releases
   - Good test coverage
   - Well-maintained

### Success Looks Like

- ✅ 1K+ GitHub stars
- ✅ Used by 100+ projects
- ✅ Demonstrably better than mypy on complex inheritance
- ✅ Mentioned in Python typing discussions

---

## 💬 Feedback

We want to hear from you!

- **What type checking problems do you face?**
- **Where does mypy/Pyright frustrate you?**
- **What features would make this tool valuable?**
- **Would you use this on your projects?**

Share your thoughts:
- [GitHub Issues](https://github.com/Undiluted7027/pyrust-check/issues)
- [GitHub Discussions](https://github.com/Undiluted7027/pyrust-check/discussions)

---

## 📝 Changelog

### Roadmap Version History

- **v1.0** (Current) - Initial comprehensive roadmap
- Future updates will be tracked here

---

<div align="center">

**Building a better Python type checker, one commit at a time** 🚀

[Back to README](./README.md) • [MVP Roadmap](./MVP_ROADMAP.md) • [Implementation Plan](./docs/reference/guide.md)

---

*Last Updated: December 1, 2025*

*Next Review: December 15, 2025*

</div>