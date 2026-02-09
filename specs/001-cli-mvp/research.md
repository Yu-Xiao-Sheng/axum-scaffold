# Phase 0 Research: axum-app-create CLI Tool - Phase 1 MVP

**Date**: 2025-02-05
**Purpose**: Research and document technical decisions, best practices, and implementation strategies for the CLI tool
**Status**: Complete

## Overview

This document captures research findings for building a Rust-based CLI scaffolding tool for Axum web applications. All technical decisions are documented with rationale, alternatives considered, and implementation guidance.

## 1. CLI Framework Selection

### Decision: clap + inquire

**Chosen Framework**:
- `clap` (v4.x): Command-line argument parsing and help generation
- `inquire` (v0.7.x): Interactive prompts with rich UI features

**Rationale**:

**clap**:
- Industry standard for Rust CLI tools (used by ripgrep, fd, cargo itself)
- Derive-based API for type-safe argument parsing
- Excellent help text generation
- Supports subcommands, flags, options, and positional arguments
- Native support for argv parsing from environment variables

**inquire**:
- Modern alternative to dialoguto with better UX
- Supports multi-select, confirm, text, select, and custom prompts
- Built-in validation and type conversion
- Terminal compatibility (handles TERM=dumb for CI/automation)
- Better visual feedback than dialoguto

**Alternatives Considered**:

1. **dialoguto**: Older, less maintained, less intuitive API
2. **console** + **dialoguer**: Good but requires more manual work for validation
3. **promptly**: Less feature-rich than inquire
4. **Custom prompt implementation**: Too much work, reinventing the wheel

**Implementation Notes**:
- Use `clap` derive macros for CLI structure
- Use `inquire` only when running in interactive mode (check for TTY)
- Support non-interactive mode via environment variables and flags
- Respect `CI=1` environment variable to skip prompts

## 2. Template Engine Selection

### Decision: handlebars

**Chosen Framework**: `handlebars` (v5.x)

**Rationale**:
- Industry-standard templating language (familiar to many developers)
- Logic-less templates (prevents business logic in templates)
- Built-in helpers (if, each, unless, with)
- Custom helper support for project-specific needs
- Excellent error messages with line numbers
- Compile-time template verification possible with build script

**Alternatives Considered**:

1. **tera**: More powerful but adds too much logic (could lead to business logic in templates)
2. **askama**: Compile-time only, less flexible for embedded templates
3. **string replacement**: Too error-prone, no conditionals or loops
4. **minijinja**: Good Python-like syntax, but less mature for Rust CLI tools

**Implementation Notes**:
- Store templates as `.hbs` files in `src/template/templates/`
- Use `include_str!` macro to embed templates at compile time
- Template context includes: project_name, features (database, auth, logging, biz_error), author_name
- Pre-register custom helpers for cargo_toml_format, snake_case, kebab_case

## 3. Template Distribution Strategy

### Decision: Embedded Templates (Static Assets)

**Chosen Approach**: `include_str!` macro to embed templates in binary

**Rationale**:
- **Offline operation**: No network dependency after installation
- **Simple distribution**: Single binary via crates.io
- **Fast performance**: No file I/O needed at runtime
- **Version consistency**: Templates always match CLI version
- **Zero configuration**: No template path setup required

**Implementation Strategy**:
```rust
// Template loading at compile time
const CARGO_TOML_TEMPLATE: &str = include_str!("templates/single_mode/Cargo.toml.hbs");
const MAIN_RS_TEMPLATE: &str = include_str!("templates/single_mode/src/main.rs.hbs");
// ... etc for all templates
```

**Alternatives Considered**:

1. **External template repository** (Phase 2 feature):
   - Pros: Templates can be updated independently, supports community contributions
   - Cons: Requires network, adds complexity, version mismatch risk
   - **Decision**: Deferred to Phase 2

2. **Templates in separate crate**:
   - Pros: Easier testing of templates in isolation
   - Cons: Still requires network or crates.io dependency, adds complexity
   - **Decision**: Not needed for Phase 1 simplicity

