# CLI API Contract: create-axum-app

**Version**: 1.0.0
**Last Updated**: 2025-02-05
**Purpose**: Define the CLI interface, commands, flags, and behavior

## Overview

The create-axum-app CLI tool provides a command-line interface for scaffolding Axum web applications. This contract defines the complete API surface including commands, flags, interactive prompts, exit codes, and error handling.

## Command Structure

### Basic Syntax

```bash
create-axum-app [PROJECT_NAME] [OPTIONS]
```

### Usage Examples

```bash
# Interactive mode (default)
create-axum-app

# Non-interactive with all flags
create-axum-app my-app \
  --database both \
  --auth \
  --biz-error \
  --log-level info \
  --author "John Doe" \
  --non-interactive

# Partial flags (prompt for remaining)
create-axum-app my-app --database postgresql

# Show help
create-axum-app --help

# Show version
create-axum-app --version
```

## Command-Line Options

### Positional Arguments

| Argument | Type | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `PROJECT_NAME` | string | No | `my-axum-app` | Name of the project to generate |

### Flags

| Flag | Short | Type | Required | Default | Description |
|------|-------|------|----------|---------|-------------|
| `--help` | `-h` | flag | No | - | Display help message and exit |
| `--version` | `-V` | flag | No | - | Display version information and exit |
| `--database` | `-d` | string | No | `none` | Database support: `none`, `postgresql`, `sqlite`, `both` |
| `--auth` | `-a` | flag | No | `false` | Enable JWT authentication |
| `--biz-error` | `-b` | flag | No | `false` | Enable business error handling |
| `--log-level` | `-l` | string | No | `info` | Logging level: `trace`, `debug`, `info`, `warn`, `error` |
| `--author` | flag | No | - | Author name for generated project |
| `--non-interactive` | flag | No | `false` | Disable interactive prompts (fail if required values missing) |

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `CREATE_AXUM_APP_NON_INTERACTIVE` | No | `false` | Force non-interactive mode |
| `CREATE_AXUM_APP_DEFAULT_AUTHOR` | No | (git config) | Default author name |
| `CI` | No | - | Automatically enable non-interactive mode |

## Interactive Prompts

When running in interactive mode (default), the CLI prompts for:

### 1. Project Name

```
? Project name: my-axum-app
  > Enter a name for your project (kebab-case recommended)
  - Must be valid Cargo package name
  - Cannot start with a digit
  - Cannot be a Rust keyword
  - Example: my-awesome-app
```

**Validation**:
- Must match regex: `^[a-zA-Z][a-zA-Z0-9_-]*$`
- Cannot be empty
- Max length: 100 characters
- Cannot be a Rust keyword (see RESERVED_KEYWORDS in data-model.md)

### 2. Database Support (Optional)

```
? Enable database support?
  > PostgreSQL + SQLite
    PostgreSQL only (production)
    SQLite only (development)
    None

  Toggle with arrow keys, confirm with Enter
```

**Follow-up if database enabled**:
```
? Default database URL: postgresql://postgres:password@localhost/mydb
  > This will be added to .env.example
  - PostgreSQL: postgresql://user:pass@host/db
  - SQLite: sqlite://path/to/db.sqlite
```

### 3. Authentication (Optional)

```
? Enable JWT authentication? (y/N)
  > Adds login/logout endpoints
  - Includes user model
  - JWT token-based authentication
  - Press Enter for default (N)
```

**Follow-up if auth enabled**:
```
? Token expiration (hours): 24
  > JWT token lifetime in hours
  - Default: 24 hours
  - Recommended: 1-168 hours (1-7 days)
```

### 4. Business Error Handling (Optional)

```
? Enable business error handling? (y/N)
  > Integrates biz-error crate
  - Standardized error responses
  - Multilingual error messages (EN/CN)
  - Type-safe error codes
  - Press Enter for default (N)
```

### 5. Logging Level

```
? Logging level: info
  > Controls verbosity of generated application
  - trace: Very detailed (development only)
  - debug: Detailed diagnostic info
  - info: Normal operation (default)
  - warn: Warning messages only
  - error: Error messages only
```

## Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | SUCCESS | Project generated successfully |
| 1 | GENERAL_ERROR | Generic error (see error message) |
| 2 | INVALID_ARGUMENTS | Invalid command-line arguments |
| 3 | VALIDATION_ERROR | Project name or configuration validation failed |
| 4 | FILESYSTEM_ERROR | File system operation failed |
| 5 | PERMISSION_DENIED | Insufficient permissions |
| 6 | TOOLCHAIN_MISSING | Rust/Cargo not installed |
| 7 | DIRECTORY_EXISTS | Target directory already exists |
| 8 | TEMPLATE_ERROR | Template rendering failed |
| 10 | USER_ABORTED | User cancelled operation (Ctrl+C) |

