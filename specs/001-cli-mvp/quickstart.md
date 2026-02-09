# Developer Quick Start Guide: axum-app-create CLI Tool

**Date**: 2025-02-05
**Audience**: Developers working on the axum-app-create CLI tool
**Purpose**: Get up and running with development workflow quickly

## Prerequisites

Before starting, ensure you have:

- **Rust toolchain**: Rust 1.75+ (use `rustup update`)
- **Git**: For version control
- **Editor**: VS Code (recommended) or your preferred editor
- **Terminal**: Bash, Zsh, or Fish

## Quick Start (5 Minutes)

### 1. Clone and Build

```bash
# Clone the repository
git clone https://github.com/your-org/axum-app-create.git
cd axum-app-create

# Build the CLI tool (debug mode)
cargo build

# Or build optimized release binary
cargo build --release
```

### 2. Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_validate_project_name
```

### 3. Try the CLI (Development Mode)

```bash
# Run directly from cargo
cargo run -- --help

# Generate a test project interactively
cargo run -- my-test-app

# Generate with flags (non-interactive)
cargo run -- --project-name my-app \
    --database both \
    --auth \
    --biz-error \
    --log-level info
```

### 4. Test Generated Project

```bash
# Navigate to generated project
cd my-test-app

# Run the generated Axum server
cargo run

# In another terminal, test the endpoint
curl http://127.0.0.1:8080/health

# Run tests in generated project
cargo test
```

## Development Workflow

### Directory Structure

```
axum-app-create/
├── src/                      # CLI tool source
│   ├── main.rs               # Entry point
│   ├── cli/                  # CLI interaction layer
│   ├── template/             # Template engine
│   ├── generator/            # Project generation logic
│   └── utils/                # Utilities
├── templates/                # Project templates (not source, embedded)
├── tests/                    # Integration tests
│   ├── integration/          # Integration tests
│   └── fixtures/             # Test fixtures
└── specs/                    # Feature specifications
    └── 001-cli-mvp/          # Current feature
        ├── spec.md           # Feature specification
        ├── plan.md           # Implementation plan
        ├── research.md       # Technical research
        └── data-model.md     # Data model
```

### Code Organization

**CLI Layer** (`src/cli/`):
- `mod.rs`: Module exports
- `args.rs`: Command-line argument parsing with clap
- `prompts.rs`: Interactive prompts with inquire

**Template Engine** (`src/template/`):
- `mod.rs`: Module exports
- `engine.rs`: Handlebars template rendering
- `context.rs`: Template context construction
- `templates/`: Embedded .hbs files (not in source tree)

**Generator** (`src/generator/`):
- `mod.rs`: Module exports
- `project.rs`: Orchestrate project generation
- `validator.rs`: Project name and config validation
- `git.rs`: Git initialization (git init, .gitignore, commit)

**Utils** (`src/utils/`):
- `mod.rs`: Module exports
- `rust_toolchain.rs`: Rust/Cargo installation check

### Common Development Tasks

#### 1. Add a New CLI Flag

Edit `src/cli/args.rs`:

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "axum-app-create")]
#[command(about = "Scaffold a new Axum web application", long_about = None)]
struct CliArgs {
    /// Project name
    #[arg(short, long)]
    project_name: Option<String>,

    /// Enable database support
    #[arg(long, value_name = "DATABASE")]
    database: Option<String>,

    /// Your new flag here
    #[arg(long, value_name = "VALUE")]
    new_flag: Option<String>,

    /// Non-interactive mode
    #[arg(long)]
    non_interactive: bool,
}
```

#### 2. Add a New Interactive Prompt

Edit `src/cli/prompts.rs`:

```rust
use inquire::{Text, Select, Confirm};

pub fn prompt_new_flag() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let prompt = Confirm::new("Enable new feature?")
        .with_default(false)
        .with_help_string("This enables the new feature");

    if prompt.prompt()? {
        let value = Text::new("Enter value:")
            .with_default("default_value")
            .prompt()?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}
```

#### 3. Add a New Template File

