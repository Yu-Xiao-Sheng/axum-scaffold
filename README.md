# create-axum-app

> A command-line tool to quickly create Axum web applications with zero configuration

[![CI](https://github.com/Yu-Xiao-Sheng/create-axum-app/workflows/CI/badge.svg)](https://github.com/Yu-Xiao-Sheng/create-axum-app/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

`create-axum-app` is a CLI tool that helps you quickly scaffold Axum web applications in Rust, similar to `create-react-app` or `cargo-generate`. It generates production-ready projects with sensible defaults, structured logging, error handling, and optional features like database support and authentication.

## 🚀 Features

### Phase 1 MVP (Current)
- ✅ **Single Project Mode**: Generate single-package Axum applications
- ✅ **Interactive Prompts**: Friendly CLI with interactive configuration
- ✅ **Sensible Defaults**: Works out of the box with zero configuration
- ✅ **Production-Ready Templates**: Includes tracing, error handling, proper project structure
- ✅ **Bilingual Documentation**: English and Chinese (中文)
- ✅ **Git Initialization**: Automatic git repo, .gitignore, and initial commit

### Planned Features (Phase 2+)
- 🚧 Database Support (PostgreSQL, SQLite)
- 🚧 JWT Authentication
- 🚧 Business Error Handling (biz-error integration)
- 🚧 Workspace Mode (multi-package projects)
- 🚧 Custom Templates
- 🚧 Plugin System

## 📦 Installation

### From crates.io (Coming Soon)
```bash
cargo install create-axum-app
```

### From Source
```bash
git clone https://github.com/Yu-Xiao-Sheng/create-axum-app.git
cd create-axum-app
cargo install --path .
```

## 🦀 Quick Start

### Interactive Mode (Recommended)
```bash
create-axum-app my-app
# Follow the prompts to configure your project
cd my-app
cargo run
```

### Command-Line Mode
```bash
create-axum-app my-app \
  --database both \
  --auth \
  --biz-error \
  --log-level info

cd my-app
cargo run
```

### Non-Interactive Mode (CI/CD)
```bash
create-axum-app my-app --non-interactive
```

## 📚 Command-Line Options

| Flag | Short | Description |
|------|-------|-------------|
| `--project-name <NAME>` | `-p` | Set project name |
| `--database <DB>` | `-d` | Database support: `none`, `postgresql`, `sqlite`, `both` |
| `--auth` | `-a` | Enable JWT authentication |
| `--biz-error` | `-b` | Enable business error handling |
| `--log-level <LEVEL>` | `-l` | Logging level: `trace`, `debug`, `info`, `warn`, `error` |
| `--author <NAME>` | | Author name for generated project |
| `--non-interactive` | | Disable prompts (fail if required values missing) |
| `--help` | `-h` | Show help message |
| `--version` | `-V` | Show version |

## 🛠️ Development

### Prerequisites
- Rust 1.75+ (use `rustup` to install)
- Git (for project initialization)

### Build from Source
```bash
# Clone repository
git clone https://github.com/Yu-Xiao-Sheng/create-axum-app.git
cd create-axum-app

# Build
cargo build --release

# Run
cargo run -- --help
```

### Run Tests
```bash
cargo test
```

### Check Code Style
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings
```

## 📖 Roadmap

- [x] CLI argument parsing with clap
- [x] Basic project structure generation
- [ ] Interactive prompts with inquire
- [ ] Template rendering with Handlebars
- [ ] Project name validation
- [ ] Embedded project templates
- [ ] Git initialization
- [ ] Optional feature templates (database, auth, logging)
- [ ] Integration tests
- [ ] Documentation and examples

See [issues](https://github.com/Yu-Xiao-Sheng/create-axum-app/issues) for detailed progress.

## 🤝 Contributing

Contributions are welcome! Please feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup
```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

## 📄 License

This project is licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio
- [clap](https://github.com/clap-rs/clap) - Command-line argument parser
- [inquire](https://github.com/mikaelmello/inquire) - Interactive prompts
- [handlebars-rust](https://github.com/sunng87/handlebars-rust) - Template engine
- Inspired by [create-react-app](https://github.com/facebook/create-react-app) and [cargo-generate](https://github.com/cargo-generate/cargo-generate)

---

**Note**: This project is currently in active development (Phase 1 MVP). APIs and features may evolve before 1.0 release.
