# Tasks: create-axum-app CLI Tool - Phase 1 MVP

**Input**: Design documents from `/specs/001-cli-mvp/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: This feature includes integration tests to verify CLI functionality, template rendering, and generated project compilation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **CLI Tool**: Repository root (`src/`, `tests/`)
- **Generated Projects**: Created in user-specified directories (not in source tree)

---

## Phase 1: Setup (Shared Infrastructure) ✅ COMPLETE

**Purpose**: Project initialization and basic structure

- [X] T001 Create CLI tool project structure: src/main.rs, src/cli/, src/template/, src/generator/, src/utils/, tests/integration/, tests/fixtures/
- [X] T002 Initialize Rust project with clap, inquire, handlebars, thiserror, anyhow dependencies in Cargo.toml
- [X] T003 [P] Configure rustfmt.toml with Rust 2024 edition settings
- [X] T004 [P] Create .github/workflows/ci.yml for CI/CD pipeline
- [X] T005 [P] Create README.md with CLI tool description and installation instructions
- [X] T006 [P] Create LICENSE file (MIT or Apache-2.0)

**✅ Phase 1 Status**: COMPLETED (2025-02-06)
**Commit**: 4865a01 - feat: restructure repository as CLI tool with embedded templates (Phase 1)

---

## Phase 2: Foundational (Blocking Prerequisites) ✅ COMPLETE

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T007 Implement ProjectConfig, FeatureSet, DatabaseOption, and template context structures in src/config/mod.rs
- [X] T008 [P] Implement project name validation (Cargo naming conventions, reserved keywords) in src/utils/validator.rs
- [X] T009 [P] Implement Rust toolchain detection (rustc, cargo check) in src/utils/rust_toolchain.rs
- [X] T010 Setup Handlebars template engine with strict mode and custom helpers in src/template/engine.rs
- [X] T011 Create template context builder (project_name variants, author detection, year) in src/template/context.rs
- [X] T012 Implement CLI argument parsing with clap derive macros in src/cli/args.rs
- [X] T013 Implement non-interactive mode detection (CI env var, TTY check) in src/cli/mod.rs
- [X] T014 [P] Create embedded template constants structure in src/template/templates/mod.rs
- [X] T015 [P] Implement file system operations (directory creation, file writing with error handling) in src/generator/project.rs
- [X] T016 [P] Implement Git initialization (git init, .gitignore creation, initial commit) in src/generator/git.rs
- [X] T017 Create thiserror-based error types (CliError, ValidationError, GenerationError) in src/error.rs
- [X] T018 Configure tracing logging for CLI tool (RUST_LOG support, formatted output) in src/main.rs

**✅ Phase 2 Status**: COMPLETED (2025-02-06)
**Commits**: cf7eb46, f00fa28, c1c0beb

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Quick Start with Zero Configuration (Priority: P1) 🎯 MVP

**Goal**: Generate a minimal, compilable Axum web application with sensible defaults that runs on localhost:8080

**Independent Test**: Run `create-axum-app my-app` accepting all defaults → navigate to my-app → run `cargo run` → verify server starts on port 8080 and responds to GET /health with `{"status":"ok"}`

### Templates for User Story 1

- [X] T019 [P] [US1] Create Cargo.toml.hbs template with axum, tokio, tracing, serde dependencies in src/template/templates/single_mode/Cargo.toml.hbs
- [X] T020 [P] [US1] Create main.rs.hbs template with Axum server setup (127.0.0.1:8080, health endpoint) in src/template/templates/single_mode/src/main.rs.hbs
- [X] T021 [P] [US1] Create lib.rs.hbs template with module exports in src/template/templates/single_mode/src/lib.rs.hbs
- [X] T022 [P] [US1] Create config.rs.hbs template with environment variable loading (HOST, PORT) in src/template/templates/single_mode/src/config.rs.hbs
- [X] T023 [P] [US1] Create health.rs.hbs template with GET /health endpoint in src/template/templates/single_mode/src/handlers/health.rs.hbs
- [X] T024 [P] [US1] Create .env.example template with PORT=8080, HOST=127.0.0.1 in src/template/templates/single_mode/.env.example
- [X] T025 [P] [US1] Create .gitignore template with Rust/Cargo standard patterns in src/template/templates/single_mode/.gitignore
- [X] T026 [P] [US1] Create README.md.hbs template with bilingual (EN/CN) quick start guide in src/template/templates/single_mode/README.md.hbs

### CLI Logic for User Story 1

- [X] T027 [US1] Implement interactive project name prompt with validation in src/cli/prompts.rs (depends on T008)
- [X] T028 [US1] Implement generate_project() orchestration function (template rendering, file writing, Git init) in src/generator/project.rs (depends on T010, T015, T016)
- [X] T029 [US1] Implement main CLI flow: parse args → validate config → prompt for missing values → generate project → print success message in src/main.rs (depends on T012, T013, T027, T028)
- [X] T030 [US1] Add progress indicators during generation (✓ Created Cargo.toml, ✓ Created src/main.rs, etc.) in src/generator/project.rs
- [X] T031 [US1] Implement success message with next steps (cd my-app && cargo run) in src/main.rs

### Tests for User Story 1

- [X] T032 [P] [US1] Integration test: generate basic project and verify Cargo.toml exists in tests/integration/generation_tests.rs
- [X] T033 [P] [US1] Integration test: generated project compiles with `cargo check` in tests/integration/generation_tests.rs
- [X] T034 [P] [US1] Integration test: generated server starts and responds to GET /health in tests/integration/generation_tests.rs
- [X] T035 [P] [US1] Unit test: project name validation (valid names, invalid names, reserved keywords) in tests/unit/validation_tests.rs

**✅ Phase 3 Status**: COMPLETED (2026-02-06)
**Commits**: 8865d22 - feat: complete Phase 3 MVP templates and CLI generation
**Tests**: All 24 unit tests + 5 integration tests passing

**Checkpoint**: At this point, User Story 1 should be fully functional - users can generate and run a minimal Axum app with zero configuration

---

## Phase 4: User Story 2 - Interactive Customization Options (Priority: P2)

**Goal**: Enable optional features (database, authentication, logging, biz-error) via interactive prompts with proper integration

**Independent Test**: Run `create-axum-app --database both --auth --biz-error my-custom-app` → verify generated project includes all selected features without conflicts → verify project compiles and runs

### Prompts for User Story 2

- [ ] T036 [P] [US2] Implement database selection prompt (none/postgresql/sqlite/both) in src/cli/prompts.rs
- [ ] T037 [P] [US2] Implement database URL prompt with validation (postgresql:// or sqlite:// format) in src/cli/prompts.rs
- [ ] T038 [P] [US2] Implement authentication confirmation prompt in src/cli/prompts.rs
- [ ] T039 [P] [US2] Implement biz-error confirmation prompt in src/cli/prompts.rs
- [ ] T040 [P] [US2] Implement log level selection prompt (trace/debug/info/warn/error) in src/cli/prompts.rs

### Database Feature Templates

- [ ] T041 [P] [US2] Create database-enabled Cargo.toml.hbs variant with sqlx dependencies in src/template/templates/database/Cargo.toml.hbs
- [ ] T042 [P] [US2] Create migrations/001_create_users.sql template (PostgreSQL and SQLite) in src/template/templates/database/migrations/001_create_users.sql.hbs
- [ ] T043 [P] [US2] Create db.rs.hbs template with sqlx Pool setup and PgPool/SqlitePool types in src/template/templates/database/src/db.rs.hbs
- [ ] T044 [P] [US2] Create config.rs.hbs variant with DATABASE_URL support in src/template/templates/database/src/config.rs.hbs
- [ ] T045 [P] [US2] Create .env.example variant with DATABASE_URL template in src/template/templates/database/.env.example

### Authentication Feature Templates

- [ ] T046 [P] [US2] Create auth-enabled Cargo.toml.hbs variant with jsonwebtoken, bcrypt dependencies in src/template/templates/auth/Cargo.toml.hbs
- [ ] T047 [P] [US2] Create user.rs.hbs template with User model (id, username, email, password_hash) in src/template/templates/auth/src/models/user.rs.hbs
- [ ] T048 [P] [US2] Create auth.rs.hbs template with Claims struct, JWT generation/validation in src/template/templates/auth/src/middleware/auth.rs.hbs
- [ ] T049 [P] [US2] Create login.rs.hbs template with POST /auth/login and POST /auth/register endpoints in src/template/templates/auth/src/handlers/login.rs.hbs
- [ ] T050 [P] [US2] Create .env.example variant with JWT_SECRET template in src/template/templates/auth/.env.example

### Logging Feature Templates

- [ ] T051 [P] [US2] Create logging-enabled Cargo.toml.hbs variant with tracing-subscriber, env-filter in src/template/templates/logging/Cargo.toml.hbs
- [ ] T052 [P] [US2] Create main.rs.hbs variant with tracing-subscriber initialization (LOG_LEVEL support) in src/template/templates/logging/src/main.rs.hbs

### Business Error Feature Templates

- [ ] T053 [P] [US2] Create biz-error-enabled Cargo.toml.hbs variant with biz-error dependency in src/template/templates/biz_error/Cargo.toml.hbs
- [ ] T054 [P] [US2] Create biz_errors.yaml.hbs template with common error codes (validation, not found, unauthorized, database) in English and Chinese in src/template/templates/biz_error/biz_errors.yaml.hbs
- [ ] T055 [P] [US2] Create error_handler.rs.hbs template with biz-error Axum integration middleware in src/template/templates/biz_error/src/error_handler.rs.hbs
- [ ] T056 [P] [US2] Create example handler showing biz-error usage with custom data field in src/template/templates/biz_error/src/handlers/examples.rs.hbs

### CLI Logic for User Story 2

- [ ] T057 [US2] Implement feature flag integration in template context (add conditional blocks to templates) in src/template/context.rs
- [ ] T058 [US2] Implement conditional template rendering based on FeatureSet in src/template/engine.rs
- [ ] T059 [US2] Update main CLI flow to collect feature prompts before generation in src/main.rs (depends on T036-T040)

### Tests for User Story 2

- [ ] T060 [P] [US2] Integration test: generate project with database feature → verify sqlx dependency and migrations in tests/integration/feature_tests.rs
- [ ] T061 [P] [US2] Integration test: generate project with auth feature → verify JWT endpoints and user model in tests/integration/feature_tests.rs
- [ ] T062 [P] [US2] Integration test: generate project with biz-error feature → verify error definitions and YAML file in tests/integration/feature_tests.rs
- [ ] T063 [P] [US2] Integration test: generate project with all features → verify no conflicts, compiles successfully in tests/integration/feature_tests.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - users can generate projects with optional features

---

## Phase 5: User Story 3 - Clear Error Messages and Guidance (Priority: P3)

**Goal**: Provide actionable error messages with suggested fixes for common failure scenarios

**Independent Test**: Trigger error conditions (invalid name, existing dir, missing Rust) → verify each error shows clear message with suggested fix

### Error Messages for User Story 3

- [ ] T064 [P] [US3] Implement invalid project name error with naming rules explanation in src/utils/validator.rs
- [ ] T065 [P] [US3] Implement directory exists error with overwrite/cancel prompt in src/generator/project.rs
- [ ] T066 [P] [US3] Implement Rust toolchain missing error with installation link (https://rustup.rs/) in src/utils/rust_toolchain.rs
- [ ] T067 [P] [US3] Implement permission denied error with alternative directory suggestion in src/generator/project.rs
- [ ] T068 [P] [US3] Implement template rendering error with file path and line number in src/template/engine.rs
- [ ] T069 [P] [US3] Implement generic generation failure error with troubleshooting link in src/generator/project.rs
- [ ] T070 [US3] Add --help flag usage instructions to all error messages in src/cli/args.rs

### Tests for User Story 3

- [ ] T071 [P] [US3] Unit test: invalid project name errors (empty, starts with digit, special chars, reserved keyword) in tests/unit/error_tests.rs
- [ ] T072 [P] [US3] Unit test: directory exists error handling and overwrite prompt in tests/unit/error_tests.rs
- [ ] T073 [P] [US3] Unit test: Rust toolchain missing error with installation instructions in tests/unit/error_tests.rs
- [ ] T074 [P] [US3] Integration test: trigger permission denied error → verify helpful message in tests/integration/error_tests.rs

**Checkpoint**: All user stories should now be independently functional with comprehensive error handling

---

## Phase 6: User Story 4 - Project Verification and Quick Start Guide (Priority: P3)

**Goal**: Display clear next steps after generation and include comprehensive README in generated projects

**Independent Test**: Generate project → verify CLI prints success message with cd and cargo run commands → verify README contains quick start, structure overview, next steps

### Post-Generation Output for User Story 4

- [ ] T075 [US4] Create success message template with ASCII art star emoji, next steps commands, and happy hacking message in src/generator/project.rs
- [ ] T076 [US4] Add project name, generation time, and command summary to success message in src/generator/project.rs

### Enhanced README Templates for User Story 4

- [ ] T077 [P] [US4] Update README.md.hbs with quick start section (cargo run, cargo test commands) in src/template/templates/single_mode/README.md.hbs
- [ ] T078 [P] [US4] Add project structure overview to README.md.hbs (file tree with descriptions) in src/template/templates/single_mode/README.md.hbs
- [ ] T079 [P] [US4] Add next steps section to README.md.hbs (add endpoints, configure database, deploy) in src/template/templates/single_mode/README.md.hbs
- [ ] T080 [P] [US4] Add feature-specific README sections (database setup, auth usage, biz-error customization) in src/template/templates/*/README.md.hbs

### Inline Code Comments for User Story 4

- [ ] T081 [P] [US4] Add explanatory comments to main.rs.hbs (server setup, router, state) in src/template/templates/single_mode/src/main.rs.hbs
- [ ] T082 [P] [US4] Add explanatory comments to config.rs.hbs (environment variables, validation) in src/template/templates/single_mode/src/config.rs.hbs
- [ ] T083 [P] [US4] Add explanatory comments to handler templates (endpoint purpose, request/response formats) in src/template/templates/single_mode/src/handlers/*.rs.hbs

### Tests for User Story 4

- [ ] T084 [P] [US4] Integration test: verify success message contains correct commands and project name in tests/integration/output_tests.rs
- [ ] T085 [P] [US4] Integration test: following printed commands (cd, cargo run) successfully starts server in tests/integration/output_tests.rs
- [ ] T086 [P] [US4] Integration test: generated README contains all sections and is valid markdown in tests/integration/output_tests.rs

**Checkpoint**: All user stories complete with excellent onboarding experience

---

## Phase 7: User Story 5 - Standardized Error Response Format (Priority: P2)

**Goal**: Integrate biz-error crate for production-ready error handling with type-safe codes and i18n

**Independent Test**: Generate project with --biz-error → trigger various errors → verify all responses follow {code, msg, data} format with correct HTTP status codes

### Biz-Error Integration for User Story 5

- [ ] T087 [US5] Update template context to include biz_error config (languages, error_def_path) in src/template/context.rs
- [ ] T088 [US5] Add conditional biz-error template blocks (only include if feature enabled) in src/template/engine.rs
- [ ] T089 [P] [US5] Create biz_errors.yaml.hbs with 10+ common error codes (VALIDATION_ERROR, INVALID_INPUT, RESOURCE_NOT_FOUND, USER_NOT_FOUND, UNAUTHORIZED, INVALID_CREDENTIALS, FORBIDDEN, DATABASE_ERROR, CONNECTION_ERROR, INTERNAL_ERROR) in src/template/templates/biz_error/biz_errors.yaml.hbs
- [ ] T090 [P] [US5] Add bilingual messages (en, zh-CN) for all error codes in biz_errors.yaml.hbs
- [ ] T091 [P] [US5] Create error handler middleware template with BizError trait implementation in src/template/templates/biz_error/src/error_handler.rs.hbs
- [ ] T092 [P] [US5] Create example error-returning handler demonstrating validation error with data field in src/template/templates/biz_error/src/handlers/examples.rs.hbs
- [ ] T093 [US5] Update main.rs.hbs to include error handler middleware when biz-error enabled in src/template/templates/biz_error/src/main.rs.hbs

### Tests for User Story 5

- [ ] T094 [P] [US5] Integration test: verify biz-errors.yaml contains all required error codes and translations in tests/integration/biz_error_tests.rs
- [ ] T095 [P] [US5] Integration test: trigger validation error → verify {code: 1001, msg: "Validation failed", data: {...}} response in tests/integration/biz_error_tests.rs
- [ ] T096 [P] [US5] Integration test: trigger not found error → verify 404 status and {code: 2001} response in tests/integration/biz_error_tests.rs
- [ ] T097 [P] [US5] Integration test: trigger unauthorized error → verify 401 status and {code: 3001} response in tests/integration/biz_error_tests.rs
- [ ] T098 [P] [US5] Integration test: error with custom data field → verify data contains validation details in tests/integration/biz_error_tests.rs

**Checkpoint**: User Story 5 complete - generated projects have production-ready error handling

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories and final validation

### Documentation

- [ ] T099 [P] Create comprehensive CLI tool README.md with installation, usage, examples, and troubleshooting
- [ ] T100 [P] Create CHANGELOG.md following Keep a Changelog format
- [ ] T101 [P] Add inline documentation to all public API modules (rustdoc comments)
- [ ] T102 [P] Create CONTRIBUTING.md with development setup, code style, PR process

### Code Quality

- [ ] T103 Run `cargo clippy -- -D warnings` and fix all warnings
- [ ] T104 Run `cargo fmt` and ensure consistent formatting
- [ ] T105 Add missing rustdoc documentation to all public structs, enums, functions
- [ ] T106 Refactor any code duplication identified during implementation

### Performance

- [ ] T107 Profile CLI startup time with hyperfine → optimize if >100ms target not met
- [ ] T108 Profile project generation time → optimize if >10s target not met
- [ ] T109 Check release binary size → enable LTO, codegen-units=1, strip if >10MB
- [ ] T110 Profile memory usage during generation → optimize if >50MB RSS target not met

### Security

- [ ] T111 Run `cargo audit` to check for security vulnerabilities in dependencies
- [ ] T112 Validate that JWT_SECRET in templates is clearly marked as example only
- [ ] T113 Ensure no hardcoded secrets or credentials in any templates or code
- [ ] T114 Add .env files to generated .gitignore templates

### Validation

- [ ] T115 Run all integration tests and verify 100% pass rate
- [ ] T116 Generate test projects with all feature combinations → verify all compile and run
- [ ] T117 Test on all target platforms (Linux, macOS, Windows) via GitHub Actions
- [ ] T118 Validate quickstart.md instructions by following them from scratch

### Final Checks

- [ ] T119 Verify generated projects pass `cargo clippy` with no warnings
- [ ] T120 Verify generated projects pass `cargo fmt --check`
- [ ] T121 Verify generated projects have comprehensive README with all sections
- [ ] T122 Test edge cases: long names (>100 chars), non-ASCII paths, concurrent invocations, Ctrl+C handling

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase - INDEPENDENT of other user stories
- **User Story 2 (Phase 4)**: Depends on Foundational phase - INDEPENDENT of other user stories (can run in parallel with US1, US3, US4)
- **User Story 3 (Phase 5)**: Depends on Foundational phase - INDEPENDENT of other user stories (can run in parallel)
- **User Story 4 (Phase 6)**: Depends on Foundational phase - INDEPENDENT of other user stories (can run in parallel)
- **User Story 5 (Phase 7)**: Depends on Foundational phase + optional to Story 2 (biz-error is optional feature in US2)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1) - MVP**: No dependencies on other user stories
- **User Story 2 (P2) - Customization**: No dependencies on other user stories
- **User Story 3 (P3) - Error Messages**: No dependencies on other user stories
- **User Story 4 (P3) - Verification**: No dependencies on other user stories
- **User Story 5 (P2) - Biz-Error**: Optional feature from US2, but can be implemented independently

### Critical Paths

**MVP Path (User Story 1 only)**:
1. Setup (T001-T006) → Foundational (T007-T018) → User Story 1 (T019-T035)

**Full Implementation Path**:
1. Setup (T001-T006) → Foundational (T007-T018) →
2. US1 (T019-T035) + US2 (T036-T063) + US3 (T064-T074) + US4 (T075-T086) + US5 (T087-T098) [can run in parallel] →
3. Polish (T099-T122)

### Within Each User Story

- Templates marked [P] can be created in parallel
- Integration tests depend on templates being complete
- CLI orchestration code depends on templates and prompts being complete
- Tests should be written alongside implementation (TDD optional but recommended)

### Parallel Opportunities

**Setup Phase**:
- T003, T004, T005, T006 can run in parallel (different files)

**Foundational Phase**:
- T008, T009 can run in parallel (different modules)
- T014 can run in parallel with T015, T016 (different concerns)
- After T007 (config structures), T010-T017 can mostly run in parallel

**User Story 1**:
- T019-T026 (all templates) can run in parallel
- T032-T035 (all tests) can run in parallel

**User Story 2**:
- T036-T040 (all prompts) can run in parallel
- T041-T045 (database templates) can run in parallel
- T046-T050 (auth templates) can run in parallel
- T051-T052 (logging templates) can run in parallel
- T053-T056 (biz-error templates) can run in parallel
- T060-T063 (all tests) can run in parallel

**User Story 3**:
- T064-T069 (all error messages) can run in parallel
- T071-T074 (all tests) can run in parallel

**User Story 4**:
- T077-T083 (all README and comment updates) can run in parallel
- T084-T086 (all tests) can run in parallel

**User Story 5**:
- T089-T092 (biz-error templates) can run in parallel
- T094-T098 (all tests) can run in parallel

**Polish Phase**:
- T099-T102 (all documentation) can run in parallel
- T103-T106 (all code quality) can run sequentially
- T107-T110 (all performance) can run sequentially
- T111-T114 (all security) can run in parallel
- T115-T118 (all validation) can run sequentially
- T119-T122 (all final checks) can run sequentially

---

## Parallel Example: User Story 2 Templates

```bash
# Launch all database feature templates together:
Task T041: "Create database-enabled Cargo.toml.hbs variant"
Task T042: "Create migrations/001_create_users.sql template"
Task T043: "Create db.rs.hbs template with sqlx Pool setup"
Task T044: "Create config.rs.hbs variant with DATABASE_URL support"
Task T045: "Create .env.example variant with DATABASE_URL template"