3. **Runtime file system templates**:
   - Pros: Easy to edit templates during development
   - Cons: Distribution complexity, installation burden
   - **Decision**: Use `include_str!` with dev mode override (optional)

**Trade-offs**:
- Binary size: Each template adds to binary size (mitigated by reasonable template size)
- Update cycle: Template changes require new CLI version (acceptable for Phase 1)

## 4. Database Technology Selection

### Decision: PostgreSQL (Production) + SQLite (Development)

**Chosen Stack**:
- **Production**: PostgreSQL 14+ via `sqlx` with runtime connection pooling
- **Development**: SQLite via `sqlx` for zero-config local development
- **Migration Tool**: `sqlx-cli` for schema migrations

**Rationale**:

**PostgreSQL for Production**:
- Industry standard for production databases
- Excellent JSON support (for API responses)
- Strong ACID guarantees
- Connection pooling ecosystem (deadpool, bb8, or sqlx built-in)
- Cloud-native (AWS RDS, Google Cloud SQL, Azure Database)

**SQLite for Development**:
- Zero configuration (file-based database)
- No server installation required
- Fast for local development and testing
- Easy to reset (delete file)
- Sufficient for single-developer projects

**sqlx as Unified Interface**:
- Compile-time checked queries (via `sqlx-cli` macro)
- Same API for PostgreSQL and SQLite
- Async/await support (matches Axum)
- Built-in connection pooling
- Migration support

**Alternatives Considered**:

1. **sea-orm**: Higher-level ORM, but adds abstraction layer and slower performance
2. **diesel**: Excellent but compile-time only, less flexible for dynamic queries
3. **postgres + rusqlite**: Separate drivers, more complex code
4. **MongoDB**: NoSQL, not aligned with relational patterns for Axum

**Implementation Notes**:
- Use `sqlx::Postgres` for production, `sqlx::Sqlite` for development
- Switch via `DATABASE_URL` environment variable format detection
- Example `.env` files for both environments
- Connection pool configuration: min 1, max 10 connections
- Migration files in `migrations/` directory

## 5. Authentication Strategy

### Decision: JWT-based Authentication

**Chosen Approach**: JSON Web Tokens (JWT) with HS256 signing

**Implementation Components**:
- `jsonwebtoken` crate for token generation and validation
- `jsonwebtoken` decoding middleware for Axum
- Secret key via `JWT_SECRET` environment variable
- Login endpoint for token issuance
- Protected routes with authentication middleware
- Token expiration: 24 hours default

**Rationale**:
- **Stateless**: No server-side session storage required
- **Scalable**: Works across multiple instances (horizontal scaling)
- **Standard**: Industry-accepted, well-documented
- **Axum integration**: `tower-http` has JWT middleware
- **Simple**: Easier for beginners than session-based auth

**Alternatives Considered**:

1. **Session-based auth** (cookies):
   - Pros: Simpler revocation, more secure for web apps
   - Cons: Requires server-side storage (Redis), harder to scale
   - **Decision**: Overkill for Phase 1 MVP

2. **OAuth2/OIDC**:
   - Pros: Industry standard for third-party login
   - Cons: Too complex for beginners, requires external services
   - **Decision**: Deferred to Phase 2 or optional feature

3. **API keys**:
   - Pros: Simple for machine-to-machine auth
   - Cons: No user context, harder to manage
   - **Decision**: Can be added as optional feature later

**Security Considerations**:
- JWT secret must be at least 256 bits (32 bytes) for HS256
- Use `JWT_SECRET` environment variable (never hardcode)
- Validate token signature on every request
- Set appropriate expiration times
- Include user claims (user_id, username) in token payload

## 6. Logging and Observability

### Decision: tracing crate with tracing-subscriber

