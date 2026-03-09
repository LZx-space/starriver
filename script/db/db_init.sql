-- 切换到预创建的系统默认数据库
\c postgres;

-- 按顺序执行ddl目录下的所有SQL文件
\i ./db/ddl/01_create_db.sql;

-- 切回目标库供后续脚本使用
\c starriver;

\i ./db/ddl/02_create_table_user.sql;
\i ./db/ddl/03_create_table_user_security_event.sql;
\i ./db/ddl/04_create_table_blog.sql;
\i ./db/ddl/05_create_table_dictionary.sql;

-- 按顺序执行dml目录下的所有SQL文件
\i ./db/dml/01_insert_user.sql;

-- 执行完成后输出提示
\echo '所有DDL脚本执行完成！'
