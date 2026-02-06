# Feature Specification: axum-app-create CLI Tool - Phase 1 MVP

**Feature Branch**: `001-cli-mvp`
**Created**: 2025-02-05
**Status**: Draft
**Input**: User description: "axum-app-create CLI tool - Phase 1 MVP

Project: A Rust scaffolding tool for Axum web framework (similar to create-react-app)

Phase 1 Focus:
- Single Project Mode only (beginner-friendly)
- Basic CLI with interactive prompts
- Minimal viable project template
- Project initialization and generation

Context: The tool aims to lower the barrier to Rust web adoption by providing a zero-configuration scaffolding experience. First phase targets individual developers and beginners with a simple single-binary project mode.

Core requirements from constitution:
- Developer Experience First: < 10s generation, sensible defaults
- Single Mode: Single Cargo package, minimal boilerplate
- Template Quality: Production-ready with tracing, error handling
- Rust Idiomaticity: Latest edition, async/await, Result types"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Quick Start with Zero Configuration (Priority: P1)

A developer new to Rust web development wants to start building an Axum web application immediately without configuring project structure, dependencies, or build settings manually.

**Why this priority**: This is the core value proposition - if users cannot quickly generate a working project, the tool fails its primary purpose. This scenario delivers immediate value and validates the entire concept.

**Independent Test**: Can be fully tested by running the CLI tool with default prompts and verifying the generated project compiles and runs a basic web server that responds to HTTP requests.

**Acceptance Scenarios**:

1. **Given** a developer with Rust toolchain installed, **When** they run the CLI command, **Then** the tool prompts for project name with a sensible default
2. **Given** the developer accepts the default project name, **When** generation completes, **Then** a new project directory is created with all necessary files
3. **Given** the generated project, **When** the developer runs the project, **Then** a web server starts on localhost and responds to HTTP requests
4. **Given** the developer provides a custom project name, **When** generation completes, **Then** the project uses the custom name in all configuration and source files
5. **Given** generation completes, **When** the developer checks the output, **Then** the generation time is under 10 seconds

---

### User Story 2 - Interactive Customization Options (Priority: P2)

A developer wants to customize their project with common options like adding database support, API framework features, logging configuration, or business error handling during project creation.

**Why this priority**: While zero-config is important, beginners quickly outgrow basic templates. This scenario extends tool value without overwhelming new users with too many options upfront.

**Independent Test**: Can be tested by invoking the CLI with different option combinations and verifying generated projects include the requested features (e.g., database connection pooling code when database option is selected).

**Acceptance Scenarios**:

1. **Given** the CLI prompts for optional features, **When** the developer selects "database support", **Then** the generated project includes database connection setup and example queries
2. **Given** the developer selects "authentication", **When** generation completes, **Then** the project includes JWT-based authentication boilerplate and login endpoint
3. **Given** the developer selects "business error handling", **When** generation completes, **Then** the project includes the biz-error library with pre-configured error definitions and example usage
4. **Given** the developer declines all optional features, **When** generation completes, **Then** the project remains minimal with only core HTTP server functionality and basic error handling
5. **Given** the CLI presents options, **When** the developer presses Enter without selection, **Then** the tool accepts the default (typically "no" for optional features)
6. **Given** multiple optional features selected, **When** generation completes, **Then** all selected features are properly integrated without conflicts

---

### User Story 3 - Clear Error Messages and Guidance (Priority: P3)

A developer encounters an issue during project generation (e.g., invalid project name, missing dependencies, file system permissions) and needs helpful guidance to resolve it.

**Why this priority**: Error handling is critical for user experience but doesn't block MVP. Beginners are more likely to succeed with good error messages, reducing support burden and frustration.

**Independent Test**: Can be tested by triggering various error conditions (invalid names, existing directories, permission issues) and verifying the tool provides actionable error messages with suggested fixes.

**Acceptance Scenarios**:

1. **Given** the developer provides an invalid project name (e.g., contains special characters), **When** validation fails, **Then** the tool explains the naming rules and prompts for a valid name
2. **Given** the target directory already exists, **When** generation is attempted, **Then** the tool warns the user and asks whether to overwrite or cancel
3. **Given** required Rust tools are missing, **When** the tool checks prerequisites, **Then** it provides installation instructions for the missing tools
4. **Given** file system permissions prevent creation, **When** the error occurs, **Then** the tool explains the permission issue and suggests running with appropriate privileges
5. **Given** generation fails for any reason, **When** the tool exits, **Then** it provides a clear error message with a suggested next step or troubleshooting link