1. Create template file: `src/template/templates/single_mode/path/to/file.ext.hbs`
2. Embed in `src/template/templates/mod.rs`:

```rust
pub const YOUR_TEMPLATE: &str = include_str!("single_mode/path/to/file.ext.hbs");
```

3. Add to template list in `src/generator/project.rs`:

```rust
use crate::template::templates::YOUR_TEMPLATE;

fn write_templates(project_dir: &Path, ctx: &TemplateContext) -> Result<()> {
    // ... existing templates ...

    // Your new template
    let template = Handlebars::new();
    let rendered = template.render_template(YOUR_TEMPLATE, ctx)?;
    fs::write(project_dir.join("path/to/file.ext"), rendered)?;

    Ok(())
}
```

4. Use variables in template:

```handlebars
# {{project_name}} - {{description}}

Generated by axum-app-create

Author: {{author_name}}
Year: {{year}}

{{#if features.database}}
Database support enabled: {{database.option}}
{{/if}}
```

#### 4. Add a New Optional Feature

1. Update `src/cli/mod.rs`:

```rust
#[derive(Debug, Clone, Default)]
pub struct FeatureSet {
    // ... existing features ...
    pub new_feature: bool,
}
```

2. Add prompt in `src/cli/prompts.rs`:

```rust
pub fn prompt_features() -> Result<FeatureSet, Box<dyn std::error::Error>> {
    let mut features = FeatureSet::default();

    // ... existing prompts ...

    features.new_feature = Confirm::new("Enable new feature?")
        .with_default(false)
        .prompt()?;

    Ok(features)
}
```

3. Update template context to include feature flag:

```rust
pub struct TemplateContext {
    // ... existing fields ...
    pub new_feature_config: Option<NewFeatureConfig>,
}
```

4. Create feature-specific templates in `src/template/templates/new_feature/`

#### 5. Test Template Rendering

```bash
# Generate a test project
cargo run -- test-app

# Inspect generated files
cat test-app/src/main.rs

# Build generated project to verify it compiles
cd test-app
cargo build
cargo test
```

#### 6. Debug Template Issues

```rust
// In src/template/engine.rs
use handlebars::Handlebars;

pub fn render_template(template: &str, ctx: &TemplateContext) -> Result<String> {
    let mut handlebars = Handlebars::new();

    // Enable strict mode (catch undefined variables)
    handlebars.set_strict_mode(true);

    // Render with error handling
    handlebars
        .render_template(template, ctx)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name() {
        assert!(validate_project_name("my-app").is_ok());
        assert!(validate_project_name("123invalid").is_err());
        assert!(validate_project_name("std").is_err()); // Reserved keyword
    }

    #[test]
    fn test_template_context() {
        let config = ProjectConfig {
            project_name: "my-app".to_string(),
            ..Default::default()
        };
        let ctx = TemplateContext::from_config(&config);

        assert_eq!(ctx.project_name, "my-app");
        assert_eq!(ctx.project_name_snake, "my_app");
        assert_eq!(ctx.project_name_pascal, "MyApp");
    }
}
```

### Integration Tests

```rust
// tests/integration/generation_tests.rs

use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_generate_basic_project() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("test-project");

    let config = ProjectConfig {
        project_name: "test-project".to_string(),
        ..Default::default()
    };

    generate_project(&project_dir, &config).unwrap();

    assert!(project_dir.join("Cargo.toml").exists());
    assert!(project_dir.join("src/main.rs").exists());

    // Verify generated project compiles
    let status = std::process::Command::new("cargo")
        .args(["check", "--manifest-path", project_dir.join("Cargo.toml").to_str().unwrap()])
        .status()
        .unwrap();

    assert!(status.success());
}
```

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Specific test
cargo test test_generate_basic_project

# With output
cargo test -- --nocapture

# With backtrace
RUST_BACKTRACE=1 cargo test
```

## Performance Profiling

### Measure Generation Time

```bash
# Use hyperfine for accurate measurements
cargo install hyperfine

hyperfine 'cargo run -- test-app'

