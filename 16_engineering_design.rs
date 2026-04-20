// ============================================================
// 模块十六：工程化与设计思想
// 设计模式/代码优化/Git
// ============================================================

// 【问题1】Rust 的常用设计模式有哪些？
//
// Rust 特有的设计模式：
//   - RAII（资源获取即初始化）
//   - Builder（构建器模式）
//   - Type State（类型状态机）
//   - Newtype（类型包装）
//   - Flyweight（享元）
//   - Observer（观察者，事件系统）

fn rust_design_patterns() {
    // Builder 模式
    struct Config {
        name: String,
        port: u16,
        debug: bool,
    }

    struct ConfigBuilder {
        name: Option<String>,
        port: Option<u16>,
        debug: Option<bool>,
    }

    impl ConfigBuilder {
        fn new() -> Self { ConfigBuilder { name: None, port: None, debug: None } }
        fn name(&mut self, n: String) -> &mut Self { self.name = Some(n); self }
        fn port(&mut self, p: u16) -> &mut Self { self.port = Some(p); self }
        fn debug(&mut self, d: bool) -> &mut Self { self.debug = Some(d); self }
        fn build(&self) -> Result<Config, String> {
            Ok(Config {
                name: self.name.clone().ok_or("name required")?,
                port: self.port.unwrap_or(8080),
                debug: self.debug.unwrap_or(false),
            })
        }
    }

    let config = ConfigBuilder::new()
        .name("myapp".to_string())
        .port(3000)
        .debug(true)
        .build()
        .unwrap();
    println!("Config: {} on port {}, debug={}", config.name, config.port, config.debug);

    // Type State 模式
    struct Draft;
    struct Published;
    struct Post<S> { content: String, _state: S }

    impl Post<Draft> {
        fn new() -> Self { Post { content: String::new(), _state: Draft } }
        fn add_text(&mut self, text: &str) { self.content.push_str(text); }
        fn request_review(self) -> Post<Published> { Post { content: self.content, _state: Published } }
    }

    impl Post<Published> {
        fn content(&self) -> &str { &self.content }
    }

    let mut draft = Post::<Draft>::new();
    draft.add_text("Hello world!");
    let published = draft.request_review();
    println!("Published post: {}", published.content());

    // Newtype 模式
    struct UserId(u64);
    struct OrderId(u64);

    fn find_user(id: UserId) -> Option<String> {
        println!("finding user {}", id.0);
        Some(format!("User-{}", id.0))
    }

    let uid = UserId(123);
    println!("found: {:?}", find_user(uid));
    // find_user(OrderId(123)); // ❌ 类型错误

    // RAII 模式（自动释放）
    struct FileHandle { fd: i32 }
    impl Drop for FileHandle {
        fn drop(&mut self) { println!("closing fd {}", self.fd); }
    }
    let _fh = FileHandle { fd: 42 };
    println!("file handle created");
    // drop 时自动调用 drop
}

// 【问题2】Rust 的代码优化技巧有哪些？
//
// 1. 避免不必要的克隆
// 2. 使用迭代器链代替循环
// 3. 预分配 Vec 容量
// 4. 使用栈代替堆分配（小型数据结构）
// 5. 避免动态分发（trait object）

fn code_optimization() {
    // 预分配容量
    let mut vec = Vec::with_capacity(1000);
    for i in 0..1000 { vec.push(i); }
    println!("pre-allocated vec with capacity {}", vec.capacity());

    // 迭代器代替循环
    let sum: i32 = (0..1000).filter(|x| x % 2 == 0).sum();
    println!("sum of even numbers 0..1000 = {}", sum);

    // 避免 clone：在所有权转移后继续使用
    let data = vec![1, 2, 3];
    let owned = data; // 所有权转移
    // println!("{:?}", data); // ❌ 已移动
    println!("owned: {:?}", owned);

    // 栈分配代替堆分配（小数组）
    let arr: [i32; 3] = [1, 2, 3];
    println!("stack array: {:?}", arr);

    // 缓存友好：连续内存访问
    fn cache_unfriendly(matrix: &[[i32; 1000]; 1000]) -> i64 {
        let mut sum = 0i64;
        for col in 0..1000 {
            for row in 0..1000 {
                sum += matrix[row][col]; // 按列访问，缓存不友好
            }
        }
        sum
    }

    fn cache_friendly(matrix: &[[i32; 1000]; 1000]) -> i64 {
        let mut sum = 0i64;
        for row in 0..1000 {
            for col in 0..1000 {
                sum += matrix[row][col]; // 按行访问，缓存友好
            }
        }
        sum
    }

    println!("cache optimization important for large data structures");

    // 使用 Cow 避免不必要的分配
    use std::borrow::Cow;
    fn process<'a>(input: &'a str) -> Cow<'a, str> {
        if input.len() < 10 {
            Cow::Borrowed(input) // 不需要分配
        } else {
            Cow::Owned(input.to_uppercase()) // 需要分配
        }
    }
    println!("short: {:?}", process("hi"));
    println!("long: {:?}", process("this is a longer string"));
}

