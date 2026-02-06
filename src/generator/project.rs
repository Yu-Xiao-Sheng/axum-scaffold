// Project generation orchestration
//
// This module handles the main project generation logic.

use crate::config::ProjectConfig;
use crate::error::{CliError, Result};
use crate::template::context::TemplateContext;
use crate::template::engine::TemplateEngine;
use crate::template::templates::get_single_mode_templates;
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
/// * `interactive` - Whether to prompt for user input on conflicts
///
/// # Returns
/// * `Ok(())` if generation succeeded
/// * `Err(CliError)` if generation failed
pub fn generate_project(project_dir: &Path, config: &ProjectConfig, interactive: bool) -> Result<()> {
    // Validate project directory doesn't exist
    if project_dir.exists() {
        // In non-interactive mode, fail immediately
        if !interactive {
            return Err(CliError::Generation(format!(
                "❌ 目录已存在 / Directory already exists: '{}'\n\n\
                 💡 修复建议 / Fix:\n\
                 - 删除现有目录 / Remove existing directory: rm -rf {}\n\
                 - 使用不同的名称 / Use a different name\n\
                 - 如果确认要覆盖，请使用 --force 标志 / If you want to overwrite, use --force flag",
                project_dir.display(),
                project_dir.display()
            )));
        }

        // In interactive mode, prompt for action
        println!("\n⚠️  警告 / Warning: 目录已存在 / Directory already exists: '{}'", project_dir.display());
        println!("📁 位置 / Location: {}", project_dir.display());
        println!();

        // Use inquire for user choice
        let options = vec![
            "覆盖 / Overwrite - Delete existing directory and regenerate",
            "取消 / Cancel - Abort project generation",
            "重命名 / Rename - Keep existing directory, use different name",
        ];

        let ans = inquire::Select::new("请选择操作 / Choose an action:", options)
            .prompt()?;

        match ans {
            "覆盖 / Overwrite - Delete existing directory and regenerate" => {
                println!("🗑️  正在删除现有目录 / Deleting existing directory...");
                std::fs::remove_dir_all(project_dir)?;
                println!("✓ 已删除 / Deleted");
            }
            "取消 / Cancel - Abort project generation" => {
                println!("❌ 已取消 / Aborted");
                return Err(CliError::Generation("项目生成已取消 / Project generation cancelled by user".to_string()));
            }
            "重命名 / Rename - Keep existing directory, use different name" => {
                println!("❌ 请使用不同的项目名称重新运行 / Please run again with a different project name");
                return Err(CliError::Generation("请使用不同的项目名称 / Please use a different project name".to_string()));
            }
            _ => {
                return Err(CliError::Generation("无效选择 / Invalid choice".to_string()));
            }
        }
    }

    println!("\n🚀 正在创建项目 / Creating project: {}", config.project_name);
    println!("📁 位置 / Location: {}", project_dir.display());

    // Create project directory
    std::fs::create_dir_all(project_dir)?;

    // Create template context
    let ctx = TemplateContext::from_config(config);

    // Create template engine
    let engine = TemplateEngine::new();

    // Get templates
    let templates = get_single_mode_templates();

    // Render and write each template
    println!("\n📝 Generating files:");

    for (name, template_file) in templates {
        // Render template
        let rendered = engine.render_template(name, template_file.content, &ctx)?;

        // Write file
        write_file(project_dir, template_file.path, &rendered)?;

        println!("  ✓ Created {}", template_file.path);
    }

    // Initialize git repository
    println!("\n🔧 Initializing git repository...");
    super::git::init_git_repo(project_dir)?;

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

/// Get success message for project generation
pub fn get_success_message(project_dir: &Path, project_name: &str) -> String {
    format!(
        r#"
✨ Project '{}' created successfully!

📂 Location: {}

🚀 Next steps:

  $ cd {}
  $ cargo run

Then test the health endpoint:
  $ curl http://127.0.0.1:8080/health

Happy hacking! 🦀
"#,
        project_name, project_dir.display(), project_name
    )
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
    fn test_generate_project_creates_all_files() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("my-test-app");
        let mut config = ProjectConfig::default();
        config.project_name = "my-test-app".to_string();

        let result = generate_project(&project_dir, &config, false);

        if let Err(e) = &result {
            eprintln!("Generation error: {:?}", e);
        }

        assert!(result.is_ok());
        assert!(project_dir.exists());

        // Verify key files were created
        assert!(project_dir.join("Cargo.toml").exists());
        assert!(project_dir.join("src/main.rs").exists());
        assert!(project_dir.join("src/lib.rs").exists());
        assert!(project_dir.join("src/config.rs").exists());
        assert!(project_dir.join("src/handlers/health.rs").exists());
        assert!(project_dir.join(".env.example").exists());
        assert!(project_dir.join(".gitignore").exists());
        assert!(project_dir.join("README.md").exists());
    }
}
