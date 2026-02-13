# 需求文档 / Requirements Document

## 简介 / Introduction

本文档定义了 `axum-app-create` CLI 工具 v0.3.0 版本的需求。v0.3.0 在已完成的 v0.2.0 基础上新增三大功能：自定义模板系统（Custom Template System）、模板继承（Template Inheritance）和项目更新机制（Project Update Mechanism）。这些功能使团队能够标准化项目脚手架、通过继承扩展内置模板、以及在工具升级或功能变更时更新已生成的项目。

This document defines the requirements for `axum-app-create` CLI tool v0.3.0. Building on the completed v0.2.0, v0.3.0 adds three major features: Custom Template System, Template Inheritance, and Project Update Mechanism. These features enable teams to standardize project scaffolding, extend built-in templates through inheritance, and update previously generated projects when the tool is upgraded or features change.

## 术语表 / Glossary

- **CLI_Tool**: `axum-app-create` 命令行工具，用于脚手架生成 Axum Web 应用项目
- **Generator**: 项目生成器模块（`generator::project`），负责编排整个项目生成流程
- **TemplateEngine**: Handlebars 模板引擎（`template::engine`），负责渲染 `.hbs` 模板文件
- **TemplateRegistry**: 模板注册表（`template::templates`），管理和提供嵌入式模板文件集合
- **TemplateResolver**: 模板解析器，负责合并内置模板与自定义模板，处理覆盖和继承逻辑
- **ProjectConfig**: 项目配置结构体（`config::ProjectConfig`），包含项目名称、功能集和子配置
- **TemplateContext**: 模板上下文（`template::context::TemplateContext`），为 Handlebars 提供渲染变量
- **CustomTemplateDir**: 用户提供的自定义模板目录，包含覆盖或扩展内置模板的 `.hbs` 文件
- **TemplateInheritance**: 模板继承机制，允许自定义模板通过 `extends` 指令扩展内置模板
- **Block**: 模板中可被子模板覆盖的命名区域，使用 `{{#block "name"}}` 定义
- **Override**: 子模板中用于替换父模板 Block 内容的指令，使用 `{{#override "name"}}` 定义
- **GenerationMetadata**: 生成元数据文件（`.axum-app-create.json`），记录项目生成时的版本、配置和时间戳
- **UpdateEngine**: 更新引擎，负责比较当前文件与新生成内容，执行差异化更新
- **UserConfig**: 用户级配置文件（`~/.axum-app-create.toml`），存储默认设置如 `template_dir`

## 需求 / Requirements

### 需求 1：自定义模板加载 / Custom Template Loading

**用户故事 / User Story:** 作为一名团队负责人，我希望能够提供自定义模板目录来覆盖或扩展内置模板，以便团队能够标准化项目脚手架。

As a team lead, I want to provide a custom template directory to override or extend built-in templates, so that my team can standardize project scaffolding.

#### 验收标准 / Acceptance Criteria

1. WHEN 用户指定 `--template-dir <PATH>` CLI 标志, THE CLI_Tool SHALL 从指定路径加载自定义模板文件
2. WHEN 自定义模板目录中存在与内置模板相同路径的文件, THE TemplateResolver SHALL 使用自定义模板替换对应的内置模板
3. WHEN 自定义模板目录中存在内置模板集合中不存在的文件, THE TemplateResolver SHALL 将这些文件作为新增模板添加到模板集合中
4. WHEN 自定义模板被加载, THE TemplateEngine SHALL 使用与内置模板相同的 Handlebars 语法和 TemplateContext 变量渲染自定义模板
5. WHEN `--template-dir` 指向不存在的路径, THE CLI_Tool SHALL 输出错误信息并以非零退出码退出
6. WHEN `--template-dir` 指向的目录为空, THE CLI_Tool SHALL 使用全部内置模板继续生成（等同于未指定自定义模板）
7. THE CLI_Tool SHALL 支持 `axum-app-create init-template` 子命令，将内置模板导出到指定目录作为自定义模板的起点
8. WHEN `init-template` 子命令被执行, THE CLI_Tool SHALL 导出当前模式（single 或 workspace）对应的所有内置模板文件
9. THE CLI_Tool SHALL 支持 `~/.axum-app-create.toml` 用户配置文件，允许设置默认的 `template_dir` 路径
10. WHEN 用户配置文件中设置了 `template_dir` 且 CLI 标志也指定了 `--template-dir`, THE CLI_Tool SHALL 使用 CLI 标志的值（CLI 标志优先级高于配置文件）

### 需求 2：模板继承系统 / Template Inheritance System

**用户故事 / User Story:** 作为一名开发者，我希望自定义模板能够继承内置模板并只覆盖特定部分，以便在不复制整个模板的情况下进行定制。

As a developer, I want custom templates to inherit from built-in templates and override only specific sections, so that I can customize without duplicating entire templates.

