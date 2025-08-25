#!/bin/bash

# 启动前端开发服务器
echo "启动前端开发服务器..."
npm run dev &
FRONTEND_PID=$!

# 等待前端服务器启动
echo "等待前端服务器启动..."
sleep 3

# 检查前端服务器是否真的启动了
echo "检查前端服务器状态..."
if ! curl -s http://localhost:5173 > /dev/null; then
    echo "前端服务器启动失败，等待更长时间..."
    sleep 5
fi

# 构建 Tauri 应用
echo "构建 Tauri 应用..."
cd src-tauri
cargo build

# 尝试运行构建的应用
echo "尝试运行应用..."
if [ -f "../target/debug/yuque-tools-gui" ]; then
    echo "运行构建的应用..."
    ../target/debug/yuque-tools-gui
else
    echo "应用文件未找到，尝试使用 cargo run..."
    cargo run
fi

# 如果应用运行失败，提供错误信息
if [ $? -ne 0 ]; then
    echo "应用运行失败，错误代码: $?"
    echo "请检查 Rust 代码和 Tauri 配置"
fi

# 清理后台进程
echo "清理后台进程..."
kill $FRONTEND_PID 2>/dev/null

echo "开发服务器已停止"

