<!--
Sync Impact Report:
- Version: Initial (1.0.0)
- New constitution created
- Templates verified: ✅ plan-template.md, ✅ spec-template.md, ✅ tasks-template.md
- No placeholders deferred
- Follow-up TODOs: None
-->

# create-axum-app Constitution

## Core Principles

### I. Developer Experience First

The tool MUST prioritize developer productivity and ease of use over implementation simplicity. Every feature decision starts with "How does this feel to use?" before "How do we build it?".

- **Interactive defaults**: Provide sensible defaults that work for 80% of use cases
- **Zero configuration required**: Tool works immediately after installation
- **Fast feedback**: Project generation completes in under 10 seconds
- **Clear errors**: Error messages guide users toward resolution, not confusion
- **Progressive disclosure**: Simple options visible by default, advanced options available when requested

**Rationale**: The primary barrier to Rust web adoption is complexity, not performance. A CLI tool that feels magical encourages adoption; one that feels bureaucratic drives users away.

### II. Dual Mode Architecture (NON-NEGOTIABLE)

The tool MUST support two distinct project modes with clear, non-overlapping use cases:

#### Single Project Mode
- **Target Audience**: Beginners, prototypes, microservices, learning projects
- **Characteristics**:
  - Single Cargo package
  - Minimal boilerplate and dependencies
  - All code in one `src/` directory
  - Binary output: single executable
  - Startup time: < 2 seconds from `cargo run`

#### Workspace Mode (Enterprise-Grade)
- **Target Audience**: Production applications, team projects, complex domains
- **Architecture Requirements**:
  ```
  workspace/
  ├── common/          # Shared infrastructure
  │   ├── Core utilities (tracing, config, error handling)
  │   ├── Database abstractions
  │   └── Reusable components
  ├── module/          # Business domain modules
  │   ├── Each module is independently testable
  │   ├── Clear domain boundaries
  │   └── Minimal cross-module dependencies
  └── provider/        # Service entry points
      ├── api-provider/   (HTTP API)
      ├── admin-provider/ (Admin interface)
      └── grpc-provider/  (Internal services)
  ```
- **Enforcement**: Workspace mode MUST require minimum module structure validation
- **Extensibility**: New modules can be added without modifying core tooling

**Rationale**: One size does not fit all. Beginners need simplicity; enterprises need structure. Forcing enterprise patterns on beginners creates cognitive overload. Forcing simplicity on enterprises creates spaghetti code.

### III. Template Quality Gates

All project templates MUST meet baseline production-readiness standards before inclusion:

- **Security**: No hardcoded secrets, secure defaults (e.g., bind to 127.0.0.1 in development)
- **Observability**: Structured logging with tracing instrumentation from day one
- **Error Handling**: Proper error types (not `unwrap()`), user-friendly error messages
- **Testing**: Integration test examples provided, test utilities included
- **Documentation**: README with quick start, inline comments for non-obvious code
- **Database**: Connection pooling configured, migration support included

**Validation Process**: Every template undergoes manual review and automated checklist verification before merge. Templates failing quality gates block release.

**Rationale**: The tool generates production code, not toy projects. Poor templates become technical debt at scale. Quality gates prevent "garbage in, garbage out."

### IV. Extensibility and Plugin System

The tool architecture MUST support user customization without forking:

- **Custom templates**: Users can provide local or remote template repositories
- **Template hooks**: `pre-generate`, `post-generate`, `pre-build`, `post-build` hooks for custom logic
- **Plugin API**: Well-defined interface for third-party extensions (e.g., `create-axum-app-auth`, `create-axum-app-graphql`)
- **Template marketplace**: Community-contributed templates with discovery and rating

**Constraints**: Core tool behavior cannot be overridden by plugins (security guardrails). Plugins run in subprocess isolation.

**Rationale**: A CLI tool is only as valuable as its ecosystem. Locking users into official templates limits adoption. Enabling community extensions creates network effects.

### V. Rust Idiomaticity

All generated code MUST follow Rust best practices and idioms:

- **Async/await**: Tokio runtime properly configured
- **Error handling**: `Result` types, `thiserror` or `anyhow` for domain errors
- **Dependencies**: Minimal external deps, prefer stdlib or tower ecosystem
- **Testing**: Unit tests alongside source code, integration tests in `tests/`
- **Documentation**: Rustdoc comments on public APIs, crate-level documentation
- **Edition**: Always use latest stable Rust edition (currently 2024)

