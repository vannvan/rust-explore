#!/bin/bash

# 多平台构建脚本
# 支持 macOS (Intel/Apple Silicon) 和 Windows

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查依赖
check_dependencies() {
    print_info "检查构建依赖..."
    
    if ! command -v npm &> /dev/null; then
        print_error "npm 未安装"
        exit 1
    fi
    
    if ! command -v tauri &> /dev/null; then
        print_error "Tauri CLI 未安装，请运行: npm install -g @tauri-apps/cli"
        exit 1
    fi
    
    print_success "依赖检查完成"
}

# 清理构建目录
clean_build() {
    print_info "清理构建目录..."
    npm run dist:clean
    print_success "清理完成"
}

# 构建前端
build_frontend() {
    print_info "构建前端应用..."
    npm run build
    print_success "前端构建完成"
}

# 构建 macOS 应用
build_macos() {
    print_info "构建 macOS 应用..."
    
    # 构建 Intel 版本
    print_info "构建 Intel 版本..."
    npm run build:mac-intel
    
    # 构建 Apple Silicon 版本
    print_info "构建 Apple Silicon 版本..."
    npm run build:mac-silicon
    
    print_success "macOS 应用构建完成"
}

# 构建 Windows 应用
build_windows() {
    print_info "构建 Windows 应用..."
    npm run build:windows
    print_success "Windows 应用构建完成"
}

# 构建 Linux 应用
build_linux() {
    print_info "构建 Linux 应用..."
    npm run build:linux
    print_success "Linux 应用构建完成"
}

# 构建所有平台
build_all() {
    print_info "构建所有平台应用..."
    npm run build:all
    print_success "所有平台应用构建完成"
}

# 构建发布版本
build_release() {
    print_info "构建发布版本..."
    
    case "$1" in
        "mac")
            npm run build:release:mac
            ;;
        "windows")
            npm run build:release:windows
            ;;
        "all")
            npm run build:release:all
            ;;
        *)
            print_error "未知的发布目标: $1"
            print_info "支持的选项: mac, windows, all"
            exit 1
            ;;
    esac
    
    print_success "发布版本构建完成"
}

# 显示帮助信息
show_help() {
    echo "多平台构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  mac             构建 macOS 应用 (Intel + Apple Silicon)"
    echo "  windows         构建 Windows 应用"
    echo "  linux           构建 Linux 应用"
    echo "  all             构建所有平台应用"
    echo "  release:mac     构建 macOS 发布版本"
    echo "  release:windows 构建 Windows 发布版本"
    echo "  release:all     构建所有平台发布版本"
    echo "  clean           清理构建目录"
    echo "  help            显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0 mac           # 构建 macOS 应用"
    echo "  $0 release:all   # 构建所有平台发布版本"
    echo "  $0 clean         # 清理构建目录"
}

# 主函数
main() {
    case "$1" in
        "mac")
            check_dependencies
            clean_build
            build_frontend
            build_macos
            ;;
        "windows")
            check_dependencies
            clean_build
            build_frontend
            build_windows
            ;;
        "linux")
            check_dependencies
            clean_build
            build_frontend
            build_linux
            ;;
        "all")
            check_dependencies
            clean_build
            build_frontend
            build_all
            ;;
        "release:mac")
            check_dependencies
            clean_build
            build_frontend
            build_release "mac"
            ;;
        "release:windows")
            check_dependencies
            clean_build
            build_frontend
            build_release "windows"
            ;;
        "release:all")
            check_dependencies
            clean_build
            build_frontend
            build_release "all"
            ;;
        "clean")
            clean_build
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        "")
            print_error "请指定构建目标"
            echo ""
            show_help
            exit 1
            ;;
        *)
            print_error "未知选项: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
    
    print_success "构建完成！"
}

# 运行主函数
main "$@"
