# Rust Learn 项目构建配置

## 概述

本目录包含了 `rust-learn` 项目的 GitHub Actions 构建配置，支持多种构建方式和自动化流程。

## 工作流文件

### 1. `rust-learn-build.yml` - 项目综合构建

**功能**: 自动检测和构建所有子项目
**触发方式**:

- 手动触发 (`workflow_dispatch`)
- 推送代码到主分支
- 创建 Pull Request

**支持项目类型**:

- **Rust 项目**: 包含 `Cargo.toml` 的目录
- **Tauri 项目**: 包含 `src-tauri/Cargo.toml` 的目录
- **Node.js 项目**: 包含 `package.json` 的目录

**构建选项**:

- 选择特定项目或构建所有项目
- 选择构建类型 (debug/release)
- 选择目标平台 (仅对支持多平台的子项目有效)
- 是否运行测试
- 自定义构建说明

### 2. `yuque-tools-gui-multi-platform.yml` - Tauri 应用多平台构建

**功能**: 专门用于构建 `yuque-tools-gui` 的多平台应用
**触发方式**:

- 手动触发 (`workflow_dispatch`)
- 推送代码到主分支
- 创建 Pull Request

**支持平台**:

- **macOS**: Intel (x86_64) 和 Apple Silicon (ARM64)
- **Windows**: x86_64 (64 位)
- **Linux**: x86_64 (64 位)

**构建选项**:

- 选择构建类型 (Intel/Apple Silicon/两者)
- 选择目标平台 (macOS/Windows/Linux/全部)
- 是否构建发布版本
- 版本标签和构建说明

### 3. `yuque-tools-release.yml` - 命令行工具发布构建

**功能**: 构建 `yuque-tools` 命令行工具的多平台版本
**触发方式**: 发布 Release 时自动触发

**支持平台**:

- FreeBSD (x86_64)
- Windows (x86_64)
- macOS (Intel + Apple Silicon)

## 使用方法

### 手动触发构建

#### 1. 项目综合构建

1. 进入 GitHub 仓库
2. 点击 **Actions** 标签
3. 选择 **Rust Learn - Multi-Project Build** 工作流
4. 点击 **Run workflow**
5. 配置构建参数:
   - **项目**: 选择要构建的项目
   - **构建类型**: debug 或 release
   - **目标平台**: 选择目标平台
   - **运行测试**: 是否运行测试
   - **构建说明**: 自定义说明

#### 2. Tauri 应用多平台构建

1. 选择 **Yuque Tools GUI - Multi-Platform Build** 工作流
2. 点击 **Run workflow**
3. 配置构建参数:
   - **构建类型**: Intel/Apple Silicon/两者
   - **目标平台**: macOS/Windows/Linux/全部
   - **发布版本**: 是否构建发布版本
   - **版本标签**: 版本号 (例如: v1.0.0)
   - **构建说明**: 自定义说明

### 自动触发构建

#### 推送代码触发

当推送代码到 `main` 或 `develop` 分支时，会自动触发相关构建：

- 修改 `**/Cargo.toml` → 触发 Rust 项目构建
- 修改 `**/package.json` → 触发 Node.js 项目构建
- 修改 `**/src-tauri/**` → 触发 Tauri 项目构建
- 修改 `**/src/**` → 触发相关项目构建

#### Pull Request 触发

创建 Pull Request 时会自动运行构建和测试，确保代码质量。

## 构建产物

### 构建产物位置

构建完成后，可以在以下位置找到构建产物：

1. **Actions 页面**: 具体运行 > Artifacts
2. **Release 页面**: 如果配置了版本标签，会自动创建 Release

### 产物类型

- **Rust 项目**: 可执行文件和依赖库
- **Tauri 项目**: 平台特定的应用包 (.app, .exe, .AppImage 等)
- **Node.js 项目**: 构建后的前端文件

### 产物保留时间

- 普通构建产物: 7 天
- Tauri 应用构建产物: 30 天
- 发布包: 90 天

## 环境配置

### 支持的操作系统

- **Ubuntu Latest**: Linux 构建环境
- **Windows Latest**: Windows 构建环境
- **macOS Latest**: macOS 构建环境

### 工具链版本

- **Rust**: 1.75
- **Node.js**: 18.x
- **Tauri CLI**: 最新版本

### 系统依赖

Linux 环境会自动安装必要的系统依赖：

```bash
libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev
librsvg2-dev libssl-dev libssl3
```

## 最佳实践

### 1. 构建策略

- **开发阶段**: 使用 debug 构建，快速迭代
- **测试阶段**: 运行测试确保代码质量
- **发布阶段**: 使用 release 构建，优化性能

### 2. 平台选择

- **开发测试**: 选择当前开发平台
- **完整测试**: 选择所有平台进行全面测试
- **发布部署**: 根据目标用户选择相应平台

### 3. 版本管理

- 使用语义化版本号 (例如: v1.0.0)
- 在发布前更新版本号
- 提供详细的构建说明和更新日志

### 4. 错误处理

- 检查构建日志获取详细错误信息
- 确保所有依赖正确安装
- 验证目标平台支持

## 故障排除

### 常见问题

#### 1. 构建失败

**检查项**:

- 代码语法错误
- 依赖版本冲突
- 目标平台不支持
- 系统依赖缺失

**解决方案**:

- 查看构建日志
- 本地测试构建
- 更新依赖版本
- 安装缺失的系统依赖

#### 2. 测试失败

**检查项**:

- 测试代码逻辑错误
- 测试环境配置问题
- 依赖服务不可用

**解决方案**:

- 本地运行测试
- 检查测试配置
- 修复测试代码

#### 3. 产物上传失败

**检查项**:

- 构建产物路径错误
- 文件权限问题
- 网络连接问题

**解决方案**:

- 验证产物路径
- 检查文件权限
- 重试上传操作

## 扩展配置

### 添加新项目

1. 在项目根目录创建新的子项目
2. 确保包含必要的配置文件 (`Cargo.toml`, `package.json` 等)
3. 工作流会自动检测新项目

### 自定义构建步骤

1. 修改对应的工作流文件
2. 添加自定义构建步骤
3. 配置特定的构建参数

### 集成第三方服务

1. 添加相应的 Action
2. 配置服务参数
3. 设置必要的密钥和权限