**Anti-patterns Banned**:
- `unwrap()` or `expect()` in user-facing code
- Blocking async code (`tokio::runtime::block_on` in async contexts)
- Global mutable state (use `once_cell` or dependency injection)
- Raw string concatenation for SQL/commands (use parameterized queries or macros)

**Rationale**: The tool teaches Rust by example. Generated code is often copied and modified. Anti-patterns in templates propagate to user projects.

### VI. Backward Compatibility (Stability)

Once released, the tool MUST maintain backward compatibility or provide clear migration paths:

- **CLI arguments**: Flag deprecation requires one version warning before removal
- **Template schemas**: Breaking changes require major version bump
- **Generated projects**: Projects created with older tool versions continue to build and run
- **Configuration file format**: New fields optional, old fields supported for ≥ 2 minor versions

**Exception**: Pre-1.0 releases may break compatibility with migration guides provided.

**Rationale**: Developers upgrade tools infrequently. Breaking changes waste time and erode trust. Stability enables adoption in teams and enterprises.

### VII. Communication Language Standard (用户沟通语言标准)

所有与用户的需求分析、报告、计划和文档输出必须使用中文。

**Language Requirements**:
- **Requirements Analysis**: All feature specifications, user stories, and requirements MUST be documented in Chinese
- **Progress Reports**: All status updates, progress reports, and completion summaries MUST be in Chinese
- **Documentation**: User-facing documentation, README files, and guides MUST be in Chinese (with English translations optional)
- **Error Messages**: Error messages shown to users SHOULD be in Chinese or bilingual
- **Code Comments**: Generated code comments MAY use Chinese for clarity when appropriate

**Exceptions**:
- Technical terms without established Chinese translations MAY use English (e.g., "crate", "macro", "trait")
- API references and official documentation links MAY remain in original language
- Internal technical documentation between developers MAY use English

**Rationale**: 本项目主要服务中文用户社区。使用中文进行需求分析和报告可以确保准确传达意图，减少理解偏差，提高协作效率。英语保留用于技术术语和国际化的场景。

## Testing Requirements

### Test Coverage

- **Unit tests**: ≥ 80% coverage for core CLI logic (argument parsing, template rendering)
- **Integration tests**: Full project generation flow tested for each template
- **Template validation**: Every generated project must compile and pass `cargo test`
- **Manual QA**: New templates manually tested on clean environments before release

### Test-First for New Features

Non-trivial features (e.g., new CLI modes, template engines) MUST follow TDD:
1. Write test documenting expected behavior
2. Get user approval on test spec
3. Implement feature until test passes
4. Refactor

**Rationale**: TDD prevents "works on my machine" issues and serves as living documentation.

## Performance Standards

- **Project generation**: Single mode < 5s, workspace mode < 10s (cold cache)
- **CLI startup**: < 100ms to first prompt
- **Memory usage**: < 50MB RSS during operation
- **Binary size**: Release build < 10MB (stripped)

**Rationale**: CLI tools compete with `cargo new` (instantaneous). Slow tools frustrate users and reduce adoption.

## Security Requirements

- **No arbitrary code execution**: Templates cannot execute shell commands during rendering
- **Input validation**: All user inputs validated and sanitized
- **Dependency pinning**: Template dependencies locked to specific versions
- **Supply chain**: Templates sourced from trusted repositories or verified checksums

## Governance

### Amendment Process

1. **Proposal**: Open issue with proposed change and rationale
2. **Discussion**: Minimum 7-day discussion period
3. **Approval**: Requires 2 maintainer approvals OR community vote (majority > 66%)
4. **Implementation**: Create PR with migration plan if breaking
5. **Ratification**: Merge PR, update version, amend constitution

### Versioning

- **MAJOR**: Principle removed or redefined, breaking governance changes
- **MINOR**: New principle added, material guidance expansion
- **PATCH**: Clarifications, typo fixes, non-semantic updates

### Compliance Review

- All PRs MUST verify compliance with current constitution
- Review checklist: Constitution compliance checkbox in PR template
- Violations require explicit discussion and amendment approval

### Enforcement

- Constitution supersedes all other documentation
- In case of conflict, constitution takes precedence
- Maintainers reserve right to revert commits violating constitution

---

**Version**: 1.1.0 | **Ratified**: 2025-02-05 | **Last Amended**: 2026-02-06