---

### User Story 4 - Project Verification and Quick Start Guide (Priority: P3)

After project generation, a developer wants to verify everything works correctly and understand how to start developing their application.

**Why this priority**: Post-generation guidance improves success rates but isn't critical for MVP. This scenario enhances onboarding and reduces time-to-first-working-code.

**Independent Test**: Can be tested by generating a project and verifying the tool outputs clear next steps, and that following those steps results in a running application.

**Acceptance Scenarios**:

1. **Given** project generation completes successfully, **When** the tool finishes, **Then** it displays a summary with commands to run and test the project
2. **Given** the developer follows the printed commands, **When** they execute them in order, **Then** the application starts without errors
3. **Given** the generated project, **When** the developer opens the README, **Then** it contains quick start instructions, project structure overview, and next steps
4. **Given** the project includes example code, **When** the developer runs the examples, **Then** they demonstrate basic functionality (e.g., "Hello World" endpoint)
5. **Given** the developer wants to customize the project, **When** they read the generated comments, **Then** key files explain their purpose without requiring external documentation

---

---

### User Story 5 - Standardized Error Response Format (Priority: P2)

A developer building a web API needs to return consistent, structured error responses to clients. The generated project should include a production-ready error handling system that provides standardized error responses, type-safe error codes, and internationalization support.

**Why this priority**: While basic error handling is sufficient for learning projects, production applications require structured error responses for client integration. Including biz-error integration as an option makes generated templates production-ready and teaches best practices.

**Independent Test**: Can be tested by generating a project with business error handling enabled and triggering different error conditions to verify all errors return a consistent JSON structure with code, message, and optional data fields.

**Acceptance Scenarios**:

1. **Given** the developer selects "business error handling" option, **When** the project is generated, **Then** it includes the biz-error library as a dependency with Axum integration
2. **Given** business error handling is enabled, **When** a handler returns an error, **Then** the response follows the standard JSON format with code (number), msg (string), and optional data (object) fields
3. **Given** a validation error occurs, **When** the error is returned, **Then** the HTTP status code matches the error severity (400 for invalid input, 404 for not found, 500 for server errors)
4. **Given** business error handling is enabled, **When** the project is generated, **Then** it includes a pre-configured error definition file (biz_errors.yaml) with common error codes in multiple languages
5. **Given** the developer defines errors in the YAML configuration, **When** the project is compiled, **Then** error code enums are automatically generated and visible in IDE autocomplete
6. **Given** an error occurs with additional context, **When** the error is returned, **Then** the data field contains validation details (e.g., which field failed and why)

---

### Edge Cases

- What happens when the project name is a reserved Cargo keyword (e.g., "std", "fn")?
- What happens when the user's home directory path contains non-ASCII characters?
- What happens if the user interrupts generation with Ctrl+C?
- How does the tool behave when run in a directory with insufficient disk space?
- What happens when the user provides an extremely long project name (>100 characters)?
- How does the tool handle concurrent invocations attempting to create projects in the same directory?
- What happens when the system has restrictive file permissions (e.g., corporate-managed developer machines)?
- What happens when the error configuration YAML file is malformed or missing required fields (if business error handling is enabled)?
- What happens when two error codes are defined with the same numeric code in biz_errors.yaml?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The tool MUST support hybrid interaction mode:
  - Accept all options (project name, optional features) via command-line flags for non-interactive/scripted usage
  - Prompt interactively for any missing required values (default behavior when run without flags)
  - Support command-line flags for: project name, database option (none/postgresql/sqlite), authentication option (yes/no), business error handling (yes/no), and logging level configuration
