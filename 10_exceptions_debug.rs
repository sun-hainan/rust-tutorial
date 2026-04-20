// ============================================================
// 模块十：异常处理与调试
// 异常分类/自定义/断点/日志
// ============================================================

use std::fmt;
use std::error::Error;

// 【问题1】Rust 的 Result 和 Option 如何替代异常机制？
//
// Rust 没有异常，用 Result<T, E> 处理可恢复错误。
// Option<T> 处理可能不存在的值。
//
// 设计哲学：
//   - panic 用于不可恢复的错误（Bug）
//   - Result 用于可恢复的错误（文件不存在、网络超时）
//   - Option 用于值可能不存在的正常情况

fn result_option_basics() {
    // Option 基本用法
    let maybe = vec![1, 2, 3].get(5);
    println!("get(5) = {:?}", maybe); // None

    // Result 基本用法
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.trim().parse::<i32>()
    }

    match parse_number("42") {
        Ok(n) => println!("parsed: {}", n),
        Err(e) => println!("error: {}", e),
    }

    // ? 运算符（早期返回）
    fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = s.trim().parse()?;
        Ok(n * 2)
    }

    // unwrap_or_else 延迟计算
    let opt: Option<i32> = None;
    let value = opt.unwrap_or_else(|| expensive_default());
    println!("value = {}", value);
}

fn expensive_default() -> i32 {
    println!("computing default...");
    42
}

// 【问题2】Rust 的 panic! 何时使用？如何捕获？
//
// panic! 用于：
//   - 不可恢复的错误（数组越界、除零）
//   - 预期的程序终止
//   - 快速失败（fail fast）
//
// 捕获：std::panic::catch_unwind（仅在多线程中使用）

fn panic_usage() {
    // 手动 panic
    // panic!("this is a critical error!");
    // println!("this line never runs after panic");

    // 数组越界触发 panic
    // let v = [1, 2, 3];
    // let _ = v[10]; // panic: index out of bounds

    // 展开栈（unwinding）vs 中止（aborting）
    // 默认 debug 模式展开，release 模式中止
    // 可以通过 RUST_BACKTRACE 控制回溯

    // catch_unwind 捕获 panic
    let result = std::panic::catch_unwind(|| {
        // vec![1, 2, 3][10]; // 会 panic
        println!("this closure runs normally");
        42
    });
    println!("catch_unwind result: {:?}", result);

    // 捕获后恢复
    let result = std::panic::catch_unwind(|| {
        panic!("intentional panic");
    });
    match result {
        Ok(_) => println!("no panic"),
        Err(_) => println!("caught a panic!"),
    }

    // 设置 panic hook
    std::panic::set_hook(Box::new(|panic_info| {
        println!("CUSTOM PANIC HANDLER: {}", panic_info);
    }));
    // panic!("test custom hook");
}

// 【问题3】Rust 的自定义错误类型如何设计？
//
// 自定义错误实现：
//   - std::error::Error trait
//   - Debug 和 Display trait
//   - source() 提供错误链

fn custom_errors() {
    #[derive(Debug)]
    enum AppError {
        NotFound(String),
        PermissionDenied,
        InvalidInput { field: String, reason: String },
        IoError(std::io::Error),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::NotFound(s) => write!(f, "未找到: {}", s),
                AppError::PermissionDenied => write!(f, "权限不足"),
                AppError::InvalidInput { field, reason } => write!(f, "无效输入 {}: {}", field, reason),
                AppError::IoError(e) => write!(f, "IO 错误: {}", e),
            }
        }
    }

    impl Error for AppError {}

    // 使用错误类型
    fn validate_user(name: &str) -> Result<(), AppError> {
        if name.is_empty() {
            return Err(AppError::InvalidInput {
                field: "name".into(),
                reason: "不能为空".into(),
            });
        }
        if name.len() > 50 {
            return Err(AppError::InvalidInput {
                field: "name".into(),
                reason: "太长（最大50字符）".into(),
            });
        }
        Ok(())
    }

    match validate_user("") {
        Ok(_) => println!("valid user"),
        Err(e) => println!("validation error: {}", e),
    }

    match validate_user("Alice") {
        Ok(_) => println!("valid user"),
        Err(e) => println!("validation error: {}", e),
    }

    // 错误链转换
    fn read_config(path: &str) -> Result<String, AppError> {
        std::fs::read_to_string(path)
            .map_err(|e| AppError::IoError(e))
    }

    match read_config("nonexistent.json") {
        Ok(content) => println!("config: {}", content),
        Err(e) => {
            println!("error: {}", e);
            if let Some(cause) = e.source() {
                println!("caused by: {}", cause);
            }
        }
    }
}

