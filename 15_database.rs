// ============================================================
// 模块十五：数据库编程
// SQL/事务/ORM
// ============================================================

use std::collections::HashMap;

// 【问题1】Rust 如何连接 SQLite？（rusqlite 示例）
//
// rusqlite：SQLite 的 Rust 绑定
// 支持事务、PreparedStatement、类型映射

fn sqlite_demo() {
    // 注意：运行需要 rusqlite 依赖
    println!(" SQLite 示例（需要 rusqlite 依赖）");
    println!();
    println!("use rusqlite::{{Connection, Result}};");
    println!();
    println!("fn main() -> Result<()> {{");
    println!("    let conn = Connection::open(\"test.db\")?;");
    println!();
    println!("    conn.execute(\"CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)\", [])?;");
    println!("    conn.execute(\"INSERT INTO users (name) VALUES (?1)\", [\"Alice\"])?;");
    println!();
    println!("    let mut stmt = conn.prepare(\"SELECT id, name FROM users\")?;");
    println!("    for row in stmt.query([])? {{");
    println!("        println!(\"id={}, name={}\", row.get(0)?, row.get(1)?);");
    println!("    }}");
    println!("    Ok(())");
    println!("}}");
}

// 【问题2】Rust 的 SQL 基本操作（CRUD）如何实现？
//
// CRUD：Create, Read, Update, Delete
// 参数化查询防 SQL 注入

fn sql_crud() {
    println!(" CRUD 操作示例");
    println!();
    println!("// 插入数据");
    println!("conn.execute(\"INSERT INTO users (name, age) VALUES (?1, ?2)\",");
    println!("    params![\"Alice\", 30])?;");
    println!();
    println!("// 查询数据");
    println!("let mut stmt = conn.prepare(\"SELECT name, age FROM users WHERE age > ?1\")?;");
    println!("let users = stmt.query_map([18], |row| {{");
    println!("    Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))");
    println!("}})?;");
    println!();
    println!("// 更新数据");
    println!("conn.execute(\"UPDATE users SET age = ?1 WHERE name = ?2\",");
    println!("    params![31, \"Alice\"])?;");
    println!();
    println!("// 删除数据");
    println!("conn.execute(\"DELETE FROM users WHERE name = ?1\", [\"Bob\"])?;");
}

// 【问题3】Rust 的事务处理如何实现？
//
// 事务：ACID（原子性、一致性、隔离性、持久性）
// BEGIN / COMMIT / ROLLBACK

fn transaction_demo() {
    println!(" 事务处理示例");
    println!();
    println!("conn.execute(\"BEGIN\", [])?;");
    println!();
    println!("let result = (|| -> Result<()> {{");
    println!("    // 第一个操作");
    println!("    conn.execute(\"INSERT INTO accounts (id, balance) VALUES (1, 100)\", [])?;");
    println!();
    println!("    // 第二个操作");
    println!("    conn.execute(\"UPDATE accounts SET balance = balance - 50 WHERE id = 1\", [])?;");
    println!();
    println!("    // 检查余额");
    println!("    let balance: i32 = conn.query_row(");
    println!("        \"SELECT balance FROM accounts WHERE id = 1\",");
    println!("        [], |row| row.get(0))?;");
    println!();
    println!("    if balance < 0 {{ return Err(\"余额不足\".into()); }}");
    println!();
    println!("    Ok(())");
    println!("}})();");
    println!();
    println!("if result.is_ok() {{");
    println!("    conn.execute(\"COMMIT\", [])?;");
    println!("}} else {{");
    println!("    conn.execute(\"ROLLBACK\", [])?;");
    println!("}}");
}

// 【问题4】Rust 的 SQL 注入防御如何实现？
//
// 参数化查询（PreparedStatement）是唯一安全方式。
// 绝对不要拼接用户输入到 SQL 字符串。

fn sql_injection_prevention() {
    println!(" SQL 注入防御");
    println!();
    println!("// ❌ 危险：字符串拼接");
    println!("let query = format!(\"SELECT * FROM users WHERE name = '{}'\", user_input);");
    println!("conn.execute(&query, [])?; // user_input = \"' OR '1'='1\" 会导致问题");
    println!();
    println!("// ✅ 安全：参数化查询");
    println!("conn.execute(\"SELECT * FROM users WHERE name = ?1\", [user_input])?;");
    println!();
    println!("// ?1 占位符会被正确转义，防止注入攻击");
}

// 【问题5】Rust 的 ORM 框架——Diesel / SQLx 是什么？
//
// Diesel：类型安全的 ORM，编译时检查 SQL
// SQLx：异步 SQL 工具箱，运行时检查
//
// Diesel 示例：
//   - 定义 schema（diesel.toml）
//   - 定义模型（use diesel::prelude::*）
//   - 查询构建器

fn orm_intro() {
    println!(" ORM 框架对比");
    println!();
    println!("## Diesel");
    println!("- 编译时 SQL 检查");
    println!("- 类型安全的查询构建器");
    println!("- 支持 PostgreSQL, MySQL, SQLite");
    println!("- 需要 diesel.toml 配置文件");
    println!();
    println!("## SQLx");
    println!("- 异步支持（async/.await）");
    println!("- 运行时检查（prepare 时验证）");
    println!("- 支持连接池");
    println!("- 无宏开销");
    println!();
    println!("## sqlx 示例");
    println!("use sqlx::{{PgPool, Row}};");
    println!();
    println!("let pool = PgPool::connect(\"postgres://user:pass@localhost/db\").await?;");
    println!("let row: (i32, String) = sqlx::query_as(\"SELECT id, name FROM users WHERE id = $1\")");
    println!("    .bind(1_i32)");
    println!("    .fetch_one(&pool)");
    println!("    .await?;");
}

