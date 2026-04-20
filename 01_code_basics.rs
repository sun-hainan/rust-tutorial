// ============================================================
// 模块一：代码规范与基础
// 注释 / 缩进 / 标识符 / 关键字
// ============================================================

// 【问题1】为什么要写注释？注释会被编译器保留吗？
//
// Rust 中有两种注释：
//   - 行注释 //        → 编译器完全忽略
//   - 文档注释 ///     → 会被编译进 crate 文档（rustdoc）
//
// 从编译原理看：注释是词法分析阶段的"令牌"，最终不生成任何目标码。
// 从机器层面看：注释在编译后完全消失，不影响二进制体积。

/// 这是一个文档注释，可以描述下面的函数
/// 用于生成 API 文档（类似 Javadoc / Python docstring）
fn greet(name: &str) -> String {
    format!("Hello, {}!", name) // 行内注释放行尾
}

// 【问题2】Rust 对缩进有何强制要求？空格 vs Tab？
//
// Rust 使用空白字符分隔 token，空格和 Tab 会产生不同的缩进效果。
// Rust 编译器不强制缩进大小，但社区规范使用 4 空格。
//
// 机器视角：缩进影响词法分析器对"缩进级别"的判定（indent/dedent 令牌）。
// 编译器视角：Rust 使用"无缩进语法"，不需要强制缩进检查，只要不产生歧义即可。
//
// 最佳实践：
//   - 使用 4 空格（IDE 自动转换 Tab）
//   - 保持每行不超过 80 字符（或 100）
//   - 多行参数对齐使用 &str 模式匹配

fn aligned_args(
    long_param_name: &str,
    another_long_param: i32,
    yet_another: f64,
) -> String {
    format!("{} {} {}", long_param_name, another_param, yet_another)
}

// 【问题3】标识符有哪些规则？哪些是 Rust 关键字？
//
// 标识符命名规则：
//   - 由字母、数字、下划线组成
//   - 不能以数字开头
//   - 区分大小写
//
// Rust 关键字（reserved keywords）：不能用作标识符
//   - 所有权相关：fn, let, mut, ref, static, struct, enum, trait, impl, type
//   - 控制流：if, else, match, loop, while, for, in, return, break, continue
//   - 模块相关：mod, use, pub, crate, super, self, Self
//   - 其他：as, where, async, await, dyn, move, unsafe, extern, const
//
// 编译器视角：关键字在词法分析时被识别为 TK_KEYWORD，不是 TK_IDENTIFIER
// 语法分析阶段会检查关键字在特定上下文中的使用是否合法

