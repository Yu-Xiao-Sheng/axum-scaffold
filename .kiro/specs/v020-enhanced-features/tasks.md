# 实施计划 / Implementation Plan: v0.2.0 Enhanced Features

## 概述 / Overview

本计划将 v0.2.0 的三大功能（工作区模式、配置预设、CI/CD 集成）分解为增量式编码任务。每个任务基于前一个任务构建，确保代码始终可编译。实现语言为 Rust，使用现有的 `cargo test` 测试框架，新增 `proptest` 用于属性测试。

## Tasks

- [x] 1. 类型系统与配置扩展 / Type System and Config Extensions
  - [x] 1.1 在 `src/config/mod.rs` 中新增 `ProjectMode` 枚举和 `Preset` 枚举
    - 添加 `ProjectMode` 枚举（`Single`/`Workspace`），实现 `Default`（Single）、`Display`、`Clone`、`Copy`、`PartialEq`、`Eq`、`Serialize`、`Deserialize`
    - 添加 `Preset` 枚举（`Minimal`/`Api`/`Fullstack`），实现 `Display`、`Clone`、`Copy`、`PartialEq`、`Eq`、`Serialize`、`Deserialize`
    - 实现 `Preset::to_feature_set()` 方法，按设计文档定义映射各预设到 `FeatureSet`
    - _Requirements: 4.1, 2.1, 2.2, 2.3_

  - [x] 1.2 扩展 `ProjectConfig` 和 `CliOverrides` 结构体
    - 在 `src/config/mod.rs` 的 `ProjectConfig` 中新增字段：`mode: ProjectMode`（默认 `Single`）、`preset: Option<Preset>`、`ci: bool`（默认 `false`）
    - 更新 `ProjectConfig::default()` 包含新字段默认值
    - 在 `src/cli/prompts.rs` 的 `CliOverrides` 中新增字段：`mode: Option<ProjectMode>`、`preset: Option<Preset>`、`ci: Option<bool>`
    - 更新 `CliOverrides::default()` 包含新字段
    - _Requirements: 4.1, 4.2, 4.4_

  - [x] 1.3 为 `Preset::to_feature_set()` 编写属性测试
    - 在 `Cargo.toml` 的 `[dev-dependencies]` 中添加 `proptest = "1"`
    - 在 `src/config/mod.rs` 的 `#[cfg(test)]` 模块中编写属性测试
    - **Property 1: 预设到功能集映射一致性** — *For any* Preset 值，`to_feature_set()` 调用两次应返回相同的 `FeatureSet`（幂等性）
    - **Validates: Requirements 2.1, 2.2, 2.3**

  - [x] 1.4 为 `ProjectMode` 和 `Preset` 编写单元测试
    - 测试 `ProjectMode::default()` 返回 `Single`
    - 测试 `Preset::Minimal.to_feature_set()` 返回全部禁用的功能集
    - 测试 `Preset::Api.to_feature_set()` 返回 PostgreSQL + auth + biz_error
    - 测试 `Preset::Fullstack.to_feature_set()` 返回 Both + auth + biz_error
    - 测试 `Display` trait 输出正确的字符串
    - _Requirements: 2.1, 2.2, 2.3, 4.1_

- [x] 2. CLI 参数与提示扩展 / CLI Arguments and Prompts Extension
  - [x] 2.1 在 `src/main.rs` 的 `CliArgs` 中新增 CLI 参数
    - 添加 `--mode` 参数（`Option<String>`，value_name = "MODE"）
    - 添加 `--preset` 参数（`Option<String>`，value_name = "PRESET"）
    - 添加 `--ci` 参数（布尔标志）
    - 在 `main()` 中添加 `--mode` 字符串到 `ProjectMode` 的解析逻辑，无效值输出错误并退出
    - 在 `main()` 中添加 `--preset` 字符串到 `Preset` 的解析逻辑，无效值输出错误并退出
    - 将解析结果传入 `CliOverrides` 的新字段
    - 更新版本号显示为 `v0.2.0`
    - _Requirements: 4.3, 4.4, 6.4, 6.5_

  - [x] 2.2 在 `src/cli/prompts.rs` 中扩展交互式提示
    - 新增 `prompt_project_mode()` 函数：提示用户选择 `Single package` / `Workspace (multi-crate)`
    - 新增 `prompt_preset()` 函数：提示用户选择 `Minimal` / `API` / `Fullstack` / `Custom`
    - 新增 `prompt_ci()` 函数：提示用户是否生成 CI/CD 工作流
    - 修改 `prompt_project_config()` 函数：
      - 在项目描述之后、功能选择之前插入模式选择提示
      - 在模式选择之后插入预设选择提示
      - 当预设为 `Custom` 或未指定时才进入逐个功能提示
      - 在功能选择之后插入 CI 提示
      - 实现预设解析优先级：单独 CLI 标志 > 预设值 > 交互式提示 > 默认值
    - 所有提示文本使用双语（英文 + 中文）
    - _Requirements: 1.11, 2.4, 2.5, 2.6, 2.7, 2.8, 3.8, 6.1, 6.2, 6.3_