**Chosen Stack**:
- `tracing`: Core instrumentation framework
- `tracing-subscriber`: Log formatting and output
- `tracing-appender`: File rotation support (optional)
- Environment variable: `LOG_LEVEL` (trace, debug, info, warn, error)

**Rationale**:
- **Modern replacement** for log crate with structured logging
- **Async-aware**: Designed for async/await (matches Axum)
- **Span support**: Request tracing across async boundaries
- **Flexible formatting**: JSON, Pretty, Compact formats
- **Filter by module**: Fine-grained control (e.g., `sqlx=debug,axum=info`)
- **Production-ready**: Used by Tokio and Axum ecosystem

**Implementation Strategy**:
```rust
// Initialize in main()
tracing_subscriber::fmt()
    .with_max_level(level_from_env("LOG_LEVEL").unwrap_or(Level::INFO))
    .with_target(false) // Simplify output for beginners
    .init();
```

**Alternatives Considered**:

1. **log + env_logger**: Older, less feature-rich, no structured logging
2. **slog**: Good but less ergonomic API, less async-friendly
3. **fern**: Flexible but less modern than tracing

**Best Practices**:
- Use `info!` for normal operation (server started, request received)
- Use `warn!` for deprecation warnings and recoverable errors
- Use `error!` for errors that affect operation
- Use `debug!` for detailed diagnostic information
- Use `trace!` for very detailed tracing (disabled by default)
- Never log secrets (JWT_SECRET, passwords, tokens)

## 7. Error Handling Strategy

### Decision: thiserror + biz-error Integration (Optional)

**Two-Tier Approach**:

1. **Internal Errors** (CLI tool itself):
   - `thiserror` for domain-specific error types
   - `anyhow` for error context in application code
   - User-friendly error messages with suggestions

2. **Generated Project Errors** (when biz-error enabled):
   - `biz-error` crate for standardized error responses
   - YAML-based error code definitions
   - Procedural macro for type-safe error enums
   - Multilingual error messages (English, Chinese)

**Rationale for biz-error Integration**:
- **Standardized format**: Consistent JSON responses `{code, msg, data}`
- **Type-safe**: Compile-time error code generation
- **i18n support**: Multiple languages in single definition
- **Axum integration**: Automatic HTTP status mapping
- **Production-ready**: Teaches best practices for API error handling

**Alternatives for CLI Tool Errors**:
1. **Anyhow only**: Simpler but loses type safety
2. **Custom error enum**: More work, less idiomatic than thiserror
3. **Box<dyn Error>**: Too generic, loses context

**Implementation Notes**:
- CLI tool errors: Use `thiserror` derive macro
  ```rust
  #[derive(thiserror::Error, Debug)]
  enum CliError {
      #[error("Invalid project name '{0}': {1}")]
      InvalidName(String, String),
      #[error("Failed to create directory: {0}")]
      DirectoryError(#[from] std::io::Error),
  }
  ```
- Generated project errors (biz-error enabled):
  - Include `biz-errors.yaml` with common error codes
  - Use `#[derive(BizError)]` macro on error enums
  - Configure Axum error handler middleware

## 8. Project Structure Design

### Decision: Single Binary CLI with Modular Templates

**Chosen Structure**:
```
src/
├── main.rs              # Entry point, CLI setup
├── cli/                 # CLI interaction layer
│   ├── mod.rs
│   ├── prompts.rs       # Interactive prompts (inquire)
│   └── args.rs          # Arg parsing (clap derive)
├── template/            # Template engine
│   ├── mod.rs
│   ├── engine.rs        # Template rendering
│   ├── context.rs       # Template context struct
│   └── templates/       # Embedded .hbs files
├── generator/           # Project generation logic
│   ├── mod.rs
│   ├── project.rs       # Orchestrate generation
│   ├── validator.rs     # Validate project names
│   └── git.rs           # Git initialization
└── utils/               # Utilities
    ├── mod.rs
    └── rust_toolchain.rs # Rust/Cargo check
```