# Or use time command
/usr/bin/time -v cargo run -- test-app
```

### Measure Binary Size

```bash
# Build release
cargo build --release

# Check binary size
ls -lh target/release/axum-app-create

# Strip symbols to reduce size
cargo strip
ls -lh target/release/axum-app-create
```

### Measure Memory Usage

```bash
# Use /usr/bin/time for memory stats
/usr/bin/time -v cargo run -- test-app 2>&1 | grep "Maximum resident set size"
```

### Profile CPU Usage

```bash
# Use flamegraph for visualization
cargo install flamegraph

cargo flamegraph --bin axum-app-create -- test-app

# View flamegraph
open flamegraph.svg
```

## Debugging

### Common Issues

**1. Template Variable Not Found**

```
Error: Variable 'foo' not found in template
```

**Solution**: Ensure variable is in `TemplateContext`:

```rust
pub struct TemplateContext {
    pub foo: String,  // Add this
    // ... other fields
}
```

**2. Git Initialization Fails**

```
Error: Failed to initialize git repository
```

**Solution**: Ensure git is installed and in PATH:

```bash
which git
git --version
```

**3. Project Already Exists**

```
Error: Directory 'my-app' already exists
```

**Solution**: Remove existing directory or use different name:

```bash
rm -rf my-app
cargo run -- my-app
```

### Debug Mode

Enable debug logging:

```bash
RUST_LOG=debug cargo run -- my-app
```

Add logging to code:

```rust
use tracing::{debug, info, warn};

debug!("Generating project at: {:?}", project_dir);
info!("Project name: {}", config.project_name);
warn!("Database feature not enabled");
```

## Publishing

### Prepare Release

```bash
# Update version in Cargo.toml
vim Cargo.toml

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build release
cargo build --release

# Check binary size
ls -lh target/release/axum-app-create
```

### Publish to crates.io

```bash
# Login to crates.io
cargo login

# Publish (dry run first)
cargo publish --dry-run

# Publish for real
cargo publish
```

### Create Git Release

```bash
# Tag release
git tag v1.0.0
git push origin v1.0.0

# Create GitHub release with binary attachments
gh release create v1.0.0 \
    --title "v1.0.0: Initial MVP Release" \
    --notes "See CHANGELOG.md for details" \
    target/release/axum-app-create
```

## Contributing

### Code Style

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy -- -D warnings

# Run all checks
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

### Commit Convention

Follow Conventional Commits:

```
feat: add PostgreSQL support
fix: resolve template rendering issue
docs: update quickstart guide
test: add integration test for project generation
refactor: simplify template engine
perf: optimize template rendering
chore: update dependencies
```

### Pull Request Process

1. Fork repository
2. Create feature branch: `git checkout -b feature/my-feature`
3. Make changes and test: `cargo test`
4. Format code: `cargo fmt`
5. Commit changes: `git commit -m "feat: add my feature"`
6. Push to fork: `git push origin feature/my-feature`
7. Create pull request on GitHub

## Additional Resources

- **Axum Documentation**: https://docs.rs/axum/
- **clap Documentation**: https://docs.rs/clap/
- **handlebars Documentation**: https://docs.rs/handlebars/
- **inquire Documentation**: https://docs.rs/inquire/
- **Rust CLI Book**: https://rust-cli.github.io/book/

## FAQ

**Q: How do I regenerate a project with different settings?**
A: Delete the existing directory and run the CLI again with new settings.

**Q: Can I update an existing project?**
A: Not in Phase 1. You'll need to manually merge changes or regenerate.

**Q: How do I add custom templates?**
A: For Phase 1, templates are embedded. Add new .hbs files to `src/template/templates/` and rebuild.

**Q: The binary is too large (>10MB). What can I do?**
A: Use LTO and strip symbols:
```toml
[profile.release]
lto = true
codegen-units = 1
strip = true
```

**Q: How do I test changes without reinstalling?**
A: Use `cargo run --` instead of the installed binary.

---

**Happy hacking!** 🦀

For questions or issues, please open a GitHub issue.
