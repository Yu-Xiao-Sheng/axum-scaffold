# 需求文档 / Requirements Document

## 简介 / Introduction

本文档定义了 `axum-app-create` CLI 工具 v0.2.0 版本的增强功能需求。v0.2.0 在已完成的 v0.1.1 MVP 基础上新增三大功能：工作区模式（Workspace Mode）、配置预设（Configuration Presets）和 CI/CD 集成（CI/CD Integration）。这些功能扩展了项目脚手架的能力，支持多 crate 工作区架构、快速预设配置和自动化持续集成工作流生成。

## 术语表 / Glossary

- **CLI_Tool**: `axum-app-create` 命令行工具，用于脚手架生成 Axum Web 应用项目
- **Generator**: 项目生成器模块（`generator::project`），负责编排整个项目生成流程
- **TemplateEngine**: Handlebars 模板引擎（`template::engine`），负责渲染 `.hbs` 模板文件
- **TemplateRegistry**: 模板注册表（`template::templates`），管理和提供嵌入式模板文件集合
- **ProjectConfig**: 项目配置结构体（`config::ProjectConfig`），包含项目名称、功能集和子配置
- **FeatureSet**: 功能集结构体（`config::FeatureSet`），定义可选功能的启用状态
- **CliArgs**: CLI 参数结构体（`main.rs::CliArgs`），使用 clap derive 宏解析命令行参数
- **CliOverrides**: CLI 覆盖结构体（`cli::prompts::CliOverrides`），存储命令行标志值用于覆盖交互式提示
- **TemplateContext**: 模板上下文（`template::context::TemplateContext`），为 Handlebars 提供渲染变量
- **ProjectMode**: 项目模式枚举，`single`（单包模式，当前行为）或 `workspace`（工作区模式）
- **Preset**: 配置预设枚举，预定义的功能组合（`minimal`、`api`、`fullstack`）
- **Workspace_Crate**: 工作区中的子 crate，包括 `api`、`domain`、`infrastructure`、`common`

## 需求 / Requirements

### 需求 1：工作区模式 / Workspace Mode

**用户故事 / User Story:** 作为一名 Rust 开发者，我希望能够生成多 crate Cargo workspace 项目，以便使用分层架构（Clean Architecture）组织大型 Axum 应用。

#### 验收标准 / Acceptance Criteria

1. WHEN 用户指定 `--mode workspace` 标志, THE CLI_Tool SHALL 生成一个包含 `[workspace]` 配置的根 `Cargo.toml`，其 `members` 字段包含 `["api", "domain", "infrastructure", "common"]`
2. WHEN 工作区模式被选择, THE Generator SHALL 创建四个子 crate 目录，每个子 crate 包含独立的 `Cargo.toml` 和 `src/lib.rs`（`api` crate 包含 `src/main.rs`）
3. WHEN 工作区模式被选择, THE Generator SHALL 将 `api` crate 设为二进制 crate（包含 HTTP handlers、routes、middleware），并依赖 `domain` 和 `infrastructure` crate
4. WHEN 工作区模式被选择, THE Generator SHALL 将 `domain` crate 设为纯库 crate，仅包含业务逻辑、实体和 trait 定义，不依赖外部框架
5. WHEN 工作区模式被选择, THE Generator SHALL 将 `infrastructure` crate 设为库 crate，包含数据库和外部服务实现，依赖 `domain` crate
6. WHEN 工作区模式被选择, THE Generator SHALL 将 `common` crate 设为库 crate，包含共享类型、工具函数和错误类型
7. WHEN 未指定 `--mode` 标志, THE CLI_Tool SHALL 默认使用 `single` 模式，保持与 v0.1.1 完全一致的行为
8. WHEN 工作区模式下启用数据库功能, THE Generator SHALL 将数据库相关代码放入 `infrastructure` crate
9. WHEN 工作区模式下启用认证功能, THE Generator SHALL 将 JWT 中间件放入 `api` crate，将认证 trait 放入 `domain` crate
10. WHEN 工作区模式下启用 biz-error 功能, THE Generator SHALL 将业务错误类型放入 `common` crate
11. WHEN 在交互模式下未通过 CLI 标志指定模式, THE CLI_Tool SHALL 提示用户选择 `single` 或 `workspace` 模式
12. THE TemplateRegistry SHALL 提供 `get_workspace_mode_templates()` 函数，返回工作区模式的模板文件集合（`HashMap<&str, TemplateFile>`）
13. WHEN 工作区模式的模板被加载, THE TemplateRegistry SHALL 使用 `include_str!()` 宏从 `workspace_mode/` 目录嵌入模板文件

### 需求 2：配置预设 / Configuration Presets

**用户故事 / User Story:** 作为一名开发者，我希望能够使用预定义的功能组合快速创建项目，以便跳过逐个选择功能的繁琐过程。

#### 验收标准 / Acceptance Criteria

1. WHEN 用户指定 `--preset minimal` 标志, THE CLI_Tool SHALL 禁用所有可选功能（database=None, auth=false, biz_error=false, logging 使用默认级别）
2. WHEN 用户指定 `--preset api` 标志, THE CLI_Tool SHALL 启用 database=PostgreSQL, auth=true, logging=info, biz_error=true
3. WHEN 用户指定 `--preset fullstack` 标志, THE CLI_Tool SHALL 启用 database=Both, auth=true, logging=info, biz_error=true
4. WHEN 预设被指定, THE CLI_Tool SHALL 跳过各个功能的交互式提示
5. WHEN 预设与单独的功能标志同时指定（例如 `--preset api --database sqlite`）, THE CLI_Tool SHALL 使用单独标志的值覆盖预设中对应的值
6. WHEN 在交互模式下未指定预设, THE CLI_Tool SHALL 在功能选择之前提示用户选择预设（`minimal`、`api`、`fullstack`、`custom`）
7. WHEN 用户在交互模式下选择 `custom`, THE CLI_Tool SHALL 按照当前 v0.1.1 的行为逐个提示功能选择
8. THE CLI_Tool SHALL 支持预设与 `--mode` 标志组合使用，预设定义功能集，模式定义项目结构

