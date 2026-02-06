// Rust toolchain detection
//
// This module checks for rustc and cargo installation.

use crate::error::{CliError, Result};
use std::process::Command;

/// Minimum required Rust version (in semantic version format)
const MIN_RUST_VERSION: &str = "1.75.0";

/// Check if Rust toolchain is installed and meets minimum version requirements
///
/// This function verifies:
/// - rustc is installed and accessible
/// - cargo is installed and accessible
/// - Rust version meets minimum requirements
///
/// # Returns
/// * `Ok(())` if the toolchain is properly installed
/// * `Err(CliError)` with helpful installation instructions if not
///
/// # Examples
/// ```
/// use create_axum_app::utils::rust_toolchain::check_rust_toolchain;
///
/// match check_rust_toolchain() {
///     Ok(()) => println!("Rust toolchain is ready"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn check_rust_toolchain() -> Result<()> {
    // Check rustc
    let rustc_version = check_command("rustc", &["--version"])?;

    // Parse version to ensure it meets minimum requirements
    if let Some(version_str) = rustc_version.split_whitespace().nth(1) {
        if !version_meets_minimum(version_str, MIN_RUST_VERSION) {
            return Err(CliError::ToolchainError(format!(
                "Rust version {} is below minimum required version {}\n\
                 Please update Rust by running: rustup update",
                version_str, MIN_RUST_VERSION
            )));
        }
    }

    // Check cargo
    check_command("cargo", &["--version"])?;

    Ok(())
}

/// Check if a command exists and can be executed
///
/// # Arguments
/// * `command` - The command to check
/// * `args` - Arguments to pass to the command (typically for version check)
///
/// # Returns
/// * `Ok(String)` containing the command output if successful
/// * `Err(CliError)` if the command failed
fn check_command(command: &str, args: &[&str]) -> Result<String> {
    match Command::new(command).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(CliError::ToolchainError(format!(
                    "{} command failed with non-zero exit code",
                    command
                )))
            }
        }
        Err(e) => {
            Err(CliError::ToolchainError(format!(
                "{} not found. Please install Rust from https://rustup.rs/\n\n\
                 Error details: {}",
                command, e
            )))
        }
    }
}

/// Compare two semantic version strings
///
/// # Arguments
/// * `current` - Current version string (e.g., "1.75.0")
/// * `minimum` - Minimum required version string
///
/// # Returns
/// * `true` if current >= minimum
/// * `false` if current < minimum
fn version_meets_minimum(current: &str, minimum: &str) -> bool {
    let parse_version = |v: &str| -> Vec<u32> {
        v.split('.')
            .map(|part| part.parse().unwrap_or(0))
            .collect()
    };

    let current_parts = parse_version(current);
    let minimum_parts = parse_version(minimum);

    current_parts >= minimum_parts
}

/// Get the current Rust version string
///
/// # Returns
/// * `Ok(String)` containing the version (e.g., "rustc 1.75.0")
/// * `Err(CliError)` if rustc is not available
pub fn get_rust_version() -> Result<String> {
    check_command("rustc", &["--version"])
}

/// Get the current Cargo version string
///
/// # Returns
/// * `Ok(String)` containing the version (e.g., "cargo 1.75.0")
/// * `Err(CliError)` if cargo is not available
pub fn get_cargo_version() -> Result<String> {
    check_command("cargo", &["--version"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(version_meets_minimum("1.75.0", "1.75.0"));
        assert!(version_meets_minimum("1.76.0", "1.75.0"));
        assert!(version_meets_minimum("2.0.0", "1.75.0"));
        assert!(!version_meets_minimum("1.74.0", "1.75.0"));
        assert!(!version_meets_minimum("1.0.0", "1.75.0"));
    }

    #[test]
    fn test_version_parsing() {
        // Test with non-standard version formats
        assert!(version_meets_minimum("1.75.0-nightly", "1.75.0"));
        assert!(version_meets_minimum("1.75.0-beta", "1.75.0"));
    }
}