# Launch all authentication feature templates together:
Task T046: "Create auth-enabled Cargo.toml.hbs variant"
Task T047: "Create user.rs.hbs template with User model"
Task T048: "Create auth.rs.hbs template with Claims struct"
Task T049: "Create login.rs.hbs template with auth endpoints"
Task T050: "Create .env.example variant with JWT_SECRET template"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only) - RECOMMENDED FOR INITIAL RELEASE

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T018) - CRITICAL
3. Complete Phase 3: User Story 1 (T019-T035)
4. **STOP and VALIDATE**: Test User Story 1 independently
   - Generate a project: `create-axum-app my-test-app`
   - Verify it compiles: `cd my-test-app && cargo build`
   - Verify it runs: `cargo run` → GET http://127.0.0.1:8080/health
   - Measure performance: generation time <10s, startup <100ms
5. Deploy/demo MVP to validate core concept
6. Gather feedback before proceeding to other user stories

**Benefits**:
- Fast time to first working version
- Validates core value proposition early
- Reduces risk by validating approach before investing in optional features
- Allows user feedback to influence feature priorities

### Incremental Delivery (Add Features After MVP)

1. MVP (User Story 1) → Deploy/Demo → Gather feedback
2. Add User Story 5 (Biz-Error) → Test independently → Deploy/Demo
3. Add User Story 2 (Customization) → Test independently → Deploy/Demo
4. Add User Story 3 (Error Messages) → Test independently → Deploy/Demo
5. Add User Story 4 (Verification) → Test independently → Deploy/Demo
6. Polish & Performance (Phase 8) → Final release