// 【问题3】Rust 的性能分析工具如何使用？
//
// cargo bench：微基准测试
// cargo flamegraph：火焰图（需要 cargo-flamegraph）
// valgrind / Massif：堆分析
// perf（Linux）：CPU 分析

fn performance_tools() {
    println!(" 性能分析工具");
    println!();
    println!("## cargo bench");
    println!("[dev-dependencies]");
    println!("criterion = \"0.5\"");
    println!();
    println!("#[bench] fn my_bench(b: &mut test::Bencher) {{");
    println!("    b.iter(|| {{ /* 测量这段代码 */ }});");
    println!("}}");
    println!();
    println!("## cargo flamegraph（Linux/macOS）");
    println!("$ cargo flamegraph --bin myapp");
    println!("# 生成火焰图，查看 CPU 时间花在哪里");
    println!();
    println!("## perf（Linux）");
    println!("$ perf record --call-graph dwarf ./target/release/myapp");
    println!("$ perf report");
    println!();
    println!("## Massif（堆分析）");
    println!("$ valgrind --tool=massif ./target/release/myapp");
    println!("$ ms_print massif.out.xxxx");
}

// 【问题4】Rust 的单元测试和集成测试如何组织？
//
// 测试文件组织：
//   - #[cfg(test)] 模块内
//   - tests/ 目录下单独文件
//   - examples/ 目录下可运行的示例

fn test_organization() {
    println!(" 测试组织");
    println!();
    println!("## 单元测试（模块内）");
    println!("mod my_module {{");
    println!("    pub fn add(a: i32, b: i32) -> i32 {{ a + b }}");
    println!();
    println!("    #[cfg(test)]");
    println!("    mod tests {{");
    println!("        use super::*;");
    println!("        #[test]");
    println!("        fn test_add() {{");
    println!("            assert_eq!(add(2, 3), 5);");
    println!("        }}");
    println!("    }}");
    println!("}}");
    println!();
    println!("## 集成测试（tests/ 目录）");
    println!("// tests/integration_test.rs");
    println!("use my_crate::{{MyStruct, MyTrait}};");
    println!();
    println!("#[test]");
    println!("fn test_my_struct() {{");
    println!("    let s = MyStruct::new();");
    println!("    // ...测试");
    println!("}}");
    println!();
    println!("## 文档测试");
    println!("/// # Example");
    println!("///");
    println!("/// ```");
    println!("/// assert_eq!(add(1, 2), 3);");
    println!("/// ```");
}

// 【问题5】Rust 的错误处理最佳实践是什么？
//
// 1. 使用 Result 而非 panic 处理可恢复错误
// 2. 自定义错误类型实现 Error trait
// 3. 使用 ? 传播错误
// 4. 避免在库中 panic
// 5. 保持错误类型的一致性

fn error_handling_best_practices() {
    println!(" 错误处理最佳实践");
    println!();
    println!("## 1. 使用 Result 处理错误");
    println!("fn read_config(path: &str) -> Result<Config, ConfigError> {{");
    println!("    // ...可能失败的操作");
    println!("    Ok(config)");
    println!("}}");
    println!();
    println!("## 2. 使用 ? 传播错误");
    println!("fn load() -> Result<Data, Error> {{");
    println!("    let file = open_file()?;");
    println!("    let content = read_to_string(file)?;");
    println!("    parse(content)?");
    println!("}}");
    println!();
    println!("## 3. 自定义错误类型");
    println!("#[derive(Debug)]");
    println!("enum AppError {{");
    println!("    IoError(std::io::Error),");
    println!("    ParseError(String),");
    println!("}}");
    println!();
    println!("## 4. 错误链（source）");
    println!("impl std::error::Error for AppError {{");
    println!("    fn source(&self) -> Option<&dyn std::error::Error> {{");
    println!("        match self {{");
    println!("            AppError::IoError(e) => Some(e),");
    println!("            AppError::ParseError(_) => None,");
    println!("        }}");
    println!("    }}");
    println!("}}");
}

// 【问题6】Rust 的 Cargo 工作空间（workspace）如何使用？
//
// workspace：管理多个相关 crate
// 共享依赖、统一的 Cargo.lock

fn cargo_workspace() {
    println!(" Cargo 工作空间");
    println!();
    println!("## Cargo.toml");
    println!("[workspace]");
    println!("members = [");
    println!("    \"crates/core\",");
    println!("    \"crates/cli\",");
    println!("    \"crates/lib\",");
    println!("]");
    println!();
    println!("## 依赖共享");
    println!("[workspace.dependencies]");
    println!("serde = \"1.0\"");
    println!("tokio = { version = \"1.0\", features = [\"full\"] }");
    println!();
    println!("## 发布");
    println!("$ cargo publish --package my-crate");
}

// 【问题7】Rust 的 Git 工作流最佳实践是什么？
//
// 分支策略：Git Flow / trunk-based
// 提交规范：Conventional Commits
// PR 流程：fork → feature → PR → review → merge

