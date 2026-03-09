@echo off
chcp 65001 >nul 2>&1

set "script_path=%~dp0"
echo 当前脚本所在的绝对路径：%script_path%
cd /d "%script_path%"

wsl psql -h localhost -p 5432 -U postgres -d postgres -f ./db/db_init.sql