**Rationale**:
- US5 (Biz-Error) is high priority (P2) and adds production readiness
- US2 (Customization) is high priority (P2) but adds complexity
- US3 and US4 are lower priority (P3) but improve UX significantly
- Each story adds value without breaking previous stories

### Parallel Team Strategy

With 3-5 developers:

1. **Team completes Setup + Foundational together** (T001-T018)
2. **Once Foundational is done, split by user story**:
   - **Developer A**: User Story 1 (MVP core) - T019-T035
   - **Developer B**: User Story 2 (Optional features) - T036-T063
   - **Developer C**: User Story 3 (Error handling) - T064-T074
   - **Developer D**: User Story 4 (Documentation) - T075-T086
   - **Developer E**: User Story 5 (Biz-error) - T087-T098
3. **Stories complete and integrate independently** (no merge conflicts expected)
4. **Team converges for Polish phase** (T099-T122)

**Coordination**:
- Daily sync on template variable naming conventions
- Shared code review for foundational components (T007-T018)
- Integration testing when stories complete
- Feature flag configuration to enable/disable stories during development

---

## Summary

**Total Tasks**: 122 tasks

**Tasks per User Story**:
- Setup: 6 tasks
- Foundational: 12 tasks (BLOCKS all stories)
- User Story 1 (Quick Start): 17 tasks (MVP!)
- User Story 2 (Customization): 28 tasks
- User Story 3 (Error Messages): 11 tasks
- User Story 4 (Verification): 12 tasks
- User Story 5 (Biz-Error): 12 tasks
- Polish: 24 tasks

**Parallel Opportunities**: 60+ tasks marked [P] can run in parallel with proper team size

**MVP Scope** (Recommended for v1.0.0):
- Setup (6 tasks) + Foundational (12 tasks) + User Story 1 (17 tasks) = 35 tasks
- Estimated effort: 1-2 weeks for solo developer, 3-5 days for small team

**Full Release Scope**:
- All 122 tasks
- Estimated effort: 4-6 weeks for solo developer, 2-3 weeks for small team

**Critical Success Factors**:
1. ✅ Complete Foundational phase before any user story work
2. ✅ Validate User Story 1 independently before adding features
3. ✅ Use [P] markers to parallelize work when team size allows
4. ✅ Run integration tests after each user story completion
5. ✅ Measure performance against targets (<10s generation, <100ms startup)

---

## Notes

- [P] tasks = different files, no dependencies on incomplete tasks
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Template tasks within a story can run in parallel (different .hbs files)
- Commit after each task or logical group for easy rollback
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