fn identifier_examples() {
    // 合法标识符
    let snake_case = 1;    // 小写蛇形（Rust 标准风格）
    let camel_case = 2;    // 驼峰命名
    let _private_var = 3;  // 下划线开头表示未使用警告抑制
    let __dunder = 4;      // 双下划线保留给编译器/标准库

    // 非法：不能以数字开头
    // let 2fast = 5;      // ❌ 编译错误

    // 关键字不能用作标识符
    // let fn = 5;         // ❌ 编译错误
    // let match = 6;      // ❌ 编译错误

    let r#return = 7;      // ✅ 转义关键字（raw identifier）
    println!("r#return = {}", r#return);
}

// 【问题4】什么是原始标识符（raw identifier）？
//
// 当与旧版 Rust 或其他语言交互时，某些版本的关键字可能与我们的标识符冲突。
// Rust 允许使用 r# 前缀将关键字转义为普通标识符。
//
// 使用场景：
//   - FFI 调用 C 函数时，C 函数名恰好是 Rust 关键字
//   - 老版本 crate 升级后新关键字与你的变量名冲突

fn raw_identifier_demo() {
    // Rust 2015 中 "dyn" 不是关键字
    // Rust 2018+ 中 "dyn" 成为关键字
    let r#dyn = "dynamic"; // 绕过关键字冲突
    println!("raw dyn = {}", r#dyn);
}

// 【问题5】Rust 的命名规范是什么？社区约定一览
//
// Rust 社区有一套 PEP8 类似的命名约定（Rust naming conventions）：
//
//   - 变量 / 函数 / 方法：snake_case（蛇形）
//   - 类型 / Trait / 结构体：PascalCase（驼峰）
//   - 常量（const）：SCREAMING_SNAKE_CASE（全大写下划线）
//   - 生命周期：'a, 'static（全小写字母前缀）
//
// 编译器层面：命名规范不强制，但 Clippy（lint 工具）会检查并警告。

fn naming_conventions_demo() {
    // 变量：snake_case
    let max_buffer_size = 1024;

    // 常量：SCREAMING_SNAKE_CASE
    const MAX_RETRY_COUNT: u32 = 3;

    // 类型：PascalCase
    struct UserProfile { name: String, age: u32 }

    // Trait：PascalCase
    trait Serializable { fn serialize(&self) -> Vec<u8>; }

    // 方法：snake_case
    impl UserProfile {
        fn full_name(&self) -> String { self.name.clone() }
    }

    println!("max_buffer_size = {}", max_buffer_size);
}

// 【问题6】Rust 的特殊下划线变量名有什么含义？
//
// 单下划线 _ 是一个特殊的"丢弃"模式（discard pattern）。
// 用于告诉编译器：这个值我不需要，不需要警告"未使用变量"。
//
// 编译器视角：_ 在模式匹配中被编译器特殊处理，不绑定到任何变量，
// 因此不会产生"未使用变量"的警告。

fn underscore_usage() {
    let _unused = 42;        // 普通变量但未使用 → 警告（如果 crate 级别设置了 unused 警告）
    let _ignored = "hello"; // 下划线开头 → 编译器直接忽略，不产生警告

    // 模式匹配中丢弃不需要的值
    let tuple = (1, 2, 3);
    let (_, second, _) = tuple; // 只取第二个值
    println!("second = {}", second);

    // 在解构中忽略字段
    struct Point { x: i32, y: i32 }
    let pt = Point { x: 10, y: 20 };
    let Point { x: _, y: _ } = pt; // 忽略所有字段（完整解构但丢弃）

    // 多次使用下划线
    let (_, _, third) = (10, 20, 30); // 三次下划线分别丢弃三个值
    println!("third = {}", third);
}

// ============================================================
// 【对比】Rust vs Lua vs Python
// ============================================================
// Rust:
//   - // 行注释，/* */ 块注释，/// 文档注释
//   - 缩进不强制（但社区用 4 空格）
//   - 关键字不能直接用作标识符，可用 r# 转义
//   - _ 表示"丢弃"，不会触发未使用警告

// Lua:
//   -- 单行注释
//   --[[ ]] 多行注释
//   缩进不强制，标识符不能以数字开头，关键字不能用作标识符
//   Lua 用 _ 作为"nil"值的占位符（如 local _, err = pcall(...)）

// Python:
//   # 单行注释
//   """ 或 ''' 用于文档字符串（同时也是多行注释）
//   缩进强制（4 空格是 PEP8 标准），标识符不能是关键字
//   _ 在交互模式中表示上一个表达式的值

fn compare_with_lua_python() {
    // Rust: 缩进不影响语法，注释不进入目标码
    let msg = "hello from rust";

    // Rust 的 _ 是"丢弃"模式
    let (_, result) = (Ok(42), "success");
    println!("result = {}", result);
}

// ============================================================
// 练习题
// ============================================================
// 1. 写一个带文档注释的函数，计算两数之和
// 2. 解释为什么 Rust 允许 r# 作为关键字前缀
// 3. 将以下 Python 代码转换为 Rust（注意命名规范）：
//    max_speed = 100
//    MyClass = "hello"

// ============================================================
// 总结
// ============================================================
// | 特性         | Rust 行为                                      |
// |-------------|-----------------------------------------------|
// | 行注释       | // 编译器忽略                                   |
// | 块注释       | /* */ 编译器忽略                                |
// | 文档注释     | /// 会编译进 rustdoc                            |
// | 缩进         | 不强制，但建议 4 空格                           |
// | 关键字       | 不能直接用作标识符，r# 前缀可转义                |
// | 下划线变量    | _ 表示丢弃，不触发未使用警告                   |
// | 命名规范     | snake_case(变量), PascalCase(类型), SCREAMING(常量) |

fn main() {
    println!("=== 模块一：代码规范与基础 ===");

    // 测试所有函数
    let greeting = greet("Rust Learner");
    println!("{}", greeting);

    identifier_examples();
    raw_identifier_demo();
    naming_conventions_demo();
    underscore_usage();
    compare_with_lua_python();

    println!("\n✅ 所有示例运行成功！");
}