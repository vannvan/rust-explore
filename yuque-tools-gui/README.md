# Yuque Tools GUI

基于 Tauri + React + Tailwind CSS 构建的语雀工具桌面应用。

## 技术栈

- **后端**: Tauri 2.0 (Rust)
- **前端**: React 18 + TypeScript
- **样式**: Tailwind CSS
- **构建工具**: Vite

## 开发环境要求

- Node.js 18+
- Rust 1.70+
- Cargo

## 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Rust 依赖（在 src-tauri 目录下）
cd src-tauri
cargo build
```

## 开发模式

```bash
# 启动前端开发服务器
npm run dev

# 启动 Tauri 应用（新终端）
npm run tauri dev
```

## 构建应用

```bash
# 构建前端
npm run build

# 构建 Tauri 应用
npm run tauri build
```

## 项目结构

```
yuque-tools-gui/
├── src/                    # React 前端源码
├── src-tauri/             # Tauri 后端源码
│   ├── src/               # Rust 源码
│   ├── Cargo.toml         # Rust 依赖配置
│   ├── tauri.conf.json    # Tauri 应用配置
│   └── build.rs           # 构建脚本
├── package.json           # 前端依赖配置
├── tailwind.config.js     # Tailwind CSS 配置
└── vite.config.ts         # Vite 配置
```

## 功能特性

![](https://p.ipic.vip/kcyyys.png)

## 许可证

MIT
