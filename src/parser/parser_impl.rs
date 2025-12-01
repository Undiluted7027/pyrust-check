use rustpython_parser::{parse, Mode, ParseError, ast::Mod};
use rustpython_parser::text_size::TextRange;
use std::fs;
use std::path::Path;
use crate::diagnostics::{PyRustError, Result, SourceLocation};

pub struct PythonParser;

impl PythonParser {
    /// Reads a file and parses it into a RustPython AST
    pub fn parse_file(path: &Path) -> Result<Vec<rustpython_parser::ast::Stmt>> {
        let source = fs::read_to_string(path)?;
        Self::parse_source(&source, path)
    }

    /// Parses a string of source code into a RustPython AST
    pub fn parse_source(
        source: &str,
        path: &Path,
    ) -> Result<Vec<rustpython_parser::ast::Stmt>> {
        // Mode::Module is standard for .py files
        let parsed = parse(source, Mode::Module, path.to_str().unwrap_or("<unknown>"))
            .map_err(|e| Self::convert_parse_error(e, source, path))?;

        // Extract statements from the Module variant
        // In rustpython-parser 0.4.0, Mod variants are tuple variants wrapping a struct
        match parsed {
            Mod::Module(m) => Ok(m.body),
            Mod::Expression(e) => {
                // Wrap single expression in an Expr statement
                Ok(vec![rustpython_parser::ast::Stmt::Expr(rustpython_parser::ast::StmtExpr {
                     range: TextRange::default(),
                     value: e.body, 
                 })])
            },
            // Handle other variants (FunctionType, Interactive) as empty for now
            _ => Ok(vec![]), 
        }
    }

    fn convert_parse_error(error: ParseError, source: &str, path: &Path) -> PyRustError {
        // Convert the byte offset to line and column
        let offset = error.offset.to_usize();
        let (line, column) = Self::offset_to_line_col(source, offset);

        let location = SourceLocation {
            file: path.to_path_buf(),
            line,
            column,
        };
        
        PyRustError::ParseError {
            location,
            message: error.error.to_string(),
        }
    }

    /// Helper to calculate line and column from byte offset
    fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
        if offset == 0 {
            return (1, 0);
        }

        let mut line = 1;
        let mut last_line_start = 0;
        
        for (i, c) in source.char_indices() {
            if i >= offset {
                break;
            }
            if c == '\n' {
                line += 1;
                last_line_start = i + 1;
            }
        }
        
        let column = offset.saturating_sub(last_line_start);
        (line, column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_parse_variable_annotation() {
        let source = r#"
x: int = 5
y: str = "hello"
"#;
        let result = PythonParser::parse_source(source, Path::new("test.py"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_syntax_error() {
        let source = "def invalid syntax";
        let result = PythonParser::parse_source(source, Path::new("test.py"));
        
        match result {
            Err(PyRustError::ParseError { location, .. }) => {
                assert_eq!(location.line, 1);
            }
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_empty_file() {
        let source = "";
        let result = PythonParser::parse_source(source, Path::new("test.py"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}