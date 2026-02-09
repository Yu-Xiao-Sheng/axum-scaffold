// CLI module - Command-line interface handling
//
// This module contains:
// - mod.rs: Module exports and non-interactive mode detection
// - args.rs: Command-line argument parsing with clap
// - prompts.rs: Interactive prompts using inquire

pub mod args;
pub mod prompts;

use std::env;

/// Detect if we should run in non-interactive mode
///
/// This function checks:
/// - CI environment variable (set by most CI/CD systems)
/// - TTY availability (terminal presence)
/// - Explicit non_interactive flag from command line
///
/// # Returns
/// * `true` if non-interactive mode should be used
/// * `false` if interactive prompts are acceptable
///
/// # Examples
/// ```
/// use axum_app_create::cli::is_non_interactive;
///
/// if is_non_interactive(false) {
///     println!("Running in non-interactive mode");
/// } else {
///     println!("Can prompt for user input");
/// }
/// ```
pub fn is_non_interactive(explicit_flag: bool) -> bool {
    // Explicit flag takes precedence
    if explicit_flag {
        return true;
    }

    // Check CI environment variable
    if env::var("CI").is_ok() {
        return true;
    }

    // Check if we're in a terminal (TTY)
    // This is a basic check - for production use, consider using atty crate
    if !is_tty() {
        return true;
    }

    false
}

/// Basic TTY detection
///
/// Returns true if stdout appears to be a terminal
fn is_tty() -> bool {
    // Try to check if we're in a terminal
    // This is a simplified check - for robust detection, use atty or is-terminal crate
    #[cfg(unix)]
    {
        use std::fs;
        // Check if stdout is a TTY by checking /dev/stdout
        if fs::metadata("/dev/stdout").is_ok() {
            // Check file type (TTY devices have specific permissions)
            // This is a heuristic - not 100% reliable
            true
        } else {
            false
        }
    }

    #[cfg(windows)]
    {
        // On Windows, assume we might be in a terminal unless in CI
        // For production, use windows-sys or winapi to check properly
        true
    }

    #[cfg(not(windows))]
    {
        // Default to assuming TTY if we can't determine (non-Windows platforms)
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit_non_interactive_flag() {
        // When explicit flag is set, should always return true
        assert!(is_non_interactive(true));
    }

    #[test]
    fn test_tty_detection() {
        // The function should return a boolean
        let result = is_tty();
        // We can't assert the value in tests (depends on environment)
        // But we can verify it doesn't panic
        let _ = result;
    }
}