- **FR-002**: The tool MUST validate project names according to Cargo naming conventions (alphanumeric, underscores, hyphens only; cannot start with a digit; cannot be a Rust keyword)
- **FR-003**: The tool MUST generate a complete, compilable Rust project structure using Cargo with all templates embedded in the CLI binary
- **FR-004**: The tool MUST create a single-package Cargo workspace with all source code in `src/` directory
- **FR-005**: The tool MUST generate an Axum web server that binds to 127.0.0.1:8080 by default (localhost port 8080), with the port configurable via the PORT environment variable
- **FR-006**: The tool MUST include structured logging with tracing instrumentation in the generated code
- **FR-007**: The tool MUST use proper error types (Result types, thiserror) in generated code
- **FR-008**: The tool MUST configure the project to use the latest stable Rust edition
- **FR-009**: The tool MUST include a README with quick start instructions and project structure overview in both English and Chinese (bilingual documentation)
- **FR-010**: The tool MUST complete project generation in under 10 seconds on modern hardware
- **FR-011**: The tool MUST provide interactive prompts for optional features (database, authentication, etc.)
- **FR-012**: The tool MUST use sensible defaults so accepting all prompts generates a working minimal project
- **FR-013**: The tool MUST display a success message with commands to run and test the generated project
- **FR-014**: The tool MUST validate that required tools (Rust, Cargo) are installed before attempting generation
- **FR-015**: The tool MUST check if the target directory already exists and prompt for overwrite confirmation
- **FR-016**: The tool MUST provide clear error messages with actionable guidance when generation fails
- **FR-017**: The tool MUST include example HTTP endpoints demonstrating basic Axum functionality
- **FR-018**: The tool MUST generate proper Cargo.toml with all necessary dependencies for Axum
- **FR-019**: The tool MUST include integration test examples in the generated project
- **FR-020**: The tool MUST support common optional features with full production-ready implementations:
  - **Database support**: Pre-configured support for PostgreSQL (production) and SQLite (local development) with connection pooling (using deadpool or sqlx-built-in pooling), example query patterns, migration support setup using a migration tool (e.g., sqlx-cli or sea-orm migrations), and environment-based configuration switching via .env file
  - **Authentication**: JWT-based authentication with token generation, validation middleware, login/logout endpoints, and user model examples with secret keys managed via environment variables
  - **Logging level configuration**: Configurable logging levels (trace, debug, info, warn, error) via environment variables (e.g., LOG_LEVEL), with .env file template for local development
- **FR-020A**: The generated project MUST include a .env.example file template documenting all required and optional environment variables, including:
  - Server configuration: PORT (default 8080), HOST (default 127.0.0.1)
  - Database: DATABASE_URL (PostgreSQL or SQLite connection string)
  - Authentication: JWT_SECRET (for JWT-based auth)
  - Logging: LOG_LEVEL (default info)
  - Application-specific settings as needed
- **FR-020B**: The tool MUST automatically initialize a Git repository in the generated project:
  - Run `git init` to create repository
  - Create a comprehensive .gitignore file excluding target/, .env, Cargo.lock, IDE files, and OS-specific files
  - Create an initial commit with the message "Initial project generated by axum-app-create"
- **FR-021**: When business error handling is enabled, the tool MUST include the biz-error library as a dependency with Axum integration features
- **FR-022**: When business error handling is enabled, the tool MUST generate a pre-configured error definition file (biz_errors.yaml) with common error codes (validation, not found, database errors) in at least two languages (English and Chinese)
- **FR-023**: When business error handling is enabled, the tool MUST use a procedural macro to automatically generate error code enums from the YAML configuration
- **FR-024**: When business error handling is enabled, all error responses MUST follow a standard JSON format with code (number), msg (string), and optional data (object) fields
- **FR-025**: When business error handling is enabled, error responses MUST automatically map to the correct HTTP status code based on the error definition
- **FR-026**: When business error handling is enabled, the error system MUST support custom error messages that override default messages
- **FR-027**: When business error handling is enabled, the error system MUST support attaching arbitrary JSON data to errors for additional context
- **FR-028**: When business error handling is enabled, generated example handlers MUST demonstrate returning business errors with rich context
- **FR-029**: When business error handling is enabled, the project README MUST document how to add new error codes and customize error handling
- **FR-030**: When business error handling is enabled, the error configuration file MUST validate that error codes are unique (no duplicate numeric codes)

### Key Entities

- **Project Template**: A collection of file templates and configuration that define the structure of a generated Axum web application project
  - Attributes: template type (single mode), included dependencies, optional features
- **User Answers**: The set of responses provided by the developer during interactive prompts
  - Attributes: project name, selected optional features, customization choices
- **Generated Project**: The complete, compilable Rust project created by the tool
  - Attributes: project name, directory structure, dependencies, included features
- **Error Definition Configuration** (when business error handling enabled): A YAML file that defines all business error codes, their numeric values, HTTP status mappings, and multilingual messages
  - Attributes: error name, numeric code, HTTP status, message translations