## Error Messages

All errors follow this format:

```
error: Error message here

  > Additional context or suggestion

  For more information, try --help
```

### Examples

**Invalid project name**:
```
error: Invalid project name '123invalid'

  > Project names cannot start with a digit

  Valid names: my-app, my_app, MyApp-123
```

**Directory exists**:
```
error: Directory 'my-app' already exists

  > Choose a different project name or remove the existing directory

  Use: rm -rf my-app
```

**Missing Rust toolchain**:
```
error: Rust toolchain not found

  > create-axum-app requires Rust and Cargo to be installed

  Install from: https://rustup.rs/
```

**Permission denied**:
```
error: Permission denied: cannot create directory '/opt/my-app'

  > Try running from your home directory or with appropriate privileges

  Suggested: cd ~ && create-axum-app my-app
```

## Output Format

### Success Output

```
✨ Created a new Axum project: my-app

  Next steps:
    cd my-app
    cargo run

  Or run tests:
    cd my-app
    cargo test

  Happy hacking! 🦀
```

### Progress Indicators

During generation, show progress:

```
🔨 Generating project files...
   ✓ Created Cargo.toml
   ✓ Created src/main.rs
   ✓ Created src/lib.rs
   ✓ Created .env.example
   ✓ Created .gitignore
   ✓ Created README.md

🔧 Initializing Git repository...
   ✓ Initialized git repo
   ✓ Created initial commit

✨ Done! Project generated in 2.3s
```

### Verbose Output

When `RUST_LOG=debug` is set:

```
[DEBUG] Validating project name: my-app
[DEBUG] Template context: { project_name: "my-app", features: {...} }
[DEBUG] Rendering template: Cargo.toml.hbs
[DEBUG] Writing file: /tmp/my-app/Cargo.toml
[INFO] ✓ Created Cargo.toml
...
```

## Generated Project Structure

### Default (No Optional Features)

```
my-app/
├── .gitignore
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── lib.rs
```

### With Database Support

```
my-app/
├── .env.example
├── .gitignore
├── Cargo.toml
├── README.md
├── biz_errors.yaml        # if biz-error enabled
├── migrations/
│   └── 001_initial.sql
└── src/
    ├── main.rs
    ├── lib.rs
    ├── config.rs
    ├── handlers/
    │   ├── mod.rs
    │   └── health.rs
    ├── models/
    │   ├── mod.rs
    │   └── user.rs        # if auth enabled
    └── middleware/
        ├── mod.rs
        └── auth.rs        # if auth enabled
```

## Non-Interactive Mode

When `--non-interactive` flag is set or `CI` environment variable is detected:

- No prompts are displayed
- Missing required values cause immediate exit with code 2
- Uses sensible defaults for all optional values
- Suitable for scripting and automation

### Example (CI/CD)

```bash
# In CI/CD pipeline
export CI=true
create-axum-app test-app --database postgresql --non-interactive
```

### Example (Scripting)

```bash
#!/bin/bash
# generate-app.sh

create-axum-app "$1" \
  --database both \
  --auth \
  --biz-error \
  --log-level info \
  --non-interactive

if [ $? -eq 0 ]; then
  echo "Project generated successfully"
  cd "$1"
  cargo test
fi
```

## Validation Rules

### Project Name Validation

```rust
// Pseudo-code
fn validate_project_name(name: &str) -> Result<(), Error> {
    // 1. Not empty
    if name.is_empty() {
        return Err(Error::EmptyName);
    }

    // 2. Max length 100
    if name.len() > 100 {
        return Err(Error::NameTooLong);
    }

    // 3. Cannot start with digit
    if name.chars().next().unwrap().is_ascii_digit() {
        return Err(Error::StartsWithDigit);
    }

    // 4. Valid characters only
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(Error::InvalidCharacters);
    }

    // 5. Not a Rust keyword
    if RUST_KEYWORDS.contains(name) {
        return Err(Error::ReservedKeyword);
    }

    Ok(())
}
```

### Database URL Validation

```rust
fn validate_database_url(url: &str) -> Result<(), Error> {
    if url.starts_with("postgresql://") || url.starts_with("sqlite://") {
        Ok(())
    } else {
        Err(Error::InvalidDatabaseUrl)
    }
}
```

## Configuration File Format

### .env.example Template

```bash
# Server Configuration
HOST=127.0.0.1
PORT=8080

# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost/mydb
# Or for SQLite:
# DATABASE_URL=sqlite://mydb.sqlite

# Authentication (if enabled)
JWT_SECRET=your-secret-key-min-32-chars

# Logging
LOG_LEVEL=info
```

