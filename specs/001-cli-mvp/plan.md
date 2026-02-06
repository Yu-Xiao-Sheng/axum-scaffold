# Implementation Plan: axum-app-create CLI Tool - Phase 1 MVP

**Branch**: `001-cli-mvp` | **Date**: 2025-02-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-cli-mvp/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a Rust-based CLI scaffolding tool (similar to create-react-app) that generates production-ready Axum web framework projects with zero configuration. The tool provides interactive prompts for project customization, supports hybrid command-line/interactive modes, and generates complete, compilable Rust projects with sensible defaults. Phase 1 MVP focuses on single-project mode with embedded templates, targeting individual developers and beginners with features including database support (PostgreSQL/SQLite), JWT authentication, structured logging, and optional biz-error integration for standardized error responses. The tool emphasizes developer experience with <10s generation time, automatic Git initialization, and bilingual documentation (English/Chinese).

## Technical Context

**Language/Version**: Rust 2024 Edition (latest stable)
**Primary Dependencies**: clap (CLI parsing), inquire (interactive prompts), handlebars (template rendering), axum (web framework), tokio (async runtime), tracing (structured logging), sqlx (database), serde (serialization), biz-error (error handling), jsonwebtoken (JWT), thiserror (error types), anyhow (error context)
**Storage**: Embedded templates (static assets in binary via include_str!), SQLite/PostgreSQL databases (generated projects only), .env files for configuration
**Testing**: cargo test with integration and unit test examples in generated projects
**Target Platform**: Cross-platform CLI (Linux, macOS, Windows) - distributed via crates.io
**Project Type**: Single binary CLI tool that generates standalone Rust projects
**Performance Goals**: < 10s project generation time, < 100ms CLI startup time, < 10MB binary size, < 50MB RSS memory usage during generation
**Constraints**: Hardware baseline (4-8 CPU cores, 8-16GB RAM, NVMe SSD), embedded templates (no network dependency), Rust toolchain validation required, tool must work offline after installation
**Scale/Scope**: Individual developers and beginners, hundreds to thousands of daily global uses, single-package Cargo project generation (workspace mode excluded from Phase 1)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle 1: Developer Experience First
✅ **PASS** - Spec requires <10s generation (SC-006), sensible defaults (FR-012), interactive prompts (FR-011), clear error messages (FR-016), and bilingual documentation (FR-009, Q10)

### Principle 2: Dual Mode Architecture
✅ **PASS** - Phase 1 focuses on single mode only (FR-004), workspace mode explicitly out of scope. Hybrid CLI interaction supported (FR-001, Q5)

### Principle 3: Template Quality Gates
✅ **PASS** - Generated projects must be compilable (SC-003), use latest Rust edition (FR-008), include structured logging (FR-006), proper error types (FR-007), and integration tests (FR-019)

### Principle 4: Extensibility
✅ **PASS** - Optional features (database, auth, logging, biz-error) are modular (FR-020, FR-021-FR-030). Custom template repositories and plugin system deferred to Phase 2

### Principle 5: Rust Idiomaticity
✅ **PASS** - Requires async/await (implied by Axum), Result types (FR-007), thiserror for domain errors (implied), proper Cargo structure (FR-004), latest edition (FR-008)

### Principle 6: Backward Compatibility
✅ **PASS** - Tool generates new projects, doesn't modify existing ones. No backward compatibility concerns for Phase 1

### Overall Assessment
**ALL GATES PASSED** - Specification complies with all constitution principles. No violations requiring justification in Complexity Tracking section.

## Project Structure

### Documentation (this feature)

```text
specs/001-cli-mvp/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# CLI Tool Structure
axum-app-create/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli/
│   │   ├── mod.rs           # CLI module
│   │   ├── prompts.rs       # Interactive prompts using inquire
│   │   └── args.rs          # Command-line argument parsing with clap
│   ├── template/
│   │   ├── mod.rs           # Template engine module
│   │   ├── engine.rs        # Template rendering with handlebars
│   │   ├── context.rs       # Template context (variables for substitution)
│   │   └── templates/       # Embedded project templates
│   │       ├── single_mode/ # Single-package project templates
│   │       │   ├── Cargo.toml.hbs
│   │       │   ├── src/
│   │       │   │   ├── main.rs.hbs
│   │       │   │   ├── lib.rs.hbs
│   │       │   │   ├── handlers/
│   │       │   │   ├── models/
│   │       │   │   └── config.rs.hbs
│   │       │   ├── tests/
│   │       │   ├── README.md.hbs
│   │       │   ├── .env.example
│   │       │   ├── .gitignore
│   │       │   └── biz_errors.yaml.hbs  # Error definitions (if biz-error enabled)
│   │       ├── database/     # Database feature templates
│   │       ├── auth/         # Authentication feature templates
│   │       └── logging/      # Logging feature templates
│   ├── generator/
│   │   ├── mod.rs           # Generator module
│   │   ├── project.rs       # Project generation logic
│   │   ├── validator.rs     # Project name validation
│   │   └── git.rs           # Git initialization (git init, .gitignore, commit)
│   └── utils/
│       ├── mod.rs           # Utility functions
│       └── rust_toolchain.rs # Rust/Cargo installation check
├── tests/
│   ├── integration/         # Integration tests
│   │   ├── cli_tests.rs     # CLI argument parsing tests
│   │   ├── generation_tests.rs # Project generation tests
│   │   └── template_tests.rs    # Template rendering tests
│   └── fixtures/            # Test fixtures
├── Cargo.toml               # CLI tool dependencies
├── README.md                # CLI tool documentation
└── build.rs                 # Build script (if needed for templates)
```

**Structure Decision**: Single binary CLI tool with embedded templates (Option 1). This structure is chosen because:
1. The CLI tool itself is a single Rust package distributed via crates.io
2. All project templates are embedded in the binary using `include_str!` macro for offline operation
3. No frontend/backend separation needed - the CLI is a standalone command-line tool
4. Generated projects will be single-package Cargo workspaces (as per Phase 1 scope)
5. Tests organized by type (integration vs unit) following Rust conventions

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | All constitution gates passed | No violations to justify |