### 需求 3：CI/CD 集成 / CI/CD Integration

**用户故事 / User Story:** 作为一名开发者，我希望生成的项目包含 GitHub Actions CI 工作流文件，以便项目从一开始就具备自动化测试和代码质量检查。

#### 验收标准 / Acceptance Criteria

1. WHEN 用户指定 `--ci` 标志, THE Generator SHALL 在项目中生成 `.github/workflows/ci.yml` 文件
2. WHEN CI 工作流文件被生成, THE Generator SHALL 配置触发条件为 push 到 `main`/`master` 分支和 pull request 事件
3. WHEN CI 工作流文件被生成, THE Generator SHALL 包含四个 job：`check`（cargo check）、`test`（cargo test）、`fmt`（cargo fmt --check）、`clippy`（cargo clippy -- -D warnings）
4. WHEN CI 工作流文件被生成, THE Generator SHALL 配置使用最新稳定版 Rust 工具链
5. WHEN CI 工作流文件被生成, THE Generator SHALL 配置 cargo registry 和构建产物的缓存
6. WHEN 工作区模式下启用 CI, THE Generator SHALL 生成针对所有 workspace members 的 CI 配置（使用 `--workspace` 标志）
7. WHEN 单包模式下启用 CI, THE Generator SHALL 生成标准的单 crate CI 配置
8. WHEN 在交互模式下未通过 CLI 标志指定 CI, THE CLI_Tool SHALL 提示用户是否包含 CI/CD 工作流
9. THE TemplateEngine SHALL 使用 `.hbs` 模板文件渲染 CI 工作流，支持根据 ProjectMode 条件化生成内容

### 需求 4：配置与类型系统扩展 / Configuration and Type System Extension

**用户故事 / User Story:** 作为一名开发者，我希望新功能与现有的配置和类型系统无缝集成，以便代码库保持一致性和可维护性。

#### 验收标准 / Acceptance Criteria

1. THE ProjectConfig SHALL 包含新增字段：`mode: ProjectMode`（默认 `Single`）、`preset: Option<Preset>`、`ci: bool`（默认 `false`）
2. THE FeatureSet SHALL 保持现有字段不变，新增功能通过 ProjectConfig 顶层字段管理
3. THE CliArgs SHALL 新增三个命令行参数：`--mode`（值：single/workspace）、`--preset`（值：minimal/api/fullstack）、`--ci`（布尔标志）
4. THE CliOverrides SHALL 新增字段以支持 `mode`、`preset`、`ci` 的 CLI 覆盖
5. THE TemplateContext SHALL 新增字段 `is_workspace: bool` 和 `has_ci: bool`，用于模板条件渲染
6. WHEN `ProjectMode` 为 `Workspace`, THE TemplateContext SHALL 额外包含各子 crate 的名称和依赖关系信息
7. WHEN 从 ProjectConfig 构建 TemplateContext, THE TemplateContext::from_config() SHALL 正确映射所有新增字段

### 需求 5：生成流程编排 / Generation Flow Orchestration

**用户故事 / User Story:** 作为一名开发者，我希望项目生成流程能够根据模式和配置正确编排所有步骤，以便生成完整且可用的项目。

#### 验收标准 / Acceptance Criteria

1. WHEN 模式为 `single`, THE Generator SHALL 调用 `get_single_mode_templates()` 获取模板（保持现有行为）
2. WHEN 模式为 `workspace`, THE Generator SHALL 调用 `get_workspace_mode_templates()` 获取模板
3. WHEN CI 功能启用, THE Generator SHALL 将 CI 模板追加到当前模式的模板集合中
4. THE Generator SHALL 保持现有的生成流程：验证目录 → 创建目录 → 构建 TemplateContext → 获取模板 → 渲染 → 写入文件 → git init → cargo update
5. WHEN 工作区模式生成完成, THE Generator SHALL 验证根 `Cargo.toml` 和所有子 crate 的 `Cargo.toml` 均已正确生成
6. IF 模板渲染失败, THEN THE Generator SHALL 返回包含模板名称和具体错误信息的 `CliError::Template` 错误

### 需求 6：预设解析与优先级 / Preset Resolution and Priority

**用户故事 / User Story:** 作为一名开发者，我希望 CLI 参数的优先级规则清晰且可预测，以便我能精确控制项目配置。

#### 验收标准 / Acceptance Criteria

1. THE CLI_Tool SHALL 按以下优先级解析配置（从高到低）：单独 CLI 标志 > 预设值 > 交互式提示 > 默认值
2. WHEN 预设被指定且单独标志也被指定, THE CLI_Tool SHALL 先应用预设值，再用单独标志覆盖对应字段
3. WHEN 预设被指定且处于非交互模式, THE CLI_Tool SHALL 使用预设值填充所有未被单独标志覆盖的功能配置
4. WHEN 无效的预设名称被指定, THE CLI_Tool SHALL 输出错误信息并列出有效的预设选项（minimal, api, fullstack），然后以非零退出码退出
5. WHEN 无效的模式名称被指定, THE CLI_Tool SHALL 输出错误信息并列出有效的模式选项（single, workspace），然后以非零退出码退出
