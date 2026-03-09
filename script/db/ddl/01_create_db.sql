-- 1. 确保当前会话在postgres库（即使命令行已指定-d postgres，再加这行更保险）
\c postgres;

-- 直接删库（当前会话连的是postgres库，无冲突）
DROP DATABASE IF EXISTS starriver;
CREATE DATABASE starriver;

-- 切回目标库供后续脚本使用
\c starriver;
