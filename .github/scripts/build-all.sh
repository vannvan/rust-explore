#!/bin/bash

# Rust Learn 项目构建脚本
# 支持构建所有子项目

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

# 显示帮助信息
show_help() {
    echo "Rust Learn 项目构建脚本"
    echo ""
    echo "用法: $0 [选项] [项目名...]"
    echo ""
    echo "选项:"
    echo "  -h, --help              显示此帮助信息"
    echo "  -a, --all               构建所有项目"
    echo "  -r, --rust              仅构建 Rust 项目"
    echo "  -t, --tauri             仅构建 Tauri 项目"
    echo "  -n, --node              仅构建 Node.js 项目"
    echo "  -d, --debug             构建调试版本 (默认: release)"
    echo "  -p, --platform <平台>   指定目标平台 (macos, windows, linux, all)"
    echo "  --test                  运行测试"
    echo "  --clean                 清理构建目录"
    echo "  --install-deps          安装依赖"
    echo "  --check-deps            检查依赖"
    echo ""
    echo "项目名:"
    echo "  指定要构建的项目名称，不指定则构建所有项目"
    echo ""
    echo "示例:"
    echo "  $0 --all                    # 构建所有项目"
    echo "  $0 --rust                   # 构建所有 Rust 项目"
    echo "  $0 yuque-tools-gui          # 构建特定项目"
    echo "  $0 --platform macos --test  # 构建 macOS 版本并运行测试"
    echo "  $0 --debug --clean          # 构建调试版本并清理目录"
}

# 检查依赖
check_dependencies() {
    print_info "检查构建依赖..."
    
    # 检查 Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust 未安装，请访问 https://rustup.rs/ 安装"
        exit 1
    fi
    
    # 检查 Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js 未安装，请访问 https://nodejs.org/ 安装"
        exit 1
    fi
    
    # 检查 npm
    if ! command -v npm &> /dev/null; then
        print_error "npm 未安装"
        exit 1
    fi
    
    # 检查 Tauri CLI
    if ! command -v tauri &> /dev/null; then
        print_warning "Tauri CLI 未安装，正在安装..."
        npm install -g @tauri-apps/cli@latest
    fi
    
    print_success "依赖检查完成"
}

# 安装依赖
install_dependencies() {
    print_info "安装项目依赖..."
    
    for project in "$@"; do
        if [ -d "$project" ]; then
            print_info "安装 $project 依赖..."
            
            # 安装 Rust 依赖
            if [ -f "$project/Cargo.toml" ]; then
                cd "$project"
                cargo fetch
                cd ..
            fi
            
            # 安装 Node.js 依赖
            if [ -f "$project/package.json" ]; then
                cd "$project"
                npm ci
                cd ..
            fi
        fi
    done
    
    print_success "依赖安装完成"
}

# 清理构建目录
clean_build() {
    print_info "清理构建目录..."
    
    for project in "$@"; do
        if [ -d "$project" ]; then
            print_info "清理 $project 构建目录..."
            
            # 清理 Rust 构建目录
            if [ -d "$project/target" ]; then
                rm -rf "$project/target"
            fi
            
            # 清理 Tauri 构建目录
            if [ -d "$project/src-tauri/target" ]; then
                rm -rf "$project/src-tauri/target"
            fi
            
            # 清理前端构建目录
            if [ -d "$project/dist" ]; then
                rm -rf "$project/dist"
            fi
        fi
    done
    
    print_success "清理完成"
}

# 构建 Rust 项目
build_rust_project() {
    local project="$1"
    local build_type="$2"
    local run_tests="$3"
    
    if [ ! -f "$project/Cargo.toml" ]; then
        return
    fi
    
    print_info "构建 Rust 项目: $project"
    
    cd "$project"
    
    # 构建项目
    if [ "$build_type" = "debug" ]; then
        cargo build
    else
        cargo build --release
    fi
    
    # 运行测试
    if [ "$run_tests" = "true" ]; then
        print_info "运行 $project 测试..."
        if [ "$build_type" = "debug" ]; then
            cargo test
        else
            cargo test --release
        fi
    fi
    
    cd ..
    
    print_success "$project 构建完成"
}

# 构建 Tauri 项目
build_tauri_project() {
    local project="$1"
    local build_type="$2"
    local target_platform="$3"
    local run_tests="$4"
    
    if [ ! -f "$project/src-tauri/Cargo.toml" ] || [ ! -f "$project/package.json" ]; then
        return
    fi
    
    print_info "构建 Tauri 项目: $project"
    
    cd "$project"
    
    # 安装依赖
    npm ci
    
    # 构建前端
    print_info "构建前端..."
    npm run build
    
    # 构建 Tauri 应用
    print_info "构建 Tauri 应用..."
    
    case "$target_platform" in
        "macos")
            if [ "$build_type" = "debug" ]; then
                tauri build --target x86_64-apple-darwin
                tauri build --target aarch64-apple-darwin
            else
                tauri build --target x86_64-apple-darwin --release
                tauri build --target aarch64-apple-darwin --release
            fi
            ;;
        "windows")
            if [ "$build_type" = "debug" ]; then
                tauri build --target x86_64-pc-windows-msvc
            else
                tauri build --target x86_64-pc-windows-msvc --release
            fi
            ;;
        "linux")
            if [ "$build_type" = "debug" ]; then
                tauri build --target x86_64-unknown-linux-gnu
            else
                tauri build --target x86_64-unknown-linux-gnu --release
            fi
            ;;
        "all"|*)
            if [ "$build_type" = "debug" ]; then
                tauri build --target x86_64-apple-darwin
                tauri build --target aarch64-apple-darwin
                tauri build --target x86_64-pc-windows-msvc
                tauri build --target x86_64-unknown-linux-gnu
            else
                tauri build --target x86_64-apple-darwin --release
                tauri build --target aarch64-apple-darwin --release
                tauri build --target x86_64-pc-windows-msvc --release
                tauri build --target x86_64-unknown-linux-gnu --release
            fi
            ;;
    esac
    
    cd ..
    
    print_success "$project Tauri 构建完成"
}