- **Business Error Instance** (when business error handling enabled): A specific error occurrence that includes the error code, custom message, and optional context data
  - Attributes: error code, message, data payload, HTTP status

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can generate a working Axum project from command invocation to "cargo run" success in under 30 seconds total
- **SC-002**: 90% of first-time users successfully generate and run their first project without requiring external documentation
- **SC-003**: Generated projects compile without errors on the first attempt
- **SC-004**: Generated projects pass all included tests when running "cargo test"
- **SC-005**: Tool startup time (time to first prompt) is under 100 milliseconds when running on typical developer hardware (4-8 CPU cores, 8-16GB RAM, NVMe SSD)
- **SC-006**: Project generation time (after user confirms all prompts) is under 10 seconds when running on typical developer hardware (4-8 CPU cores, 8-16GB RAM, NVMe SSD)
- **SC-007**: Error messages are actionable: 80% of users can resolve common errors (invalid project name, directory exists, missing tools) without external help
- **SC-008**: Memory usage during operation stays under 50MB RSS
- **SC-009**: Users report satisfaction with the generated code quality (measured via post-MVP survey or GitHub feedback)
- **SC-010**: When business error handling is enabled, 100% of errors returned by the generated application follow the standard JSON response format
- **SC-011**: When business error handling is enabled, error responses include the correct HTTP status code for 100% of defined error types
- **SC-012**: When business error handling is enabled, developers can define a new error code and use it in their code in under 2 minutes

### Assumptions

- Users have Rust toolchain (rustc, cargo) installed before running the tool
- The tool is distributed via crates.io and installed using `cargo install axum-app-create`
- Users have basic familiarity with command-line interfaces
- Users have write permissions in their working directory
- The tool runs on Linux, macOS, and Windows (major platforms)
- Users have internet connectivity for the initial tool installation from crates.io (but not for project generation)
- Generated projects target development environments initially (localhost binding, development logging)
- Template dependencies are pinned to specific versions to ensure reproducibility
- When business error handling is enabled, developers are familiar with YAML configuration file format
- When business error handling is enabled, the default error configuration includes at least English and Chinese (zh-CN) translations
- When business error handling is enabled, error messages are primarily targeted at API consumers (developers or frontend applications)

## Clarifications

### Session 2025-02-05

- Q: What level of support should optional features (database, authentication, logging) provide in Phase 1? → A: Full integration - Pre-configured, production-ready implementations with actual database pooling, authentication server, and configurable logging levels
- Q: What hardware specifications should be used as the baseline for performance benchmarks? → A: Typical developer hardware - 4-8 CPU cores, 8-16GB RAM, NVMe SSD (common dev machines from 2023-2024)
- Q: How should project templates be distributed in Phase 1? → A: Embedded templates - All project templates are embedded in the CLI binary during build (simplest, no network dependency, works offline)
- Q: Which database technologies should be supported in Phase 1? → A: PostgreSQL + SQLite - PostgreSQL for production deployments, SQLite for local development (balances production readiness with zero-config local experience)
- Q: Should the CLI support non-interactive mode for scripting/automation? → A: Hybrid mode - Accept command-line flags for all options, default to interactive prompts for missing values (supports both beginners and automation)
- Q: How should the CLI tool be distributed and installed? → A: crates.io publication - Install via `cargo install axum-app-create` (standard Rust pattern, no external infrastructure)
- Q: How should generated projects manage configuration? → A: Environment variables with .env file support (12-factor app compliant, container-native, simple)
- Q: Should the generated project automatically initialize Git repository? → A: Initialize with .gitignore and initial commit - Run `git init`, create Rust/Cargo .gitignore, and make initial commit (instant version control, prevents committing generated files)
- Q: What should be the default server port for generated projects? → A: Fixed port 8080 (simple and common, documented in README)
- Q: What language should the generated README documentation use? → A: Bilingual (English + Chinese) - Both languages in same README, aligns with error message i18n support

---

### Out of Scope for Phase 1

- Workspace Mode (multi-package projects) is explicitly excluded from Phase 1
- Custom template repositories or plugin system are Phase 2 features
- Project update/migration functionality (e.g., updating dependencies in existing projects) is not included
- GUI or web-based interfaces are out of scope (CLI only)
- Production deployment configuration (Docker, cloud deployment) is deferred to later phases
- Advanced project customization (custom middleware, specific middleware combinations) is not included
- Template marketplace or community templates are Phase 2 features
- Advanced business error handling features are out of scope:
  - Dynamic error code loading from external sources (database, remote API) is not included
  - Error code management UI or dashboard for visual error definition is not included
  - Automatic error code migration tools from other error handling libraries are not included
  - Advanced error response formats (XML, Protocol Buffers) are not supported (JSON only)
  - Error analytics or tracking (monitoring which errors occur most frequently) is not included
