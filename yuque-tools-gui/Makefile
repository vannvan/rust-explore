# 多平台构建 Makefile
# 支持 macOS (Intel/Apple Silicon)、Windows 和 Linux

.PHONY: help clean build build-mac build-windows build-linux build-all
.PHONY: build-release build-release-mac build-release-windows build-release-all
.PHONY: docker-build docker-build-mac docker-build-windows docker-build-linux docker-build-all
.PHONY: install-deps check-deps

# 默认目标
.DEFAULT_GOAL := help

# 颜色定义
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[1;33m
BLUE := \033[0;34m
NC := \033[0m # No Color

# 打印带颜色的消息
define print_info
	@echo "$(BLUE)[INFO]$(NC) $(1)"
endef

define print_success
	@echo "$(GREEN)[SUCCESS]$(NC) $(1)"
endef

define print_warning
	@echo "$(YELLOW)[WARNING]$(NC) $(1)"
endef

define print_error
	@echo "$(RED)[ERROR]$(NC) $(1)"
endef

# 帮助信息
help: ## 显示帮助信息
	@echo "多平台构建 Makefile"
	@echo ""
	@echo "用法: make [目标]"
	@echo ""
	@echo "目标:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'
	@echo ""
	@echo "示例:"
	@echo "  make build-mac        # 构建 macOS 应用"
	@echo "  make build-release-all # 构建所有平台发布版本"
	@echo "  make clean            # 清理构建目录"

# 安装依赖
install-deps: ## 安装构建依赖
	$(call print_info,安装构建依赖...)
	@npm install -g @tauri-apps/cli
	@npm ci
	$(call print_success,依赖安装完成)

# 检查依赖
check-deps: ## 检查构建依赖
	$(call print_info,检查构建依赖...)
	@which npm > /dev/null || (echo "$(RED)错误: npm 未安装$(NC)" && exit 1)
	@which tauri > /dev/null || (echo "$(RED)错误: Tauri CLI 未安装$(NC)" && exit 1)
	@which node > /dev/null || (echo "$(RED)错误: Node.js 未安装$(NC)" && exit 1)
	@which cargo > /dev/null || (echo "$(RED)错误: Rust 未安装$(NC)" && exit 1)
	$(call print_success,依赖检查完成)

# 清理构建目录
clean: ## 清理构建目录
	$(call print_info,清理构建目录...)
	@rm -rf src-tauri/target src-tauri/dist dist
	$(call print_success,清理完成)

# 构建前端
build-frontend: ## 构建前端应用
	$(call print_info,构建前端应用...)
	@npm run build
	$(call print_success,前端构建完成)

# 构建 macOS 应用
build-mac: check-deps clean build-frontend ## 构建 macOS 应用 (Intel + Apple Silicon)
	$(call print_info,构建 macOS 应用...)
	@npm run build:mac
	$(call print_success,macOS 应用构建完成)

# 构建 Windows 应用
build-windows: check-deps clean build-frontend ## 构建 Windows 应用
	$(call print_info,构建 Windows 应用...)
	@npm run build:windows
	$(call print_success,Windows 应用构建完成)

# 构建 Linux 应用
build-linux: check-deps clean build-frontend ## 构建 Linux 应用
	$(call print_info,构建 Linux 应用...)
	@npm run build:linux
	$(call print_success,Linux 应用构建完成)

# 构建所有平台应用
build-all: check-deps clean build-frontend ## 构建所有平台应用
	$(call print_info,构建所有平台应用...)
	@npm run build:all
	$(call print_success,所有平台应用构建完成)

# 构建发布版本
build-release: check-deps clean build-frontend ## 构建发布版本
	$(call print_info,构建发布版本...)
	@npm run build:release
	$(call print_success,发布版本构建完成)

# 构建 macOS 发布版本
build-release-mac: check-deps clean build-frontend ## 构建 macOS 发布版本
	$(call print_info,构建 macOS 发布版本...)
	@npm run build:release:mac
	$(call print_success,macOS 发布版本构建完成)

# 构建 Windows 发布版本
build-release-windows: check-deps clean build-frontend ## 构建 Windows 发布版本
	$(call print_info,构建 Windows 发布版本...)
	@npm run build:release:windows
	$(call print_success,Windows 发布版本构建完成)

# 构建所有平台发布版本
build-release-all: check-deps clean build-frontend ## 构建所有平台发布版本
	$(call print_info,构建所有平台发布版本...)
	@npm run build:release:all
	$(call print_success,所有平台发布版本构建完成)

# Docker 构建
docker-build: ## 使用 Docker 构建所有平台应用
	$(call print_info,使用 Docker 构建所有平台应用...)
	@docker-compose up builder
	$(call print_success,Docker 构建完成)

# Docker 构建 macOS
docker-build-mac: ## 使用 Docker 构建 macOS 应用
	$(call print_info,使用 Docker 构建 macOS 应用...)
	@docker-compose up builder-macos
	$(call print_success,macOS Docker 构建完成)

# Docker 构建 Windows
docker-build-windows: ## 使用 Docker 构建 Windows 应用
	$(call print_info,使用 Docker 构建 Windows 应用...)
	@docker-compose up builder-windows
	$(call print_success,Windows Docker 构建完成)

# Docker 构建 Linux
docker-build-linux: ## 使用 Docker 构建 Linux 应用
	$(call print_info,使用 Docker 构建 Linux 应用...)
	@docker-compose up builder-linux
	$(call print_success,Linux Docker 构建完成)

# 开发环境
dev: ## 启动开发环境
	$(call print_info,启动开发环境...)
	@npm run dev

# 测试构建
test: ## 测试构建
	$(call print_info,测试构建...)
	@npm run build
	@npm run build:tauri
	$(call print_success,测试构建完成)

# 显示构建产物
show-output: ## 显示构建产物
	$(call print_info,构建产物位置:)
	@echo "  macOS:   src-tauri/target/*/release/bundle/"
	@echo "  Windows: src-tauri/target/x86_64-pc-windows-msvc/release/bundle/"
	@echo "  Linux:   src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/"

# 快速构建 (仅当前平台)
build: check-deps clean build-frontend ## 快速构建当前平台应用
	$(call print_info,快速构建当前平台应用...)
	@npm run build:tauri
	$(call print_success,快速构建完成)

# 发布准备
release-prepare: clean build-release-all ## 准备发布版本
	$(call print_info,准备发布版本...)
	@mkdir -p release
	@cp -r src-tauri/target/*/release/bundle/* release/ 2>/dev/null || true
	$(call print_success,发布版本准备完成)
	@echo "发布文件位于 release/ 目录"