#### 验收标准 / Acceptance Criteria

1. WHEN 自定义模板包含 `{{!-- extends: <base_template_path> --}}` 指令, THE TemplateEngine SHALL 将该模板视为子模板，加载指定的内置模板作为基础模板
2. WHEN 内置模板使用 `{{#block "name"}}...{{/block}}` 定义命名块, THE TemplateEngine SHALL 将块内容作为默认内容渲染
3. WHEN 子模板使用 `{{#override "name"}}...{{/override}}` 定义覆盖块, THE TemplateEngine SHALL 使用覆盖块的内容替换基础模板中同名块的默认内容
4. WHEN 子模板未覆盖某个块, THE TemplateEngine SHALL 保留基础模板中该块的默认内容
5. WHEN 自定义模板不包含 `extends` 指令, THE TemplateResolver SHALL 将该模板视为完全替换，直接替换对应的内置模板
6. THE TemplateEngine SHALL 在内置模板中为关键区域定义块：`imports`、`config`、`routes`、`middleware`、`main_body`
7. WHEN `extends` 指令引用不存在的基础模板路径, THE TemplateEngine SHALL 输出包含模板名称和路径的错误信息
8. WHEN 子模板中的 `override` 块名称在基础模板中不存在, THE TemplateEngine SHALL 输出警告信息并忽略该覆盖块
9. THE TemplateEngine SHALL 支持在一个子模板中覆盖多个块

### 需求 3：内置模板块重构 / Built-in Template Block Refactoring

**用户故事 / User Story:** 作为一名开发者，我希望内置模板定义清晰的可覆盖块，以便自定义模板能够精确地扩展特定部分。

As a developer, I want built-in templates to define clear overridable blocks, so that custom templates can precisely extend specific sections.

#### 验收标准 / Acceptance Criteria

1. THE TemplateRegistry SHALL 在 `single_mode/src/main.rs.hbs` 模板中定义以下块：`imports`、`config`、`routes`、`middleware`、`main_body`
2. THE TemplateRegistry SHALL 在 `workspace_mode/api/src/main.rs.hbs` 模板中定义相同的块集合
3. THE TemplateRegistry SHALL 在 `Cargo.toml.hbs` 模板中定义 `dependencies` 和 `dev_dependencies` 块
4. WHEN 块被添加到内置模板后, THE Generator SHALL 生成与重构前功能完全一致的项目（向后兼容）
5. THE TemplateEngine SHALL 在没有子模板覆盖时，渲染块的默认内容（即块定义不影响正常渲染）

### 需求 4：项目更新子命令 / Project Update Subcommand

**用户故事 / User Story:** 作为一名开发者，我希望能够更新已生成的项目以获取工具升级带来的改进或更改功能配置，以便项目保持最新状态。

As a developer, I want to update a previously generated project to get tool upgrades or change feature configuration, so that my project stays up to date.

#### 验收标准 / Acceptance Criteria

1. THE CLI_Tool SHALL 支持 `axum-app-create update [PROJECT_DIR]` 子命令，默认 `PROJECT_DIR` 为当前目录
2. WHEN `update` 子命令被执行, THE UpdateEngine SHALL 读取项目中的 `.axum-app-create.json` 元数据文件以获取原始生成配置
3. WHEN 项目目录中不存在 `.axum-app-create.json` 文件, THE CLI_Tool SHALL 输出错误信息提示该项目不是由 axum-app-create 生成的
4. WHEN 更新被执行, THE UpdateEngine SHALL 使用元数据中的配置重新生成所有模板文件，并与当前文件进行逐文件比较
5. WHEN 文件内容与新生成内容相同, THE UpdateEngine SHALL 跳过该文件
6. WHEN 文件内容与新生成内容不同且文件未被用户手动修改, THE UpdateEngine SHALL 自动更新该文件
7. WHEN 文件内容与新生成内容不同且文件已被用户手动修改, THE UpdateEngine SHALL 在交互模式下提示用户选择操作（覆盖、跳过、查看差异）
8. THE CLI_Tool SHALL 支持 `--dry-run` 标志，仅显示将要进行的更改而不实际修改文件
9. THE CLI_Tool SHALL 支持 `--force` 标志，强制覆盖所有文件（包括用户修改过的文件）

### 需求 5：生成元数据 / Generation Metadata

**用户故事 / User Story:** 作为一名开发者，我希望生成的项目包含元数据文件，以便更新机制能够了解项目的原始配置。

As a developer, I want generated projects to include metadata files, so that the update mechanism can understand the original project configuration.

#### 验收标准 / Acceptance Criteria