// 【问题6】Rust 的连接池如何实现？（r2d2 示例）
//
// 连接池：复用数据库连接，减少连接开销。
// r2d2 是 Rust 常用的连接池库。

fn connection_pool_demo() {
    println!(" 连接池示例（r2d2）");
    println!();
    println!("use r2d2::Pool;");
    println!("use r2d2_sqlite::SqliteConnectionManager;");
    println!();
    println!("let manager = SqliteConnectionManager::file(\"test.db\");");
    println!("let pool = Pool::builder()");
    println!("    .max_size(10) // 最大连接数");
    println!("    .build(manager)?;");
    println!();
    println!("// 获取连接");
    println!("let conn = pool.get()?;");
    println!("conn.execute(\"SELECT ...\", [])?;");
    println!();
    println!("// 连接自动归还到池中");
}

// 【问题7】Rust 的数据库迁移如何管理？
//
// 迁移：版本化数据库 schema 变更。
// diesel migrations 命令行工具。
// sqlx-cli 也提供迁移支持。

fn migration_demo() {
    println!(" 数据库迁移");
    println!();
    println!("## Diesel 迁移");
    println!("创建 migrations/ 文件夹，每迁移一个 SQL 文件：");
    println!("  2024-01-01-000000_create_users/up.sql");
    println!("  2024-01-01-000000_create_users/down.sql");
    println!();
    println!("// up.sql");
    println!("CREATE TABLE users ({{");
    println!("    id SERIAL PRIMARY KEY,");
    println!("    name VARCHAR(255) NOT NULL");
    println!("}});");
    println!();
    println!("// diesel.toml");
    println!("[print_schema]");
    println!("file = \"src/schema.rs\"");
    println!("filter = {{ only_tables = [\"users\"] }}");
    println!();
    println!("// 运行迁移");
    println!("diesel migration run");
    println!("diesel migration redo");
}

// 【问题8】Rust 的 NoSQL 支持如何？（Redis / MongoDB）
//
// Redis：redis-rs / redis-rs-async
// MongoDB：mongodb crate

fn nosql_support() {
    println!(" NoSQL 数据库支持");
    println!();
    println!("## Redis（redis-rs）");
    println!("use redis::{{Client, AsyncCommands}};");
    println!();
    println!("let client = Client::open(\"redis://127.0.0.1/\")?;");
    println!("let mut con = client.get_connection()?;");
    println!();
    println!("// SET / GET");
    println!("con.set(\"key\", \"value\")?;");
    println!("let val: String = con.get(\"key\")?;");
    println!();
    println!("// Hash");
    println!("con.hset(\"user:1\", \"name\", \"Alice\")?;");
    println!("let name: String = con.hget(\"user:1\", \"name\")?;");
    println!();
    println!("## MongoDB（mongodb）");
    println!("use mongodb::{{Client, bson::doc}};");
    println!();
    println!("let client = Client::with_uri_str(\"mongodb://localhost:27017\").await?;");
    println!("let db = client.database(\"test\");");
    println!("let coll = db.collection::<Document>(\"users\");");
    println!();
    println!("coll.insert_one(doc! {{ \"name\": \"Alice\", \"age\": 30 }}).await?;");
}

// ============================================================
// 【对比】Rust vs Lua vs Python 数据库
// ============================================================
// Rust:
//   - rusqlite：SQLite
//   - diesel：ORM（编译时检查）
//   - sqlx：异步 SQL（运行时检查）
//   - redis-rs：Redis
//   - 类型安全，连接池支持

// Lua:
//   - luasql：SQLite/MySQL/PostgreSQL
//   - redis-lua：Redis
//   - LuaSQL 使用方式类似
//   - 动态类型，简单易用

// Python:
//   - sqlite3：内置
//   - SQLAlchemy：ORM
//   - psycopg2/asyncpg：PostgreSQL
//   - redis-py：Redis
//   - PyMongo：MongoDB

fn compare_databases() {
    println!("=== 三语言数据库对比 ===");

    println!();
    println!("| 特性       | Rust        | Python           | Lua          |");
    println!("|------------|-------------|------------------|--------------|");
    println!("| SQLite     | rusqlite    | sqlite3 (内置)    | luasql-sqlite|");
    println!("| ORM        | Diesel/SQLx | SQLAlchemy       | 无           |");
    println!("| 异步       | sqlx        | asyncpg/aiohttp   | nginx cosocket|");
    println!("| Redis      | redis-rs    | redis-py         | redis-lua    |");
    println!("| 类型安全   | 编译时       | 运行时           | 无           |");
}

// ============================================================
// 练习题
// ============================================================
// 1. 用 rusqlite 创建表、插入数据、查询数据
// 2. 实现一个事务包装函数，确保操作的原子性
// 3. 解释 PreparedStatement 的原理和防注入机制
// 4. 对比 Diesel 和 SQLx 的设计理念

fn main() {
    println!("=== 模块十五：数据库编程 ===");

    sqlite_demo();
    sql_crud();
    transaction_demo();
    sql_injection_prevention();
    orm_intro();
    connection_pool_demo();
    migration_demo();
    nosql_support();
    compare_databases();

    println!("\n✅ 所有示例运行成功！");
}