// 【问题4】Rust 的调试技巧——println 和 dbg! 是什么？
//
// println!：格式化输出（release 编译保留）
// dbg!：调试宏，输出文件名、行号、表达式值（debug only）
//
// 机器视角：dbg! 在 release 编译时被移除（#[cfg(debug_assertions)]）

fn debug_prints() {
    let x = 5;
    let y = 3;

    println!("x = {}, y = {}", x, y); // 普通打印
    dbg!(x + y); // 调试打印：输出表达式和值

    let point = (1, 2);
    dbg!(&point); // 借用打印

    // 条件调试
    #[cfg(debug_assertions)]
    println!("DEBUG MODE enabled");

    // 环境变量控制日志
    if std::env::var("DEBUG").is_ok() {
        println!("DEBUG: something happened");
    }
}

// 【问题5】Rust 的日志系统如何使用？
//
// 常用日志库：log + env_logger / tracing
// 日志级别：error, warn, info, debug, trace

fn logging_system() {
    use std::io::Write;
    use log::{info, warn, error, debug};

    // 简单日志初始化
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            writeln!(buf, "[{}] {}: {}",
                record.level(),
                record.target(),
                record.args())
        })
        .init();

    info!("程序启动");
    warn!("这是一个警告");
    error!("这是一个错误");
    debug!("debug 信息（需要 RUST_LOG=debug）");

    // 带上下文记录
    let user_id = 123;
    info!("user {} logged in", user_id);

    // 结构化日志
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    struct Request { method: String, path: String }

    let req = Request { method: "GET".into(), path: "/api/users".into() };
    info!("request: {:?}", req);
}

// 【问题6】Rust 的单元测试和集成测试如何编写？
//
// #[test] 标记测试函数
// assert_eq!, assert_ne!, assert! 断言
// #[should_panic] 预期 panic

fn unit_tests() {
    // 被测试的函数
    fn add(a: i32, b: i32) -> i32 { a + b }
    fn division(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 { Err("除数不能为零".into()) } else { Ok(a / b) }
    }

    // 单元测试
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_division() {
        assert_eq!(division(10, 2).unwrap(), 5);
        assert!(division(10, 0).is_err());
    }

    #[test]
    #[should_panic]
    fn test_panic_expected() {
        panic!("this panic is expected");
    }

    // 测试私有函数（在同一模块）
    fn private_helper(x: i32) -> i32 { x * 2 }
    #[test]
    fn test_private_helper() {
        assert_eq!(private_helper(5), 10);
    }

    println!("单元测试定义完成（实际运行需要 cargo test）");
}

// 【问题7】Rust 的性能分析工具——cargo bench / perf / flamegraph 是什么？
//
// cargo bench：基准测试
// perf（Linux）/ Instruments（macOS）：系统级分析
// cargo-flamegraph：火焰图生成
//
// 编译时加 --release 以优化