# 构建 Node.js 项目
build_node_project() {
    local project="$1"
    local build_type="$2"
    local run_tests="$3"
    
    if [ ! -f "$project/package.json" ]; then
        return
    fi
    
    print_info "构建 Node.js 项目: $project"
    
    cd "$project"
    
    # 安装依赖
    npm ci
    
    # 构建项目
    if grep -q '"build"' package.json; then
        print_info "构建项目..."
        npm run build
    else
        print_info "项目没有构建脚本，跳过构建"
    fi
    
    # 运行测试
    if [ "$run_tests" = "true" ] && grep -q '"test"' package.json; then
        print_info "运行测试..."
        npm test
    fi
    
    cd ..
    
    print_success "$project Node.js 构建完成"
}

# 检测项目类型
detect_projects() {
    local rust_projects=()
    local tauri_projects=()
    local node_projects=()
    
    for dir in */; do
        if [ -d "$dir" ]; then
            local project_name="${dir%/}"
            
            # 检测 Rust 项目
            if [ -f "$dir/Cargo.toml" ]; then
                rust_projects+=("$project_name")
            fi
            
            # 检测 Tauri 项目
            if [ -f "$dir/src-tauri/Cargo.toml" ]; then
                tauri_projects+=("$project_name")
            fi
            
            # 检测 Node.js 项目
            if [ -f "$dir/package.json" ]; then
                node_projects+=("$project_name")
            fi
        fi
    done
    
    echo "Rust 项目: ${rust_projects[*]}"
    echo "Tauri 项目: ${tauri_projects[*]}"
    echo "Node.js 项目: ${node_projects[*]}"
    
    # 返回项目列表
    if [ "$1" = "rust" ]; then
        echo "${rust_projects[@]}"
    elif [ "$1" = "tauri" ]; then
        echo "${tauri_projects[@]}"
    elif [ "$1" = "node" ]; then
        echo "${node_projects[@]}"
    else
        echo "${rust_projects[@]}" "${tauri_projects[@]}" "${node_projects[@]}"
    fi
}

# 主函数
main() {
    local build_all=false
    local build_rust=false
    local build_tauri=false
    local build_node=false
    local build_type="release"
    local target_platform="all"
    local run_tests=false
    local clean_build=false
    local install_deps=false
    local check_deps=false
    local projects=()
    
    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -a|--all)
                build_all=true
                shift
                ;;
            -r|--rust)
                build_rust=true
                shift
                ;;
            -t|--tauri)
                build_tauri=true
                shift
                ;;
            -n|--node)
                build_node=true
                shift
                ;;
            -d|--debug)
                build_type="debug"
                shift
                ;;
            -p|--platform)
                target_platform="$2"
                shift 2
                ;;
            --test)
                run_tests=true
                shift
                ;;
            --clean)
                clean_build=true
                shift
                ;;
            --install-deps)
                install_deps=true
                shift
                ;;
            --check-deps)
                check_deps=true
                shift
                ;;
            -*)
                print_error "未知选项: $1"
                show_help
                exit 1
                ;;
            *)
                projects+=("$1")
                shift
                ;;
        esac
    done
    
    # 如果没有指定构建类型，默认构建所有
    if [ "$build_all" = false ] && [ "$build_rust" = false ] && [ "$build_tauri" = false ] && [ "$build_node" = false ]; then
        build_all=true
    fi
    
    # 检查依赖
    if [ "$check_deps" = true ]; then
        check_dependencies
    fi
    
    # 如果没有指定项目，检测所有项目
    if [ ${#projects[@]} -eq 0 ]; then
        if [ "$build_all" = true ]; then
            projects=($(detect_projects))
        elif [ "$build_rust" = true ]; then
            projects=($(detect_projects rust))
        elif [ "$build_tauri" = true ]; then
            projects=($(detect_projects tauri))
        elif [ "$build_node" = true ]; then
            projects=($(detect_projects node))
        fi
    fi
    
    # 显示构建信息
    print_info "开始构建项目..."
    print_info "构建类型: $build_type"
    print_info "目标平台: $target_platform"
    print_info "运行测试: $run_tests"
    print_info "项目列表: ${projects[*]}"
    
    # 安装依赖
    if [ "$install_deps" = true ]; then
        install_dependencies "${projects[@]}"
    fi
    
    # 清理构建目录
    if [ "$clean_build" = true ]; then
        clean_build "${projects[@]}"
    fi
    
    # 构建项目
    for project in "${projects[@]}"; do
        if [ ! -d "$project" ]; then
            print_warning "项目 $project 不存在，跳过"
            continue
        fi
        
        # 构建 Rust 项目
        if [ "$build_all" = true ] || [ "$build_rust" = true ]; then
            build_rust_project "$project" "$build_type" "$run_tests"
        fi
        
        # 构建 Tauri 项目
        if [ "$build_all" = true ] || [ "$build_tauri" = true ]; then
            build_tauri_project "$project" "$build_type" "$target_platform" "$run_tests"
        fi
        
        # 构建 Node.js 项目
        if [ "$build_all" = true ] || [ "$build_node" = true ]; then
            build_node_project "$project" "$build_type" "$run_tests"
        fi
    done
    
    print_success "所有项目构建完成！"
}

# 运行主函数
main "$@"
