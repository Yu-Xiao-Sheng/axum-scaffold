# Contributing to axum-app-create

> 贡献指南 / Guidelines for contributing to axum-app-create

Thank you for your interest in contributing to `axum-app-create`! This document provides guidelines and instructions for contributing.
感谢您对 `axum-app-create` 的贡献兴趣！本文档提供了贡献的指南和说明。

---

## 📋 Table of Contents / 目录

- [Code of Conduct / 行为准则](#code-of-conduct-行为准则)
- [Getting Started / 入门指南](#getting-started-入门指南)
- [Development Setup / 开发环境设置](#development-setup-开发环境设置)
- [Code Style / 代码风格](#code-style-代码风格)
- [Submitting Changes / 提交更改](#submitting-changes-提交更改)
- [Testing Guidelines / 测试指南](#testing-guidelines-测试指南)
- [Documentation Standards / 文档标准](#documentation-standards-文档标准)
- [Pull Request Process / PR 流程](#pull-request-process-pr-流程)

---

## 🤝 Code of Conduct / 行为准则

- Be respectful and inclusive / 保持尊重和包容
- Provide constructive feedback / 提供建设性反馈
- Focus on what is best for the community / 以社区利益为重
- Show empathy towards other community members / 对其他社区成员表示同理心

---

## 🚀 Getting Started / 入门指南

### Prerequisites / 前置要求

- Rust toolchain 1.75+ / Rust 工具链 1.75+
- Git / 版本控制
- Basic knowledge of Axum web framework / Axum Web 框架基础知识

### Installation / 安装

```bash
# Fork the repository / Fork 仓库
git clone https://github.com/YOUR-USERNAME/axum-app-create.git
cd axum-app-create

# Install dependencies / 安装依赖
cargo build

# Run tests / 运行测试
cargo test
```

---

## 🛠️ Development Setup / 开发环境设置

### Recommended Tools / 推荐工具

- **Editor / 编辑器**: VS Code, IntelliJ IDEA, or Neovim
  - rust-analyzer extension for Rust support / rust-analyzer 扩展以支持 Rust
  - CodeLLDB for debugging / CodeLLDB 用于调试

- **CLI Tools / 命令行工具**:
  ```bash
  # Install useful development tools / 安装有用的开发工具
  cargo install cargo-watch      # Watch for changes and re-run tests
  cargo install cargo-edit       # Manage dependencies
  cargo install cargo-audit      # Security audit
  ```

### Project Structure / 项目结构

```
axum-app-create/
├── src/
│   ├── cli/           # Command-line interface / 命令行界面
│   ├── config/        # Configuration structures / 配置结构
│   ├── error.rs       # Error types / 错误类型
│   ├── generator/     # Project generation logic / 项目生成逻辑
│   ├── template/      # Template engine & templates / 模板引擎和模板
│   └── utils/         # Utilities (validation, toolchain) / 工具函数
├── tests/             # Integration tests / 集成测试
└── specs/             # Feature specifications / 功能规范
```

---

## 📐 Code Style / 代码风格

### Rust Conventions / Rust 规范

- **Edition / 版本**: Rust 2024 Edition
- **Formatting / 格式化**: Use `cargo fmt` (100% consistent / 100% 一致)
- **Linting / 代码检查**: Pass `cargo clippy -- -D warnings`
- **Documentation / 文档**: Public APIs must have rustdoc comments / 公共 API 必须有 rustdoc 注释

### Naming Conventions / 命名规范

- **Modules / 模块**: `snake_case` (e.g., `template_engine`)
- **Types / 类型**: `PascalCase` (e.g., `TemplateContext`)
- **Functions / 函数**: `snake_case` (e.g., `render_template`)
- **Constants / 常量**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_LENGTH`)

### Code Organization / 代码组织

```rust
// 1. Module documentation / 模块文档
//! Brief description / 简短描述
//!
//! Detailed explanation / 详细说明

// 2. Imports (grouped and sorted) / 导入（分组并排序）
use std::path::Path;

use crate::error::Result;
use crate::template::context;

// 3. Types / 类型
pub struct MyStruct { ... }

// 4. Implementation / 实现
impl MyStruct {
    pub fn new() -> Self { ... }
}

// 5. Tests / 测试
#[cfg(test)]
mod tests {
    use super::*;
}
```

### Error Handling / 错误处理

- Use `thiserror` for error types / 使用 `thiserror` 定义错误类型
- Use `anyhow` for error context in main / 在 main 中使用 `anyhow` 提供错误上下文
- Provide helpful error messages / 提供有用的错误消息
- Include bilingual messages (en/zh) when user-facing / 面向用户时包含双语消息

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to read file: {path} / 读取文件失败: {path}")]
    FileReadError { path: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## ✅ Testing Guidelines / 测试指南

### Test Coverage / 测试覆盖

- **Unit tests**: Test individual functions / 测试单个函数
- **Integration tests**: Test workflows in `tests/` / 在 `tests/` 中测试工作流
- **Template tests**: Verify rendered output / 验证渲染输出

### Writing Tests / 编写测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange / 准备
        let input = "test";

        // Act / 执行
        let result = function_under_test(input);

        // Assert / 断言
        assert_eq!(result, "expected");
    }
}
```

### Running Tests / 运行测试

```bash
# Run all tests / 运行所有测试
cargo test

# Run tests with output / 运行测试并显示输出
cargo test -- --nocapture

# Run specific test / 运行特定测试
cargo test test_function_name

# Run tests in watch mode / 以监视模式运行测试
cargo watch -x test
```

### Test Requirements / 测试要求

- All new features must include tests / 所有新功能必须包含测试
- Maintain 100% test pass rate / 保持 100% 测试通过率
- Add tests for bug fixes / 为错误修复添加测试
- Test edge cases / 测试边界情况

---

## 📚 Documentation Standards / 文档标准

### rustdoc Comments / rustdoc 注释

All public APIs must have documentation:
所有公共 API 必须有文档：

```rust
/// Brief description (one sentence) / 简短描述（一句话）
///
/// Detailed explanation if needed / 如需详细说明
///
/// # Examples / 示例
///
/// ```
/// use axum_app_create::function_name;
///
/// let result = function_name("input");
/// assert_eq!(result, "expected");
/// ```
///
/// # Errors / 错误
///
/// - `InvalidInput` - if input is malformed / 如果输入格式错误
///
/// # Panics / 恐慌
///
/// Never panics / 从不恐慌
pub fn function_name(input: &str) -> String {
    // ...
}
```

### Bilingual Documentation / 双语文档

For user-facing messages, provide both English and Chinese:
面向用户的消息，提供英文和中文：

```rust
#[error("Invalid project name '{0}': {1}")]
/// Invalid project name error / 项目名称无效错误
///
/// # Arguments / 参数
///
/// * `name` - The invalid project name / 无效的项目名称
/// * `reason` - Why it's invalid / 无效的原因
InvalidName(String, String),
```

### README Updates / README 更新

When adding features, update:
添加功能时，更新：
- Feature list in README.md / README.md 中的功能列表
- Examples section / 示例部分
- CHANGELOG.md / CHANGELOG.md

---

## 🔄 Submitting Changes / 提交更改

### Git Workflow / Git 工作流

```bash
# 1. Create a feature branch / 创建功能分支
git checkout -b feature/amazing-feature

# 2. Make your changes / 进行更改
# ... write code ... / 编写代码

# 3. Commit your changes / 提交更改
git add .
git commit -m "feat: add amazing feature"

# 4. Run tests and lints / 运行测试和检查
cargo test
cargo fmt
cargo clippy -- -D warnings

# 5. Push to your fork / 推送到您的 fork
git push origin feature/amazing-feature
```

### Commit Message Format / 提交消息格式

Follow conventional commits:
遵循约定式提交：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types / 类型**:
- `feat`: New feature / 新功能
- `fix`: Bug fix / 错误修复
- `docs`: Documentation only / 仅文档
- `style`: Code style (formatting, etc.) / 代码风格（格式化等）
- `refactor`: Code refactoring / 代码重构
- `test`: Adding or updating tests / 添加或更新测试
- `chore`: Maintenance tasks / 维护任务

**Examples / 示例**:

```
feat(template): add database migration template

Add support for automatic database migrations using sqlx-cli.
Includes template for migrations/ directory and CLI commands.

Closes #123
```

### Before Submitting / 提交前

- [ ] Run `cargo test` - all tests pass / 运行 `cargo test` - 所有测试通过
- [ ] Run `cargo fmt` - code formatted / 运行 `cargo fmt` - 代码已格式化
- [ ] Run `cargo clippy -- -D warnings` - no warnings / 运行 `cargo clippy -- -D warnings` - 无警告
- [ ] Add tests for new code / 为新代码添加测试
- [ ] Update documentation / 更新文档
- [ ] Update CHANGELOG.md / 更新 CHANGELOG.md

---

## 🔍 Pull Request Process / PR 流程

### PR Checklist / PR 检查清单

When submitting a PR, ensure:
提交 PR 时，确保：

- [ ] Descriptive title and description / 清晰的标题和描述
- [ ] Linked to relevant issue / 链接到相关 issue
- [ ] All tests passing / 所有测试通过
- [ ] Code formatted with `cargo fmt` / 代码已用 `cargo fmt` 格式化
- [ ] No clippy warnings / 无 clippy 警告
- [ ] Documentation updated / 文档已更新
- [ ] CHANGELOG.md updated / CHANGELOG.md 已更新

### PR Title Format / PR 标题格式

```
type(scope): description
```

Examples / 示例:
- `feat(cli): add non-interactive mode`
- `fix(template): resolve project_snake_case variable error`
- `docs(readme): update installation instructions`

### PR Description Template / PR 描述模板

```markdown
## Summary / 概述
Brief description of changes / 更改的简要描述

## Changes Made / 所做更改
- Change 1 / 更改 1
- Change 2 / 更改 2

## Testing / 测试
- [x] Unit tests pass / 单元测试通过
- [x] Integration tests pass / 集成测试通过
- [x] Manual testing completed / 手动测试完成

## Related Issues / 相关问题
Closes #123 / 关闭 #123

## Screenshots (if applicable) / 截图（如适用）
Add screenshots for UI changes / 为 UI 更改添加截图
```

### Review Process / 审查流程

1. **Automated Checks / 自动检查**:
   - CI pipeline runs tests / CI 流水线运行测试
   - Clippy and formatting checks / Clippy 和格式化检查

2. **Code Review / 代码审查**:
   - Maintainer reviews your code / 维护者审查您的代码
   - Address feedback / 处理反馈

3. **Approval and Merge / 批准和合并**:
   - At least one approval required / 至少需要一次批准
   - Squash and merge to main / 压缩并合并到主分支

---

## 🐛 Reporting Bugs / 报告错误

### Bug Report Template / 错误报告模板

```markdown
## Description / 描述
Clear description of the bug / 错误的清晰描述

## Steps to Reproduce / 复现步骤
1. Step 1 / 步骤 1
2. Step 2 / 步骤 2
3. ... / ...

## Expected Behavior / 预期行为
What should happen / 应该发生什么

## Actual Behavior / 实际行为
What actually happens / 实际发生了什么

## Environment / 环境
- OS: [e.g., Ubuntu 22.04] / 操作系统
- Rust version: [e.g., 1.75.0] / Rust 版本
- axum-app-create version: [e.g., 0.1.0] / axum-app-create 版本

## Additional Context / 附加上下文
Logs, screenshots, or other relevant information / 日志、截图或其他相关信息
```

---

## 💡 Feature Requests / 功能请求

### Feature Request Template / 功能请求模板

```markdown
## Summary / 概述
Brief description of the feature / 功能的简要描述

## Motivation / 动机
Why is this feature needed? / 为什么需要这个功能？
What problem does it solve? / 它解决了什么问题？

## Proposed Solution / 建议的解决方案
Detailed description of the proposed solution / 建议解决方案的详细描述

## Alternatives Considered / 考虑的替代方案
What other approaches did you consider? / 您考虑了哪些其他方法？

## Additional Context / 附加上下文
Examples, mockups, or references / 示例、模型或参考
```

---

## ❓ Getting Help / 获取帮助

- **GitHub Issues**: https://github.com/Yu-Xiao-Sheng/axum-app-create/issues
- **Discussions**: https://github.com/Yu-Xiao-Sheng/axum-app-create/discussions
- **Documentation**: https://github.com/Yu-Xiao-Sheng/axum-app-create/blob/main/README.md

---

## 📜 License / 许可证

By contributing, you agree that your contributions will be licensed under the MIT or Apache-2.0 license.
通过贡献，您同意您的贡献将根据 MIT 或 Apache-2.0 许可证进行许可。

---

**Thank you for contributing! 🎉**
**感谢您的贡献！🎉**
