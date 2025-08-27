@echo off
REM 多平台构建脚本 (Windows)
REM 支持 macOS (Intel/Apple Silicon) 和 Windows

setlocal enabledelayedexpansion

REM 设置颜色代码
set "RED=[91m"
set "GREEN=[92m"
set "YELLOW=[93m"
set "BLUE=[94m"
set "NC=[0m"

REM 打印带颜色的消息
:print_info
echo %BLUE%[INFO]%NC% %~1
goto :eof

:print_success
echo %GREEN%[SUCCESS]%NC% %~1
goto :eof

:print_warning
echo %YELLOW%[WARNING]%NC% %~1
goto :eof

:print_error
echo %RED%[ERROR]%NC% %~1
goto :eof

REM 检查依赖
:check_dependencies
call :print_info "检查构建依赖..."

where npm >nul 2>nul
if %errorlevel% neq 0 (
    call :print_error "npm 未安装"
    exit /b 1
)

where tauri >nul 2>nul
if %errorlevel% neq 0 (
    call :print_error "Tauri CLI 未安装，请运行: npm install -g @tauri-apps/cli"
    exit /b 1
)

call :print_success "依赖检查完成"
goto :eof

REM 清理构建目录
:clean_build
call :print_info "清理构建目录..."
if exist "src-tauri\target" rmdir /s /q "src-tauri\target"
if exist "src-tauri\dist" rmdir /s /q "src-tauri\dist"
call :print_success "清理完成"
goto :eof

REM 构建前端
:build_frontend
call :print_info "构建前端应用..."
call npm run build
if %errorlevel% neq 0 (
    call :print_error "前端构建失败"
    exit /b 1
)
call :print_success "前端构建完成"
goto :eof

REM 构建 macOS 应用
:build_macos
call :print_info "构建 macOS 应用..."

call :print_info "构建 Intel 版本..."
call npm run build:mac-intel
if %errorlevel% neq 0 (
    call :print_error "Intel 版本构建失败"
    exit /b 1
)

call :print_info "构建 Apple Silicon 版本..."
call npm run build:mac-silicon
if %errorlevel% neq 0 (
    call :print_error "Apple Silicon 版本构建失败"
    exit /b 1
)

call :print_success "macOS 应用构建完成"
goto :eof

REM 构建 Windows 应用
:build_windows
call :print_info "构建 Windows 应用..."
call npm run build:windows
if %errorlevel% neq 0 (
    call :print_error "Windows 应用构建失败"
    exit /b 1
)
call :print_success "Windows 应用构建完成"
goto :eof

REM 构建 Linux 应用
:build_linux
call :print_info "构建 Linux 应用..."
call npm run build:linux
if %errorlevel% neq 0 (
    call :print_error "Linux 应用构建失败"
    exit /b 1
)
call :print_success "Linux 应用构建完成"
goto :eof

REM 构建所有平台
:build_all
call :print_info "构建所有平台应用..."
call npm run build:all
if %errorlevel% neq 0 (
    call :print_error "所有平台应用构建失败"
    exit /b 1
)
call :print_success "所有平台应用构建完成"
goto :eof

REM 构建发布版本
:build_release
call :print_info "构建发布版本..."

if "%1"=="mac" (
    call npm run build:release:mac
) else if "%1"=="windows" (
    call npm run build:release:windows
) else if "%1"=="all" (
    call npm run build:release:all
) else (
    call :print_error "未知的发布目标: %1"
    call :print_info "支持的选项: mac, windows, all"
    exit /b 1
)

if %errorlevel% neq 0 (
    call :print_error "发布版本构建失败"
    exit /b 1
)

call :print_success "发布版本构建完成"
goto :eof

REM 显示帮助信息
:show_help
echo 多平台构建脚本 (Windows)
echo.
echo 用法: %0 [选项]
echo.
echo 选项:
echo   mac             构建 macOS 应用 (Intel + Apple Silicon)
echo   windows         构建 Windows 应用
echo   linux           构建 Linux 应用
echo   all             构建所有平台应用
echo   release:mac     构建 macOS 发布版本
echo   release:windows 构建 Windows 发布版本
echo   release:all     构建所有平台发布版本
echo   clean           清理构建目录
echo   help            显示此帮助信息
echo.
echo 示例:
echo   %0 mac           # 构建 macOS 应用
echo   %0 release:all   # 构建所有平台发布版本
echo   %0 clean         # 清理构建目录
goto :eof

REM 主函数
:main
if "%1"=="" (
    call :print_error "请指定构建目标"
    echo.
    call :show_help
    exit /b 1
)

if "%1"=="mac" (
    call :check_dependencies
    call :clean_build
    call :build_frontend
    call :build_macos
) else if "%1"=="windows" (
    call :check_dependencies
    call :clean_build
    call :build_frontend
    call :build_windows
) else if "%1"=="linux" (
    call :check_dependencies
    call :clean_build
    call :build_frontend
    call :build_linux
) else if "%1"=="all" (
    call :check_dependencies
    call :clean_build
    call :build_frontend
    call :build_all
) else if "%1"=="release:mac" (
    call :check_dependencies
    call :clean_build
    call :build_frontend
    call :build_release "mac"
) else if "%1"=="release:windows" (
    call :check_dependencies
    call :clean_build
    call :build_frontend
    call :build_release "windows"
) else if "%1"=="release:all" (
    call :check_dependencies
    call :clean_build
    call :build_frontend
    call :build_release "all"
) else if "%1"=="clean" (
    call :clean_build
) else if "%1"=="help" (
    call :show_help
) else (
    call :print_error "未知选项: %1"
    echo.
    call :show_help
    exit /b 1
)

call :print_success "构建完成！"
goto :eof

REM 运行主函数
call :main %*