**Rationale**:
- **Separation of concerns**: Each module has single responsibility
- **Testability**: Modules can be unit tested in isolation
- **Extensibility**: Easy to add new features (templates, generators)
- **Idiomatic Rust**: Follows community conventions

**Generated Project Structure** (Single Package Mode):
```
<project-name>/
├── Cargo.toml           # Dependencies with pinned versions
├── .env.example         # Environment variables template
├── .gitignore           # Rust/Cargo standard
├── README.md            # Bilingual (EN/CN) quick start
├── biz_errors.yaml      # Error definitions (if biz-error enabled)
├── src/
│   ├── main.rs          # Server entry point
│   ├── lib.rs           # Library exports
│   ├── config.rs        # Configuration from env
│   ├── handlers/        # HTTP handlers
│   │   ├── mod.rs
│   │   ├── health.rs    # Health check endpoint
│   │   └── examples.rs  # Example endpoints
│   ├── models/          # Data models (if auth/db enabled)
│   │   ├── mod.rs
│   │   └── user.rs      # User model (if auth)
│   └── middleware/      # Axum middleware
│       ├── mod.rs
│       └── auth.rs      # Auth middleware (if auth enabled)
├── migrations/          # SQL migrations (if db enabled)
│   └── 001_initial.sql
└── tests/               # Integration tests
    └── integration_test.rs
```

**Alternatives Considered**:
1. **Flat structure** (all in src/): Too messy, hard to navigate
2. **Workspace mode** (multiple packages): Explicitly out of scope for Phase 1
3. **Framework-based structure** (controller/): Not idiomatic for Rust

## 9. Configuration Management

### Decision: Environment Variables + .env File

**Chosen Approach**:
- `.env` file for local development (not committed)
- `.env.example` file as template (committed)
- `dotenvy` crate for loading .env files
- Environment variables override .env values
- 12-factor app principles

**Configuration Variables**:
```bash
# Server Configuration
PORT=8080                  # Default server port
HOST=127.0.0.1            # Bind address

# Database (if db enabled)
DATABASE_URL=postgresql://...  # Production
DATABASE_URL=sqlite://...      # Development

# Authentication (if auth enabled)
JWT_SECRET=your-secret-key-min-32-chars

# Logging
LOG_LEVEL=info            # trace, debug, info, warn, error

# Application
APP_NAME=my-app           # Generated app name
```

**Rationale**:
- **Simple**: No complex config files or parsing
- **Container-native**: Environment variables work in Docker/K8s
- **12-factor compliant**: Industry best practice
- **Secure**: Secrets in environment, not in code
- **Flexible**: Easy to override in different environments

**Alternatives Considered**:
1. **TOML config file**: More complex, harder to use in containers
2. **YAML config file**: Overkill for simple config, YAML parsing errors
3. **JSON config file**: No comments allowed, harder to edit
4. **Config crate**: Good but adds complexity for Phase 1

**Implementation Notes**:
- Use `dotenvy::dotenv()` in main() before loading config
- Use `std::env::var()` to read variables
- Provide clear error messages for missing required variables
- Include validation in generated projects

## 10. Performance Optimization Strategy

### Performance Goals and Achievability

**Targets from SC-005, SC-006, SC-008**:
- CLI startup: < 100ms
- Generation time: < 10s
- Binary size: < 10MB
- Memory usage: < 50MB RSS

**Achievability Analysis**:

1. **CLI Startup < 100ms**: **ACHIEVABLE**
   - clap argument parsing is fast (< 10ms)
   - Template embedding has no runtime overhead
   - No heavy initialization before first user interaction
   - **Strategy**: Lazy-load expensive operations

2. **Generation Time < 10s**: **ACHIEVABLE**
   - Template rendering is fast (handlebars ~100ms per file)
   - File I/O is the bottleneck (SSD ~100MB/s)
   - Typical project ~30 files × 1KB = 30KB
   - Estimated time: 2-3 seconds for file operations
   - **Strategy**: Minimize file operations, parallel writes where possible