## Internationalization

The CLI tool supports English and Chinese (Simplified) in error messages and prompts.

### Language Detection

Language is auto-detected from environment variables:
1. `CREATE_AXUM_APP_LANG`
2. `LANG` (e.g., `en_US.UTF-8`, `zh_CN.UTF-8`)
3. Default: English

### Example

```bash
# Force Chinese
export CREATE_AXUM_APP_LANG=zh
create-axum-app my-app

# Output:
? 项目名称: my-app
  > 输入项目名称 (推荐 kebab-case)
```

## Performance Characteristics

### Expected Performance

| Operation | Target | Measured |
|-----------|--------|----------|
| CLI startup | < 100ms | TBD |
| Project generation | < 10s | TBD |
| Binary size | < 10MB | TBD |
| Memory usage | < 50MB RSS | TBD |

### Performance Profiling

To measure performance:

```bash
# Generation time
hyperfine 'create-axum-app test-app'

# Binary size
ls -lh $(which create-axum-app)

# Memory usage
/usr/bin/time -v create-axum-app test-app 2>&1 | grep "Maximum resident"
```

## Testing

### Unit Tests

Test individual functions:

```bash
cargo test --lib
```

### Integration Tests

Test full CLI invocation:

```bash
cargo test --test integration_tests
```

### Manual Testing

Test various scenarios:

```bash
# 1. Basic project
create-axum-app basic-app
cd basic-app && cargo test

# 2. With all features
create-axum-app full-app \
  --database both \
  --auth \
  --biz-error \
  --log-level debug
cd full-app && cargo test

# 3. Invalid project name
create-axum-app 123invalid
# Expected: Exit code 3

# 4. Directory exists
mkdir existing-app
create-axum-app existing-app
# Expected: Exit code 7

# 5. Non-interactive mode
create-axum-app test-app --non-interactive
# Expected: Success or exit code 2 if missing required values
```

## Versioning

The CLI follows Semantic Versioning (SemVer):

- **Major (X.0.0)**: Breaking changes to CLI interface
- **Minor (0.X.0)**: New features, backward compatible
- **Patch (0.0.X)**: Bug fixes, backward compatible

### Example

- `1.0.0`: Initial MVP release
- `1.1.0`: Add `--docker` flag
- `1.1.1`: Fix template rendering bug
- `2.0.0`: Remove `--database` flag, replace with `--db`

## Backward Compatibility

### Guaranteed Stable

The following are guaranteed to remain stable until v2.0.0:

- Command structure: `create-axum-app [PROJECT_NAME] [OPTIONS]`
- Core flags: `--help`, `--version`, `--database`, `--auth`, `--biz-error`
- Exit codes: 0 (success), 1 (error)
- Generated project structure: `src/`, `Cargo.toml`, `README.md`

### May Change

The following may change in minor releases:

- Prompt wording (improved UX)
- Error message wording
- Generated code comments
- Template internal structure

### Will Not Change

The following will NOT change without major version bump:

- Flag names (new flags may be added)
- Environment variable names
- Exit code meanings
- Project name validation rules

## Future Extensions (Phase 2+)

The following are planned for future releases but NOT part of Phase 1 MVP:

- `--template` flag: Use custom template repository
- `--workspace` flag: Generate workspace projects
- `--update` flag: Update existing projects
- `init` subcommand: Initialize in existing directory
- `config` subcommand: Manage global configuration
- Interactive mode improvements: Multi-select, advanced prompts

## Examples

### Minimal Project

```bash
create-axum-app minimal-app
```

Generates a basic Axum web server with:
- HTTP server on port 8080
- Health check endpoint
- Structured logging
- No database
- No authentication
- No business error handling

### Full-Stack Project

```bash
create-axum-app full-app \
  --database both \
  --auth \
  --biz-error \
  --log-level debug \
  --author "Jane Doe"
```

Generates a production-ready project with:
- PostgreSQL + SQLite database support
- JWT authentication
- Business error handling (i18n)
- Debug-level logging
- Complete README (EN/CN)
- Example endpoints and tests

### CI/CD Automation

```bash
# In GitHub Actions
- name: Generate test project
  run: |
    create-axum-app test-app \
      --database postgresql \
      --non-interactive
    cd test-app
    cargo test
```

## References

- **Clap Documentation**: https://docs.rs/clap/
- **Inquire Documentation**: https://docs.rs/inquire/
- **Rust CLI Book**: https://rust-cli.github.io/book/
- **SemVer**: https://semver.org/

---

**Document Version**: 1.0.0
**Last Updated**: 2025-02-05
**Maintainer**: create-axum-app team