- [x] 3. 检查点 / Checkpoint - 确保类型系统和 CLI 层编译通过
  - 运行 `cargo check` 确保所有修改编译通过
  - 运行 `cargo test` 确保现有 41 个测试全部通过
  - 如有问题请询问用户

- [x] 4. 模板上下文扩展 / Template Context Extension
  - [x] 4.1 扩展 `src/template/context.rs` 中的 `TemplateContext`
    - 在 `TemplateContext` 结构体中新增 `is_workspace: bool` 和 `has_ci: bool` 字段
    - 更新 `TemplateContext::from_config()` 方法：映射 `config.mode == ProjectMode::Workspace` 到 `is_workspace`，映射 `config.ci` 到 `has_ci`
    - _Requirements: 4.5, 4.6, 4.7_

  - [x] 4.2 为 `TemplateContext::from_config()` 新字段编写单元测试
    - 测试 `mode: Single` 时 `is_workspace == false`
    - 测试 `mode: Workspace` 时 `is_workspace == true`
    - 测试 `ci: true` 时 `has_ci == true`
    - 测试 `ci: false` 时 `has_ci == false`
    - _Requirements: 4.5, 4.7_

- [x] 5. 工作区模式模板 / Workspace Mode Templates
  - [x] 5.1 创建工作区根目录模板文件
    - 创建 `src/template/templates/workspace_mode/root/Cargo.toml.hbs`：包含 `[workspace]` 配置，members = ["api", "domain", "infrastructure", "common"]，resolver = "2"，workspace.package 共享 version/edition/authors
    - 创建 `src/template/templates/workspace_mode/root/.env.example.hbs`：复用 single_mode 的环境变量模板，适配工作区结构
    - 创建 `src/template/templates/workspace_mode/root/.gitignore`：与 single_mode 一致
    - 创建 `src/template/templates/workspace_mode/root/.dockerignore`：与 single_mode 一致
    - 创建 `src/template/templates/workspace_mode/root/README.md.hbs`：双语 README，说明工作区结构
    - 创建 `src/template/templates/workspace_mode/root/Dockerfile.hbs`：适配工作区的多阶段构建
    - _Requirements: 1.1, 1.7_

  - [x] 5.2 创建 `api` crate 模板文件
    - 创建 `src/template/templates/workspace_mode/api/Cargo.toml.hbs`：二进制 crate，依赖 domain + infrastructure + common，包含 axum/tokio/tracing/serde 等依赖，条件包含 jsonwebtoken（`{{#if has_auth}}`）
    - 创建 `src/template/templates/workspace_mode/api/src/main.rs.hbs`：HTTP 服务入口，路由注册，条件包含 auth 中间件
    - 创建 `src/template/templates/workspace_mode/api/src/lib.rs.hbs`：模块声明
    - 创建 `src/template/templates/workspace_mode/api/src/config.rs.hbs`：应用配置
    - 创建 `src/template/templates/workspace_mode/api/src/handlers/mod.rs.hbs`：handler 模块声明
    - 创建 `src/template/templates/workspace_mode/api/src/handlers/health.rs.hbs`：健康检查端点
    - 创建 `src/template/templates/workspace_mode/api/src/handlers/auth.rs.hbs`：认证端点（`{{#if has_auth}}`）
    - 创建 `src/template/templates/workspace_mode/api/src/middleware/mod.rs.hbs`：中间件模块（`{{#if has_auth}}` JWT 中间件）
    - _Requirements: 1.2, 1.3, 1.9_

  - [x] 5.3 创建 `domain` crate 模板文件
    - 创建 `src/template/templates/workspace_mode/domain/Cargo.toml.hbs`：纯库 crate，最小依赖（serde, thiserror），不依赖外部框架
    - 创建 `src/template/templates/workspace_mode/domain/src/lib.rs.hbs`：模块声明（entities, traits）
    - 创建 `src/template/templates/workspace_mode/domain/src/entities/mod.rs.hbs`：实体定义
    - 创建 `src/template/templates/workspace_mode/domain/src/traits/mod.rs.hbs`：Repository trait 定义（`{{#if has_auth}}` 包含认证 trait）
    - _Requirements: 1.4, 1.9_

  - [x] 5.4 创建 `infrastructure` crate 模板文件
    - 创建 `src/template/templates/workspace_mode/infrastructure/Cargo.toml.hbs`：库 crate，依赖 domain，条件包含 sqlx（`{{#if has_database}}`）
    - 创建 `src/template/templates/workspace_mode/infrastructure/src/lib.rs.hbs`：模块声明
    - 创建 `src/template/templates/workspace_mode/infrastructure/src/db.rs.hbs`：数据库连接和操作（`{{#if has_database}}`）
    - _Requirements: 1.5, 1.8_

  - [x] 5.5 创建 `common` crate 模板文件
    - 创建 `src/template/templates/workspace_mode/common/Cargo.toml.hbs`：库 crate，共享类型
    - 创建 `src/template/templates/workspace_mode/common/src/lib.rs.hbs`：模块声明
    - 创建 `src/template/templates/workspace_mode/common/src/error.rs.hbs`：业务错误类型（`{{#if has_biz_error}}`）
    - _Requirements: 1.6, 1.10_

  - [x] 5.6 在 `src/template/templates/mod.rs` 中注册工作区模板
    - 实现 `get_workspace_mode_templates()` 函数，返回 `HashMap<&'static str, TemplateFile>`
    - 使用 `include_str!()` 嵌入 `workspace_mode/` 目录下所有模板文件
    - 模板路径键使用相对于项目根的路径（如 `"api/Cargo.toml"`、`"domain/src/lib.rs"`）
    - _Requirements: 1.12, 1.13_