3. **Binary Size < 10MB**: **CHALLENGING but ACHIEVABLE**
   - Rust binaries typically 5-15MB for CLI tools
   - Templates embedded add size (~500KB estimated)
   - **Optimization strategies**:
     - Use `lto = true` in Cargo.toml (Link-Time Optimization)
     - Use `codegen-units = 1` for better optimization
     - Strip symbols: `cargo strip`
     - Use `--release` builds only
   - **Fallback**: Document that binary size may exceed 10MB on some platforms

4. **Memory Usage < 50MB RSS**: **ACHIEVABLE**
   - Rust has minimal runtime overhead
   - Templates are in binary (not heap allocated)
   - File I/O uses buffered writers
   - **Strategy**: Stream templates, don't load all in memory

**Optimization Techniques**:

1. **Lazy Template Loading**:
   ```rust
   // Instead of loading all templates at startup
   // Load on-demand and cache
   fn get_template(name: &str) -> &'static str {
       match name {
           "Cargo.toml" => CARGO_TOML_TEMPLATE,
           "main.rs" => MAIN_RS_TEMPLATE,
           // ...
       }
   }
   ```

2. **Parallel File Operations** (where safe):
   ```rust
   // Use rayon for parallel template rendering (if needed)
   use rayon::prelude::*;
   files.par_iter().for_each(|file| render_and_write(file));
   ```

3. **Efficient String Handling**:
   - Use `Cow<str>` to avoid allocations when possible
   - Use `String::with_capacity()` for known template sizes
   - Avoid cloning large strings

