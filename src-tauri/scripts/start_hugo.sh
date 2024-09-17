#!/bin/bash

# 设置 Hugo 项目的根目录
hugo_project_dir="/Users/fuhui/Code/src/github.com/think-next/blog"
port=1313 

# 进入 Hugo 项目目录
cd "$hugo_project_dir"

# 启动 Hugo 服务，并设置端口和草稿模式
hugo server -p $port

# 脚本执行完毕后的提示信息
echo "Hugo server started at http://localhost:{$port}"