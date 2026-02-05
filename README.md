# create-axum-app

> A command-line tool to quickly create Axum web applications

[![Crates.io](https://img.shields.io/crates/v/create-axum-app)](https://crates.io/crates/create-axum-app)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

`create-axum-app` is a CLI tool that helps you quickly scaffold Axum web applications in Rust, similar to how `create-next-app` works for Next.js projects.

## 🚀 Features

### Single Project Mode
- Quick-start single-module Axum application
- Perfect for learning and small projects
- Minimal setup, maximum productivity

### Workspace Mode (Enterprise-Grade)
- Multi-module workspace architecture
- Ideal for large-scale applications
- Organized structure for team collaboration

**Workspace Architecture:**
```
create-axum-app/
├── common/          # Global shared modules
│   ├── Infrastructure
│   ├── Utilities
│   └── Components
├── module/          # Business modules
│   ├── user-module/
│   ├── order-module/
│   └── ... (multiple business modules)
└── provider/        # Axum service providers
    ├── api-provider/
    ├── admin-provider/
    └── ... (multiple service modules)
```

## 📦 Installation

```bash
cargo install create-axum-app
```

## 🦀 Quick Start

### Create a Single Project

```bash
create-axum-app my-app --mode single
```

### Create a Workspace Project

```bash
create-axum-app my-workspace --mode workspace
```

### Interactive Mode

```bash
create-axum-app my-app
# Follow the prompts to configure your project
```

## 🛠️ Roadmap

- [ ] CLI command implementation
- [ ] Single project template
- [ ] Workspace project template
- [ ] Interactive configuration wizard
- [ ] Custom template support
- [ ] Plugin system

## 📚 Documentation

Coming soon! For now, please refer to the [issues](https://github.com/Yu-Xiao-Sheng/create-axum-app/issues) for development progress.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio
- [Tower](https://github.com/tower-rs/tower) - Library of modular and reusable components
- Inspired by [create-next-app](https://github.com/vercel/next.js/tree/canary/packages/create-next-app)

---

**Note**: This project is currently under active development. The APIs and features may change before the 1.0 release.
