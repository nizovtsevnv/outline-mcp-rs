//! Command-line interface module
//!
//! Simple CLI argument parsing for MCP server configuration.

use std::env;

/// Application name constant
const NAME: &str = env!("CARGO_PKG_NAME");

/// Application version constant
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Help content from documentation file
const HELP_CONTENT: &str = include_str!("../docs/HELP.md");

/// Command-line commands
#[derive(Debug, PartialEq, Eq)]
pub enum CliCommand {
    /// Run in HTTP mode
    Http,
    /// Run in STDIO mode (default)
    Stdio,
    /// Show help (handled internally)
    Help,
    /// Show version (handled internally)  
    Version,
}

/// Parse command-line arguments
///
/// Returns the parsed command or exits the process for help/version.
#[must_use]
pub fn parse_args() -> CliCommand {
    let args: Vec<String> = env::args().collect();

    // Check for help arguments
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" | "help" => {
                print_help();
                std::process::exit(0);
            }
            "--version" | "-v" | "version" => {
                print_version();
                std::process::exit(0);
            }
            "--http" | "http" => {
                return CliCommand::Http;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                eprintln!("Use --help for usage information");
                std::process::exit(1);
            }
        }
    }

    CliCommand::Stdio
}

/// Print help information
fn print_help() {
    println!("{HELP_CONTENT}");
}

/// Print version information
fn print_version() {
    println!("{NAME} {VERSION}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_constant() {
        assert_eq!(NAME, "outline-mcp-rs");
    }

    #[test]
    fn test_version_constant() {
        // Version should be non-empty and contain dots (semver format)
        // Allow const_is_empty since VERSION is a compile-time constant from env!()
        #[allow(clippy::const_is_empty)]
        {
            assert!(!VERSION.is_empty());
        }
        assert!(VERSION.contains('.'));
    }

    #[test]
    fn test_help_content() {
        // Help content should contain expected sections
        assert!(HELP_CONTENT.contains("USAGE:"));
        assert!(HELP_CONTENT.contains("CURSOR IDE"));
    }
}
