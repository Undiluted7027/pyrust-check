use clap::{Parser, Subcommand};
use std::path::PathBuf;
use colored::*;
use pyrust_check::parser::PythonParser;

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
        // Explicitly handle the subcommand Check
        Some(Commands::Check { path }) => {
            check_command(&path);
        }
        // Handle the case where no subcommand is provided (default behavior)
        None => {
            if let Some(path) = cli.path {
                check_command(&path);
            } else {
                eprintln!("{}", "Error: Please provide a path to check".red());
                use clap::CommandFactory;
                let _ = Cli::command().print_help();
                std::process::exit(1);
            }
        }
    }
}

fn parse_command(path: &PathBuf) {
    println!("{} {}", "Parsing:".blue(), path.display());
    
    match PythonParser::parse_file(path) {
        Ok(ast) => {
            println!("{}", "✓ Parsed successfully".green());
            println!("\nAST has {} statements", ast.len());
            // In the future we can print the debug view of our simplified AST
            println!("{:#?}", ast); 
        }
        Err(e) => {
            eprintln!("{} {}", "✗ Parse error:".red(), e);
            std::process::exit(1);
        }
    }
}

fn check_command(path: &PathBuf) {
    println!("{} {}", "Checking:".blue(), path.display());
    println!("{}", "Type checking not yet implemented".yellow());
}