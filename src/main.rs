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
            // TODO: Implement type checking (Phase 3)
            println!("Type checking not yet implemented.");
        }
        Some(Commands::Parse { path }) => {
            println!("Parsing: {}", path.display());
            // TODO: Implement parsing (Phase 1)
            println!("Parser not yet implemented.");
        }
        None => {
            if let Some(path) = cli.path {
                println!("Checking: {}", path.display());
                // TODO: Implement type checking (Phase 3)
                println!("Type checking not yet implemented.");
            } else {
                use clap::CommandFactory;
                let _ = Cli::command().print_help();
            }
        }
    }
}