1. WHEN 项目被生成, THE Generator SHALL 在项目根目录创建 `.axum-app-create.json` 文件
2. THE GenerationMetadata SHALL 包含以下字段：`version`（工具版本）、`generated_at`（ISO 8601 时间戳）、`config`（完整的 ProjectConfig 序列化）、`file_checksums`（每个生成文件的 SHA-256 校验和）
3. WHEN `.axum-app-create.json` 被序列化, THE Generator SHALL 使用 serde_json 格式化输出（pretty print）以便人类可读
4. THE Generator SHALL 将 `.axum-app-create.json` 添加到生成的 `.gitignore` 中（避免提交到版本控制）
5. WHEN 更新操作完成后, THE UpdateEngine SHALL 更新 `.axum-app-create.json` 中的 `version`、`generated_at` 和 `file_checksums` 字段

### 需求 6：用户修改检测 / User Modification Detection

**用户故事 / User Story:** 作为一名开发者，我希望更新机制能够检测我手动修改过的文件，以便避免意外覆盖我的自定义代码。

As a developer, I want the update mechanism to detect files I've manually modified, so that my custom code is not accidentally overwritten.

#### 验收标准 / Acceptance Criteria

1. WHEN UpdateEngine 检查文件是否被用户修改, THE UpdateEngine SHALL 计算当前文件的 SHA-256 校验和并与 `.axum-app-create.json` 中存储的校验和比较
2. WHEN 当前文件的校验和与存储的校验和匹配, THE UpdateEngine SHALL 判定该文件未被用户修改（可安全更新）
3. WHEN 当前文件的校验和与存储的校验和不匹配, THE UpdateEngine SHALL 判定该文件已被用户修改（需要冲突解决）
4. WHEN 文件在元数据中不存在（新增文件）, THE UpdateEngine SHALL 直接创建该文件
5. WHEN 元数据中记录的文件在磁盘上不存在（已删除文件）, THE UpdateEngine SHALL 重新创建该文件并在交互模式下通知用户

### 需求 7：用户配置文件 / User Configuration File

**用户故事 / User Story:** 作为一名开发者，我希望能够在用户级配置文件中设置默认选项，以便不必每次都通过 CLI 标志指定常用设置。

As a developer, I want to set default options in a user-level configuration file, so that I don't have to specify common settings via CLI flags every time.

#### 验收标准 / Acceptance Criteria

1. THE CLI_Tool SHALL 在启动时检查 `~/.axum-app-create.toml` 文件是否存在
2. WHEN 用户配置文件存在, THE CLI_Tool SHALL 解析 TOML 格式的配置并应用默认值
3. THE UserConfig SHALL 支持以下配置项：`template_dir`（默认自定义模板目录路径）
4. WHEN 用户配置文件格式无效, THE CLI_Tool SHALL 输出警告信息并继续使用默认值（不中断执行）
5. THE CLI_Tool SHALL 按以下优先级解析配置（从高到低）：CLI 标志 > 用户配置文件 > 默认值

### 需求 8：CLI 参数扩展 / CLI Arguments Extension

**用户故事 / User Story:** 作为一名开发者，我希望新功能通过清晰的 CLI 参数和子命令暴露，以便我能方便地使用所有新功能。

As a developer, I want new features exposed through clear CLI arguments and subcommands, so that I can conveniently use all new capabilities.

#### 验收标准 / Acceptance Criteria

1. THE CliArgs SHALL 新增 `--template-dir <PATH>` 参数，用于指定自定义模板目录
2. THE CLI_Tool SHALL 新增 `init-template [OUTPUT_DIR]` 子命令，默认输出到 `./templates`
3. THE CLI_Tool SHALL 新增 `update [PROJECT_DIR]` 子命令，默认更新当前目录
4. THE `update` 子命令 SHALL 支持 `--dry-run` 和 `--force` 标志
5. WHEN 使用 `--help` 标志, THE CLI_Tool SHALL 显示所有新增参数和子命令的双语帮助信息（英文 + 中文）
6. THE CLI_Tool SHALL 将版本号更新为 `0.3.0`

### 需求 9：向后兼容性 / Backward Compatibility

**用户故事 / User Story:** 作为一名现有用户，我希望升级到 v0.3.0 后现有的使用方式不受影响，以便平滑过渡到新版本。

As an existing user, I want upgrading to v0.3.0 to not break my existing workflows, so that I can smoothly transition to the new version.

#### 验收标准 / Acceptance Criteria

1. WHEN 未指定 `--template-dir` 且无用户配置文件, THE CLI_Tool SHALL 使用全部内置模板生成项目（与 v0.2.0 行为一致）
2. WHEN 使用 v0.2.0 的 CLI 参数组合, THE CLI_Tool SHALL 生成与 v0.2.0 功能一致的项目
3. THE Generator SHALL 确保所有现有的 70 个测试继续通过
4. WHEN 内置模板被重构添加块定义后, THE Generator SHALL 生成与重构前内容完全一致的输出文件