4. **Build Optimizations**:
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   strip = true
   opt-level = "z"  # Optimize for size
   ```

**Measurement Strategy**:
- Use `hyperfine` for benchmarking CLI startup
- Use `time` command for generation time
- Use `ls -lh` for binary size
- Use `/usr/bin/time -v` for memory usage (RSS)

## 11. Git Integration

### Decision: Automatic Git Initialization

**Chosen Approach**:
- Run `git init` to create repository
- Create `.gitignore` with Rust/Cargo standard patterns
- Create initial commit with message: "Initial project generated by axum-app-create"
- Use `git2` crate for programmatic Git operations

**Rationale**:
- **Instant version control**: Users get Git repo immediately
- **Prevent mistakes**: .gitignore prevents committing generated files
- **Best practice**: Teaches beginners to use Git from day one
- **Easy undo**: First commit allows easy rollback

**.gitignore Patterns**:
```gitignore
# Rust
/target/
**/*.rs.bk
Cargo.lock

# Environment
.env
.env.local

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
```

**Alternatives Considered**:
1. **Skip Git initialization**: Faster but teaches bad practices
2. **Use subprocess (git command)**: Simpler but requires git in PATH
3. **git2 crate**: More robust, cross-platform, no external dependency

**Implementation Notes**:
```rust
use git2::{Repository, Signature, Time, Oid};

fn init_git(project_dir: &Path) -> Result<()> {
    // Initialize repository
    let repo = Repository::init(project_dir)?;

    // Create .gitignore
    fs::write(project_dir.join(".gitignore"), GITIGNORE_CONTENT)?;

    // Add all files
    let mut index = repo.index()?;
    index.add_all(["*"], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create initial commit
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = Signature::now("Generator", "generator@axum-app-create")?;
    let parent_commit = None;
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial project generated by axum-app-create",
        &tree,
        parent_commit.iter().collect::<Vec<_>>().as_slice(),
    )?;

    Ok(())
}
```

## 12. Cross-Platform Compatibility

### Decision: Support Linux, macOS, Windows

**Chosen Approach**:
- Use Rust's cross-platform standard library
- Avoid platform-specific code where possible
- Use `dirs` crate for cross-platform directory paths
- Test on all three platforms via GitHub Actions

**Platform-Specific Considerations**:

1. **Line Endings**:
   - Git handles LF vs CRLF conversion
   - Templates use LF (Unix style)
   - `.gitattributes` ensures consistency

2. **Path Separators**:
   - Use `Path` and `PathBuf` from std
   - Never hardcode "/" or "\\"
   - Use `path.join("subdir")` for concatenation

3. **Permissions**:
   - Unix: Execute permissions for scripts (if any)
   - Windows: No execute permission concept
   - Solution: Avoid shell scripts in generated projects

4. **Terminal**:
   - `inquire` handles TERM=dumb for non-TTY environments
   - Windows: Use Windows 10+ terminal for ANSI support
   - Fallback: Detect TTY and use non-interactive mode

**Implementation Notes**:
- Use `cfg(unix)` and `cfg(windows)` only when necessary
- Prefer platform-agnostic APIs
- Test on real hardware, not just CI

## 13. Testing Strategy

### Decision: Three-Tier Testing

**Testing Levels**:

1. **Unit Tests** (CLI tool):
   - Test individual functions in isolation
   - Mock file I/O using temp directories
   - Test template rendering logic
   - Test validation logic (project names, etc.)

2. **Integration Tests** (CLI tool):
   - Test full CLI invocation with various flags
   - Test project generation end-to-end
   - Verify generated projects compile and pass tests
   - Test error conditions (invalid names, existing dirs)

3. **Generated Project Tests** (in generated projects):
   - Include example integration tests
   - Test health check endpoint
   - Test error handling endpoints
   - Demonstrate testing best practices

**Testing Tools**:
- `cargo test` for unit and integration tests
- `tempfile` crate for temp directories in tests
- `assert_cmd` for testing CLI binaries
- `assert_fs` for file system assertions

**Example Test Structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_project_name_valid() {
        assert!(validate_project_name("my-app").is_ok());
        assert!(validate_project_name("my_app").is_ok());
    }

    #[test]
    fn test_validate_project_name_invalid() {
        assert!(validate_project_name("123invalid").is_err());
        assert!(validate_project_name("std").is_err()); // Keyword
    }

    #[test]
    fn test_generate_project() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("test-project");

        generate_project(&project_dir, &ProjectConfig::default())
            .unwrap();

        assert!(project_dir.join("Cargo.toml").exists());
        assert!(project_dir.join("src/main.rs").exists());
    }
}
```

## 14. Documentation Strategy

### Decision: Bilingual README (English + Chinese)

**Chosen Approach**:
- Single README.md with both languages
- English section first, followed by Chinese section
- Clear separators between languages
- Consistent structure across both languages

**README Structure**:
```markdown
# My Axum App

Generated by [axum-app-create](https://github.com/...).

## English

### Quick Start
\`\`\`bash
cargo run
\`\`\`

### Project Structure
...

### Features
...

### Configuration
...

### Testing
\`\`\`bash
cargo test
\`\`\`

---

## 中文

### 快速开始
\`\`\`bash
cargo run
\`\`\`

### 项目结构
...

### 功能特性
...

### 配置
...

### 测试
\`\`\`bash
cargo test
\`\`\`
```

**Rationale**:
- **Accessibility**: Supports both English and Chinese-speaking developers
- **Single file**: Easier to maintain than separate README files
- **Aligned with biz-error**: Consistent i18n approach
- **Community**: Expands potential user base

**Implementation Notes**:
- Use professional translation (not machine translation)
- Keep both sections in sync when updating
- Use clear section headers for navigation

## 15. Dependency Version Strategy

### Decision: Pin to Specific Minor Versions

**Chosen Approach**:
- Pin dependencies to specific versions in Cargo.toml
- Use `=x.y.z` format for critical dependencies
- Update dependencies via separate maintenance workflow

**Rationale**:
- **Reproducibility**: Generated projects work consistently
- **Stability**: Avoid breaking changes from dependency updates
- **Debugging**: Known versions simplify issue diagnosis

**Example Cargo.toml**:
```toml
[dependencies]
axum = "=0.7.4"
tokio = { version = "=1.35.0", features = ["full"] }
tracing = "=0.1.40"
tracing-subscriber = { version = "=0.3.18", features = ["env-filter"] }
sqlx = { version = "=0.7.3", features = ["runtime-tokio-rustls", "postgres", "sqlite"] }
```

**Alternatives Considered**:
1. **Carets (^)**: Allows automatic updates, risk of breaking changes
2. **Tildes (~)**: Allows patch updates, still risk of bugs
3. **Latest**: Too unpredictable, causes reproducibility issues

**Maintenance Strategy**:
- Document dependency versions in plan.md
- Update versions quarterly or as needed
- Test thoroughly before updating
- Follow semver (major.minor.patch)

## 16. Distribution Strategy

### Decision: crates.io Publication

**Chosen Approach**:
- Publish via crates.io as `axum-app-create`
- Installation: `cargo install axum-app-create`
- Follow semver for versioning
- Use semantic release (v1.0.0 for MVP)

**Rationale**:
- **Standard Rust pattern**: Familiar to Rust developers
- **Simple**: No custom infrastructure needed
- **Discovery**: crates.io provides search and discovery
- **Trust**: Official Rust package registry

**Release Process**:
1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Tag release in Git: `git tag v1.0.0`
4. Publish: `cargo publish`
5. GitHub release with binaries (optional)

**Alternatives Considered**:
1. **Homebrew tap**: macOS only, more maintenance
2. **AUR (Arch User Repository)**: Arch only, more maintenance
3. **GitHub releases**: Manual per-platform builds
4. **Docker image**: Overkill for CLI tool

## Summary of Technical Decisions

| Area | Decision | Key Alternatives Rejected |
|------|----------|---------------------------|
| CLI Framework | clap + inquire | dialoguto, console, custom |
| Template Engine | handlebars | tera, askama, string replacement |
| Template Distribution | Embedded (include_str!) | External repo (Phase 2) |
| Database | PostgreSQL + SQLite via sqlx | sea-orm, diesel, MongoDB |
| Authentication | JWT (jsonwebtoken) | Session-based, OAuth2 |
| Logging | tracing + tracing-subscriber | log + env_logger, slog |
| Error Handling | thiserror + biz-error | anyhow only, custom enum |
| Project Structure | Single binary CLI | Workspace mode (Phase 2) |
| Configuration | Environment variables + .env | TOML, YAML, JSON files |
| Git Integration | Automatic init (git2 crate) | subprocess, skip |
| Testing | Three-tier (unit/integration/example) | Single-tier, external tests |
| Documentation | Bilingual README (EN/CN) | English only, separate files |
| Dependencies | Pin to specific versions | Carets (^), tildes (~), latest |
| Distribution | crates.io | Homebrew, AUR, Docker releases |

## Open Questions and Future Research

1. **Template Hot-Reloading** (Phase 2):
   - How to support live template editing during development?
   - Consider `notify` crate for file watching

2. **Workspace Mode** (Phase 2):
   - Research multi-package project generation
   - Consider workspace member configuration

3. **Template Marketplace** (Phase 2):
   - Research community template sharing
   - Consider git-based template repository

4. **Performance Profiling**:
   - Profile actual generation time with real templates
   - Optimize if >10s target not met

5. **Binary Size Optimization**:
   - If >10MB, consider conditional compilation
   - Research UPX compression for release builds

## References and Resources

- **Axum Documentation**: https://docs.rs/axum/
- **clap Documentation**: https://docs.rs/clap/
- **handlebars Documentation**: https://docs.rs/handlebars/
- **sqlx Documentation**: https://docs.rs/sqlx/
- **tracing Documentation**: https://docs.rs/tracing/
- **biz-error Documentation**: https://docs.rs/biz-error/
- **Rust CLI Book**: https://rust-cli.github.io/book/
- **12-Factor App**: https://12factor.net/

---

**Next Steps**: Proceed to Phase 1 (Data Model, API Contracts, Quick Start Guide)
