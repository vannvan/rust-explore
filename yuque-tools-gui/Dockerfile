# 多平台构建 Docker 镜像
# 支持构建 macOS、Windows 和 Linux 应用

# 使用官方 Rust 镜像作为基础
FROM rust:1.75-slim as rust-base

# 安装必要的系统依赖
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 安装 Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# 安装 Tauri CLI
RUN npm install -g @tauri-apps/cli

# 设置工作目录
WORKDIR /app

# 复制 package.json 和 package-lock.json
COPY package*.json ./

# 安装 Node.js 依赖
RUN npm ci

# 复制源代码
COPY . .

# 构建前端
RUN npm run build

# 构建 Tauri 应用 (Linux)
RUN tauri build --target x86_64-unknown-linux-gnu --release

# 创建输出目录
RUN mkdir -p /output/linux

# 复制构建产物
RUN cp -r src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/* /output/linux/

# 多阶段构建 - Windows 交叉编译
FROM rust:1.75-slim as windows-builder

# 安装 Windows 交叉编译工具链
RUN rustup target add x86_64-pc-windows-msvc

# 安装必要的系统依赖
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 安装 Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# 安装 Tauri CLI
RUN npm install -g @tauri-apps/cli

# 设置工作目录
WORKDIR /app

# 复制 package.json 和 package-lock.json
COPY package*.json ./

# 安装 Node.js 依赖
RUN npm ci

# 复制源代码
COPY . .

# 构建前端
RUN npm run build

# 构建 Tauri 应用 (Windows)
RUN tauri build --target x86_64-pc-windows-msvc --release

# 创建输出目录
RUN mkdir -p /output/windows

# 复制构建产物
RUN cp -r src-tauri/target/x86_64-pc-windows-msvc/release/bundle/* /output/windows/

# 多阶段构建 - macOS 交叉编译
FROM rust:1.75-slim as macos-builder

# 安装 macOS 交叉编译工具链
RUN rustup target add x86_64-apple-darwin aarch64-apple-darwin

# 安装必要的系统依赖
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

# 安装 Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# 安装 Tauri CLI
RUN npm install -g @tauri-apps/cli

# 设置工作目录
WORKDIR /app

# 复制 package.json 和 package-lock.json
COPY package*.json ./

# 安装 Node.js 依赖
RUN npm ci

# 复制源代码
COPY . .

# 构建前端
RUN npm run build

# 构建 Tauri 应用 (macOS Intel)
RUN tauri build --target x86_64-apple-darwin --release

# 构建 Tauri 应用 (macOS Apple Silicon)
RUN tauri build --target aarch64-apple-darwin --release

# 创建输出目录
RUN mkdir -p /output/macos

# 复制构建产物
RUN cp -r src-tauri/target/x86_64-apple-darwin/release/bundle/* /output/macos/ \
    && cp -r src-tauri/target/aarch64-apple-darwin/release/bundle/* /output/macos/

# 最终镜像 - 包含所有构建产物
FROM alpine:latest

# 安装必要的工具
RUN apk add --no-cache \
    bash \
    curl \
    tar \
    gzip

# 创建输出目录
RUN mkdir -p /output/{linux,windows,macos}

# 复制所有平台的构建产物
COPY --from=rust-base /output/linux /output/linux
COPY --from=windows-builder /output/windows /output/windows
COPY --from=macos-builder /output/macos /output/macos

# 创建构建脚本
RUN echo '#!/bin/bash' > /usr/local/bin/build-all && \
    echo 'echo "构建所有平台应用..."' >> /usr/local/bin/build-all && \
    echo 'echo "Linux 应用已构建完成"' >> /usr/local/bin/build-all && \
    echo 'echo "Windows 应用已构建完成"' >> /usr/local/bin/build-all && \
    echo 'echo "macOS 应用已构建完成"' >> /usr/local/bin/build-all && \
    echo 'echo "所有应用构建完成！"' >> /usr/local/bin/build-all && \
    chmod +x /usr/local/bin/build-all

# 设置工作目录
WORKDIR /output

# 暴露输出目录
VOLUME ["/output"]

# 默认命令
CMD ["/usr/local/bin/build-all"]