- [x] 6. CI/CD 模板 / CI/CD Templates
  - [x] 6.1 创建 CI 工作流模板文件
    - 创建 `src/template/templates/ci/.github/workflows/ci.yml.hbs`
    - 模板包含 4 个 job：check、test、fmt、clippy
    - 使用 `{{#if is_workspace}}--workspace{{/if}}` 条件渲染 workspace 标志
    - 配置触发条件：push 到 main/master + pull_request
    - 配置 `dtolnay/rust-toolchain@stable` 和 `Swatinem/rust-cache@v2`
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.9_

  - [x] 6.2 在 `src/template/templates/mod.rs` 中注册 CI 模板
    - 实现 `get_ci_templates()` 函数，返回 `HashMap<&'static str, TemplateFile>`
    - 使用 `include_str!()` 嵌入 CI 模板
    - _Requirements: 3.9_

- [x] 7. 生成器流程修改 / Generator Flow Modifications
  - [x] 7.1 修改 `src/generator/project.rs` 中的 `generate_project()` 函数
    - 导入 `get_workspace_mode_templates` 和 `get_ci_templates`
    - 根据 `config.mode` 选择模板集：`Single` 调用 `get_single_mode_templates()`，`Workspace` 调用 `get_workspace_mode_templates()`
    - 当 `config.ci == true` 时，将 `get_ci_templates()` 的结果 extend 到模板集合中
    - 保持现有的渲染、写入、git init、cargo update 流程不变
    - _Requirements: 5.1, 5.2, 5.3, 5.4_

  - [x] 7.2 更新 `src/generator/project.rs` 中的 `get_success_message()` 函数
    - 当工作区模式时，在成功消息中显示工作区结构说明
    - 当 CI 启用时，在成功消息中提示 CI 工作流已生成
    - _Requirements: 5.4_

- [x] 8. 检查点 / Checkpoint - 确保完整生成流程可用
  - 运行 `cargo check` 确保所有修改编译通过
  - 运行 `cargo test` 确保现有测试全部通过
  - 如有问题请询问用户

