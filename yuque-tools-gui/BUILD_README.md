# 多平台构建配置说明

## 概述

本项目配置了完整的多平台构建系统，支持构建适用于以下平台的应用程序：

- **macOS**: Intel (x86_64) 和 Apple Silicon (ARM64)
- **Windows**: x86_64 (64 位)
- **Linux**: x86_64 (64 位)

## 构建方式

### 1. 使用 npm 脚本 (推荐)

#### 基本构建命令

```bash
# 构建当前平台应用
npm run build:tauri

# 构建 macOS 应用 (Intel + Apple Silicon)
npm run build:mac

# 构建 Windows 应用
npm run build:windows

# 构建 Linux 应用
npm run build:linux

# 构建所有平台应用
npm run build:all
```

#### 发布版本构建

```bash
# 构建所有平台发布版本
npm run build:release:all

# 构建 macOS 发布版本
npm run build:release:mac

# 构建 Windows 发布版本
npm run build:release:windows
```

#### 清理和准备

```bash
# 清理构建目录
npm run dist:clean

# 准备发布版本
npm run dist:prepare
```

### 2. 使用 Makefile

#### 安装依赖

```bash
# 安装构建依赖
make install-deps

# 检查依赖
make check-deps
```

#### 构建命令

```bash
# 显示帮助信息
make help

# 构建 macOS 应用
make build-mac

# 构建 Windows 应用
make build-windows

# 构建 Linux 应用
make build-linux

# 构建所有平台应用
make build-all

# 构建发布版本
make build-release-all
```

#### 其他命令

```bash
# 清理构建目录
make clean

# 测试构建
make test

# 显示构建产物位置
make show-output

# 准备发布版本
make release-prepare
```

### 3. 使用构建脚本

#### Linux/macOS

```bash
# 给脚本添加执行权限
chmod +x scripts/build.sh

# 构建 macOS 应用
./scripts/build.sh mac

# 构建 Windows 应用
./scripts/build.sh windows

# 构建所有平台应用
./scripts/build.sh all

# 构建发布版本
./scripts/build.sh release:all

# 清理构建目录
./scripts/build.sh clean

# 显示帮助信息
./scripts/build.sh help
```

#### Windows

```cmd
# 构建 macOS 应用
scripts\build.bat mac

# 构建 Windows 应用
scripts\build.bat windows

# 构建所有平台应用
scripts\build.bat all

# 构建发布版本
scripts\build.bat release:all

# 清理构建目录
scripts\build.bat clean

# 显示帮助信息
scripts\build.bat help
```

### 4. 使用 Docker

#### 构建所有平台

```bash
# 构建所有平台应用
docker-compose up builder

# 仅构建 macOS 应用
docker-compose up builder-macos

# 仅构建 Windows 应用
docker-compose up builder-windows

# 仅构建 Linux 应用
docker-compose up builder-linux
```

#### 开发环境

```bash
# 启动开发环境
docker-compose up dev
```

### 5. 使用 GitHub Actions

GitHub Actions 会自动在以下情况下触发构建：

- 推送到 `main` 或 `develop` 分支
- 创建 Pull Request
- 发布 Release

构建产物会自动上传为 Artifacts。

## 构建产物

### 文件位置

构建完成后，应用文件位于以下目录：

```
src-tauri/target/
├── x86_64-apple-darwin/release/bundle/     # macOS Intel
├── aarch64-apple-darwin/release/bundle/    # macOS Apple Silicon
├── x86_64-pc-windows-msvc/release/bundle/ # Windows
└── x86_64-unknown-linux-gnu/release/bundle/ # Linux
```

### 文件类型

- **macOS**: `.app` 包和 `.dmg` 安装包
- **Windows**: `.exe` 安装程序
- **Linux**: `.AppImage`、`.deb` 和 `.rpm` 包

## 系统要求

### 开发环境

- **Node.js**: 18.x 或更高版本
- **Rust**: 1.75 或更高版本
- **Tauri CLI**: 最新版本
- **操作系统**: macOS、Windows 或 Linux

### 目标平台

- **macOS**: 10.13 或更高版本
- **Windows**: Windows 10 或更高版本
- **Linux**: Ubuntu 18.04 或更高版本

## 交叉编译

### macOS 到其他平台

```bash
# 在 macOS 上构建 Windows 应用
rustup target add x86_64-pc-windows-msvc
npm run build:windows

# 在 macOS 上构建 Linux 应用
rustup target add x86_64-unknown-linux-gnu
npm run build:linux
```

### Linux 到其他平台

```bash
# 在 Linux 上构建 Windows 应用
rustup target add x86_64-pc-windows-msvc
npm run build:windows

# 在 Linux 上构建 macOS 应用
rustup target add x86_64-apple-darwin aarch64-apple-darwin
npm run build:mac
```

### Windows 到其他平台

```bash
# 在 Windows 上构建 Linux 应用
rustup target add x86_64-unknown-linux-gnu
npm run build:linux

# 在 Windows 上构建 macOS 应用
rustup target add x86_64-apple-darwin aarch64-apple-darwin
npm run build:mac
```

## 常见问题

### 1. 构建失败

**问题**: 构建过程中出现错误

**解决方案**:

```bash
# 清理构建目录
make clean

# 重新安装依赖
npm ci

# 重新构建
make build
```

### 2. 交叉编译失败

**问题**: 无法构建其他平台的应用

**解决方案**:

```bash
# 安装目标平台工具链
rustup target add <target-triple>

# 检查目标平台支持
rustup target list --installed
```

### 3. 依赖问题

**问题**: 缺少系统依赖

**解决方案**:

```bash
# 在 Ubuntu/Debian 上
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev

# 在 macOS 上
brew install pkg-config

# 在 Windows 上
# 安装 Visual Studio Build Tools
```

### 4. 权限问题

**问题**: 脚本无法执行

**解决方案**:

```bash
# 给脚本添加执行权限
chmod +x scripts/build.sh
chmod +x scripts/build.bat
```

## 最佳实践

### 1. 构建顺序

1. 先构建前端应用
2. 再构建 Tauri 应用
3. 最后打包分发

### 2. 版本管理

- 使用语义化版本号
- 在发布前更新版本号
- 使用 Git 标签管理发布版本

### 3. 测试

- 在目标平台上测试应用
- 使用虚拟机测试不同平台
- 进行自动化测试

### 4. 分发

- 使用 GitHub Releases 分发
- 提供安装说明
- 包含更新日志

## 配置说明

### Tauri 配置

主要配置位于 `src-tauri/tauri.conf.json`：

```json
{
  "bundle": {
    "active": true,
    "targets": "all",
    "identifier": "com.yuque-tools.gui"
  }
}
```

### 构建脚本配置

构建脚本支持以下环境变量：

- `NODE_ENV`: 构建环境 (development/production)
- `CARGO_TERM_COLOR`: Rust 构建输出颜色

## 总结

本构建系统提供了多种方式来构建多平台应用：

✅ **npm 脚本**: 简单易用，适合日常开发  
✅ **Makefile**: 功能完整，适合自动化构建  
✅ **构建脚本**: 跨平台支持，适合 CI/CD  
✅ **Docker**: 环境隔离，适合团队协作  
✅ **GitHub Actions**: 自动化构建，适合开源项目

选择适合你需求的构建方式，开始构建多平台应用吧！