fn git_best_practices() {
    println!(" Git 工作流");
    println!();
    println!("## 提交规范（Conventional Commits）");
    println!("<type>(<scope>): <description>");
    println!();
    println!("feat: add user authentication");
    println!("fix(auth): handle expired tokens");
    println!("docs: update README");
    println!("refactor(parser): improve error messages");
    println!("perf(cache): optimize lookup speed");
    println!();
    println!("## 分支命名");
    println!("- feature/user-authentication");
    println!("- fix/token-expiry");
    println!("- refactor/data-pipeline");
    println!();
    println!("## 常用命令");
    println!("$ git checkout -b feature/new-feature");
    println!("$ git add .");
    println!("$ git commit -m \"feat(scope): description\"");
    println!("$ git push -u origin feature/new-feature");
}

// 【问题8】Rust 的 CI/CD 流程如何配置？
//
// GitHub Actions：常见的 CI 平台
// 测试矩阵：多个 Rust 版本 / 操作系统

fn cicd_setup() {
    println!(" GitHub Actions CI 配置");
    println!();
    println!("# .github/workflows/ci.yml");
    println!("name: CI");
    println!();
    println!("on: [push, pull_request]");
    println!();
    println!("jobs:");
    println!("  test:");
    println!("    runs-on: ubuntu-latest");
    println!("    steps:");
    println!("      - uses: actions/checkout@v4");
    println!("      - uses: dtolnay/rust-toolchain@stable");
    println!("      - run: cargo test");
    println!("      - run: cargo clippy -- -D warnings");
    println!("      - run: cargo fmt --check");
    println!();
    println!("  docs:");
    println!("    runs-on: ubuntu-latest");
    println!("    steps:");
    println!("      - uses: actions/checkout@v4");
    println!("      - uses: dtolnay/rust-toolchain@stable");
    println!("      - run: cargo doc --no-deps");
    println!();
    println!("  fmt:");
    println!("    runs-on: ubuntu-latest");
    println!("    steps:");
    println!("      - uses: actions/checkout@v4");
    println!("      - uses: dtolnay/rust-toolchain@stable");
    println!("      - run: rustfmt --check src/**/*.rs");
}

// 【问题9】Rust 的文档如何编写？
//
// cargo doc：生成文档
// rustdoc: Markdown 中的代码块会被测试
// doc comments: /// 或 //! 语法

fn documentation() {
    println!(" 文档编写");
    println!();
    println!("## 文档注释");
    println!("/// 描述函数功能");
    println!("///");
    println!("/// # Arguments");
    println!("///");
    println!("/// * `name` - 用户名称");
    println!("///");
    println!("/// # Example");
    println!("///");
    println!("/// ```");
    println!("/// let result = greet(\"Alice\");");
    println!("/// assert_eq!(result, \"Hello, Alice!\");");
    println!("/// ```");
    println!("pub fn greet(name: &str) -> String {{");
    println!("    format!(\"Hello, {{}}!\", name)");
    println!("}}");
    println!();
    println!("## 模块级文档");
    println!("//! # My Module");
    println!("//!");
    println!("//! 这个模块提供了核心功能。");
    println!();
    println!("## 构建文档");
    println!("$ cargo doc --open");
    println!("$ cargo doc --no-deps");
}

// ============================================================
// 【对比】Rust vs Lua vs Python 工程化
// ============================================================
// Rust:
//   - Cargo：内置构建/包管理
//   - Rustfmt：代码格式化
//   - Clippy：Linting
//   - 编译时检查多，运行时错误少

// Lua:
//   - LuaRocks：包管理器
//   - 无内置格式化/Lint
//   - 动态类型，错误在运行时发现

// Python:
//   - pip/poetry/conda：包管理
//   - black/isort/ruff：格式化/Lint
//   - pytest/mypy：测试/类型检查
//   - 类型注解（Python 3.5+）

fn compare_engineering() {
    println!("=== 三语言工程化对比 ===");
    println!();
    println!("| 方面        | Rust     | Python      | Lua      |");
    println!("|-------------|----------|-------------|----------|");
    println!("| 包管理      | Cargo    | pip/poetry   | LuaRocks |");
    println!("| 格式化      | rustfmt  | black        | 无       |");
    println!("| Lint        | clippy   | ruff/pylint | luacheck|");
    println!("| 类型检查    | 编译时   | mypy         | 无       |");
    println!("| 测试框架    | #[test]  | pytest       | busted   |");
    println!("| 文档        | rustdoc  | sphinx       | 无内置   |");
    println!("| CI/CD       | GitHub Actions | GitHub Actions | 通用 |");
}

// ============================================================
// 练习题
// ============================================================
// 1. 实现一个 Builder 模式的 HTTP 请求构建器
// 2. 编写一个带性能测试的 benchmark（使用 criterion）
// 3. 配置 GitHub Actions CI，包含 test/clippy/fmt
// 4. 解释 RAII 模式在 Rust 中的应用（File/锁/连接）

fn main() {
    println!("=== 模块十六：工程化与设计思想 ===");

    rust_design_patterns();
    code_optimization();
    performance_tools();
    test_organization();
    error_handling_best_practices();
    cargo_workspace();
    git_best_practices();
    cicd_setup();
    documentation();
    compare_engineering();

    println!("\n✅ 所有示例运行成功！");
}