fn benchmarking_demo() {
    use std::time::Instant;

    // 简单性能测量
    let start = Instant::now();
    let result = (0..10000_i32).filter(|x| x % 2 == 0).count();
    let elapsed = start.elapsed();
    println!("filtered {} even numbers in {:?}", result, elapsed);

    // cargo bench 格式
    // #[bench] fn bench_filter(b: &mut test::Bencher) { b.iter(|| ...); }

    // micro-benchmark 比较
    fn fast_operation() -> usize {
        (0..1000_i32).sum::<i32>() as usize
    }
    fn slow_operation() -> usize {
        let mut sum = 0;
        for i in 0..1000_i32 { sum += i; }
        sum as usize
    }

    let start = Instant::now();
    for _ in 0..1000 { fast_operation(); }
    println!("fast: {:?}", start.elapsed());

    let start = Instant::now();
    for _ in 0..1000 { slow_operation(); }
    println!("slow: {:?}", start.elapsed());
}

// 【问题8】Rust 的反调试技术——如何检测调试器？
//
// 常用技术：
//   - 检查 /proc/self/status（Linux）
//   - 检测 TracerPid（Linux）
//   - Windows 使用 IsDebuggerPresent
//
// 跨平台库：debugger-info

fn anti_debug() {
    // Linux 检测调试器
    #[cfg(target_os = "linux")]
    fn is_debugger_present() -> bool {
        use std::fs;
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("TracerPid:") {
                    let pid = line.split_whitespace().nth(1).unwrap_or("0");
                    return pid != "0";
                }
            }
        }
        false
    }

    #[cfg(not(target_os = "linux"))]
    fn is_debugger_present() -> bool { false }

    if is_debugger_present() {
        println!("WARNING: Debugger detected!");
    } else {
        println!("No debugger detected");
    }

    // 时间检测（调试器会显著减慢执行）
    use std::time::Instant;
    let start = Instant::now();
    // 小规模计算
    let _ = (0..1000_i32).sum::<i32>();
    let elapsed = start.elapsed();

    if elapsed.as_millis() > 10 {
        println!("WARNING: Execution time unusually long: {:?}", elapsed);
    }
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - panic! 用于不可恢复错误
//   - Result<T, E> 用于可恢复错误
//   - Option<T> 用于可能不存在的值
//   - 无异常传播（? 运算符替代）
//   - 编译时穷尽检查（match 必须处理所有分支）

// Python:
//   - try/except/finally 处理异常
//   - raise 抛出异常
//   - Exception 基类，支持自定义
//   - 有 else 子句（无异常时执行）
//   - 有 finally（始终执行）

// Lua:
//   - error() 抛出错误，pcall/xpcall 捕获
//   - 没有类型化的错误，必须手动检查
//   - nil 表示空值
//   - 没有 try-catch，用 pcall 模拟

// Go:
//   - 无异常，用 error 接口处理错误
//   - 多返回值 (value, error) 模式
//   - errors.New / fmt.Errorf 创建错误
//   - defer 用于资源清理
//   - panic() 用于真正的异常情况

// C++:
//   - try/catch/throw 处理异常
//   - std::exception 基类
//   - noexcept 声明不会抛出的函数
//   - RAII 用于资源管理（析构函数）

fn compare_exceptions() {
    println!("=== 三语言异常处理对比 ===");

    // Rust 强制错误处理
    // Lua 手动检查返回值
    // Python try/except

    // 错误类型化
    // Rust: Result<Success, SpecificError>
    // Python: Exception with type hierarchy
    // Lua: nil or manual error codes
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 实现一个 parse_config 函数，从文件读取配置，返回自定义 AppError 错误类型
// 2. 实现一个 safe_division 函数，使用 Result<i32, String> 处理除零错误
// 3. 写一个测试套件，测试二分查找的边界情况（空数组、单元素、目标在首尾、不存在）
// 4. 解释为什么 Rust 的 Result 必须被处理（编译错误 vs 运行时异常），说明编译器如何强制
// 5. 实现一个自定义 panic hook，捕获并记录 panic 的位置和调用栈

fn main() {
    println!("=== 模块十：异常处理与调试 ===");

    result_option_basics();
    panic_usage();
    custom_errors();
    debug_prints();
    logging_system();
    unit_tests();
    benchmarking_demo();
    anti_debug();
    compare_exceptions();

    println!("\n✅ 所有示例运行成功！");
}