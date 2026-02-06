// Project generation orchestration
//
// This module handles the main project generation logic.

use crate::config::ProjectConfig;
use crate::error::{CliError, Result};
use crate::template::context::TemplateContext;
use crate::template::engine::TemplateEngine;
use std::path::Path;

/// Generate a new project with the given configuration
///
/// This function orchestrates the entire project generation process:
/// - Creates project directory
/// - Renders and writes all template files
/// - Initializes git repository
///
/// # Arguments
/// * `project_dir` - Path where the project should be created
/// * `config` - Project configuration
///
/// # Returns
/// * `Ok(())` if generation succeeded
/// * `Err(CliError)` if generation failed
pub fn generate_project(project_dir: &Path, config: &ProjectConfig) -> Result<()> {
    // Validate project directory doesn't exist
    if project_dir.exists() {
        return Err(CliError::Generation(format!(
            "Directory '{}' already exists",
            project_dir.display()
        )));
    }

    // Create project directory
    std::fs::create_dir_all(project_dir)?;

    // Create template context
    let ctx = TemplateContext::from_config(config);

    // Create template engine
    let engine = TemplateEngine::new();

    // For now, just create basic structure
    // Templates will be added in User Story 1
    create_basic_structure(project_dir, &ctx, &engine)?;

    // Initialize git repository
    super::git::init_git_repo(project_dir)?;

    Ok(())
}

/// Create basic project structure
///
/// This is a minimal implementation - full template rendering will be added in User Story 1
fn create_basic_structure(
    project_dir: &Path,
    _ctx: &TemplateContext,
    _engine: &TemplateEngine,
) -> Result<()> {
    // Create src directory
    let src_dir = project_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;

    // For now, just create placeholder files
    // Full templates will be implemented in User Story 1
    println!("✓ Created project structure at: {}", project_dir.display());
    println!("✓ Created src/ directory");

    Ok(())
}

/// Write a file to the project directory
///
/// # Arguments
/// * `project_dir` - Project root directory
/// * `relative_path` - Relative path from project root (e.g., "src/main.rs")
/// * `content` - File content to write
///
/// # Returns
/// * `Ok(())` if write succeeded
/// * `Err(CliError)` if write failed
pub fn write_file(project_dir: &Path, relative_path: &str, content: &str) -> Result<()> {
    let file_path = project_dir.join(relative_path);

    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write file
    std::fs::write(&file_path, content)?;

    Ok(())
}

/// Ensure a directory exists in the project
///
/// # Arguments
/// * `project_dir` - Project root directory
/// * `relative_path` - Relative path to directory (e.g., "src/handlers")
///
/// # Returns
/// * `Ok(())` if directory exists or was created
/// * `Err(CliError)` if directory creation failed
pub fn ensure_dir(project_dir: &Path, relative_path: &str) -> Result<()> {
    let dir_path = project_dir.join(relative_path);
    std::fs::create_dir_all(dir_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let content = "Hello, World!";

        let result = write_file(temp_dir.path(), "test.txt", content);

        assert!(result.is_ok());
        assert!(temp_dir.path().join("test.txt").exists());
    }

    #[test]
    fn test_ensure_dir() {
        let temp_dir = TempDir::new().unwrap();

        let result = ensure_dir(temp_dir.path(), "nested/dir/test");

        assert!(result.is_ok());
        assert!(temp_dir.path().join("nested/dir/test").exists());
    }

    #[test]
    fn test_generate_project_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("my-test-app");
        let config = ProjectConfig::default();

        let result = generate_project(&project_dir, &config);

        assert!(result.is_ok());
        assert!(project_dir.exists());
        assert!(project_dir.join("src").exists());
    }
}