- [x] 9. 集成测试 / Integration Tests
  - [x] 9.1 在 `tests/generation_tests.rs` 中添加工作区模式生成测试
    - 测试：使用 `mode: Workspace` 生成项目，验证根 `Cargo.toml` 包含 `[workspace]` 和 `members`
    - 测试：验证四个子 crate 目录（api/domain/infrastructure/common）及其 `Cargo.toml` 和 `src/lib.rs` 均存在
    - 测试：验证 `api` crate 包含 `src/main.rs`
    - 测试：验证 `mode: Single` 保持与 v0.1.1 完全一致的行为（现有测试覆盖）
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7_

  - [x] 9.2 在 `tests/generation_tests.rs` 中添加工作区模式 + 功能组合测试
    - 测试：工作区模式 + 数据库功能，验证 `infrastructure/src/db.rs` 存在
    - 测试：工作区模式 + 认证功能，验证 `api/src/handlers/auth.rs` 和 `api/src/middleware/mod.rs` 存在
    - 测试：工作区模式 + biz-error 功能，验证 `common/src/error.rs` 存在
    - _Requirements: 1.8, 1.9, 1.10_

  - [x] 9.3 在 `tests/generation_tests.rs` 中添加预设测试
    - 测试：使用 `Preset::Minimal` 生成项目，验证无数据库文件、无认证文件
    - 测试：使用 `Preset::Api` 生成项目，验证包含 PostgreSQL 数据库文件和认证文件
    - 测试：使用 `Preset::Fullstack` 生成项目，验证包含双数据库和所有功能文件
    - _Requirements: 2.1, 2.2, 2.3_

  - [x] 9.4 在 `tests/generation_tests.rs` 中添加 CI 模板测试
    - 测试：`ci: true` 时验证 `.github/workflows/ci.yml` 文件存在
    - 测试：`ci: false` 时验证 `.github/workflows/ci.yml` 文件不存在
    - 测试：工作区模式 + CI 时验证 ci.yml 包含 `--workspace` 标志
    - 测试：单包模式 + CI 时验证 ci.yml 不包含 `--workspace` 标志
    - _Requirements: 3.1, 3.6, 3.7_

  - [x] 9.5 在 `tests/generation_tests.rs` 中添加编译验证测试
    - 测试：工作区模式基础项目（无可选功能）通过 `cargo check --workspace`
    - 测试：工作区模式全功能项目通过 `cargo check --workspace`
    - _Requirements: 5.5_

  - [x] 9.6 为预设覆盖逻辑编写属性测试
    - 在 `src/cli/prompts.rs` 的 `#[cfg(test)]` 模块中编写
    - **Property 2: 单独标志覆盖预设值** — *For any* Preset 和 *any* 单独 CLI 标志组合，解析后的 FeatureSet 中被标志覆盖的字段应等于标志值，未被覆盖的字段应等于预设值
    - **Validates: Requirements 2.5, 6.1, 6.2**

- [x] 10. 文档与版本更新 / Documentation and Version Bump
  - [x] 10.1 更新 `Cargo.toml` 版本号为 `"0.2.0"`
    - _Requirements: N/A_

  - [x] 10.2 更新 `README.md` 文档
    - 添加工作区模式使用说明和示例命令
    - 添加配置预设使用说明和示例命令
    - 添加 CI/CD 集成使用说明
    - 更新功能列表和命令行参数表
    - _Requirements: N/A_

  - [x] 10.3 更新 `CHANGELOG.md`
    - 添加 v0.2.0 版本条目，列出三大新功能
    - _Requirements: N/A_

- [ ] 11. 最终检查点 / Final Checkpoint
  - 运行 `cargo check` 确保编译通过
  - 运行 `cargo test` 确保所有测试通过（包括新增测试）
  - 运行 `cargo clippy -- -D warnings` 确保无 lint 警告
  - 运行 `cargo fmt --check` 确保代码格式正确
  - 如有问题请询问用户

## Notes

- 所有任务均为必需，包括属性测试和单元测试
- 每个任务引用了具体的需求编号以确保可追溯性
- 检查点任务确保增量验证
- 属性测试使用 `proptest` 库验证通用正确性属性
- 单元测试验证具体示例和边界条件
