// ============================================================
// 模块二：变量与数据类型
// 变量 / 常量 / 作用域 / 生命周期 / 基本类型 / 引用类型 / 类型转换 / 空值 / 进制
// ============================================================

// 【问题1】Rust 的变量声明与不可变性是什么关系？
//
// 默认：let 声明的变量是不可变的（immutable）。
// 可变：必须显式使用 mut 关键字。
//
// 编译器视角：不可变变量在编译时就被标记，编译器可以：
//   - 进行更多优化（如常量折叠、循环不变代码外提）
//   - 检测到意外的多重写入会报编译错误（数据竞争预防）
//
// 机器视角：不可变变量可以放在寄存器或栈上，运行时不需要写屏障（write barrier）。

fn variable_immutability() {
    let x = 5;
    // x = 6; // ❌ 编译错误：cannot assign to immutable variable `x`

    let mut y = 5;
    y = 6; // ✅ 可变变量
    println!("y = {}", y);

    // 不可变绑定 vs 可变绑定的本质区别：
    //   - 不可变：编译时确定"不会变"，优化器可以假设值不变
    //   - 可变：运行时会变，编译器保守处理
}

// 【问题2】Rust 的所有权（ownership）机制是什么？
//
// Rust 的核心创新：所有权系统，无 GC 但无数据竞争。
// 规则：
//   1. 每个值有且只有一个所有者
//   2. 当所有者离开作用域，值被 drop（释放内存）
//   3. 值的所有权可以移动（move）到另一个变量
//
// 机器视角：所有权本质上是对"堆内存分配"的追踪。
// 编译器在编译时计算每个值的生命周期，插入 free() 调用。
// 避免了手动 free 的遗漏（use-after-free）和双重释放（double-free）。

fn ownership_demo() {
    let s1 = String::from("hello"); // s1 拥有堆上的字符串数据
    let s2 = s1;                    // 所有权从 s1 移动到 s2
    // println!("{}", s1);          // ❌ 编译错误：s1 的所有权已移动
    println!("s2 = {}", s2);

    // 整数等 Copy 类型不受影响（它们在栈上）
    let a = 5;
    let b = a; // Copy：a 的值被复制，a 仍有效
    println!("a = {}, b = {}", a, b);
}

// 【问题3】借用（borrowing）与引用（&）是什么？
//
// 借用：允许不获取所有权而使用某个值。
// 语法：&T 表示不可变引用，&mut T 表示可变引用。
//
// 规则：
//   - 任意时刻：要么有多个不可变引用，要么有唯一的一个可变引用（二选一）
//   - 引用必须始终有效（无空引用 / 无悬垂引用）
//
// 编译器视角：借用检查器（borrow checker）在编译时验证这些规则。
// 它通过构造借用图（borrow graph）来检测是否违反规则。

fn borrowing_demo() {
    let mut s = String::from("hello");

    // 不可变借用
    let len = calculate_length(&s);
    println!("'{}' 的长度是 {}", s, len);

    // 可变借用
    append_world(&mut s);
    println!("追加后: {}", s);

    // 借用规则验证：
    // let r1 = &s;     // 第一个不可变引用
    // let r2 = &s;     // 第二个不可变引用 ← OK（多个不可变引用）
    // let r3 = &mut s; // 可变引用 ← ❌ 编译错误（同时存在不可变和可变）
}

fn calculate_length(s: &String) -> usize {
    s.len() // 借用，不获取所有权
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

// 【问题4】Rust 的作用域与生命周期（lifetime）是什么？
//
// 生命周期：引用保持有效的时间范围。
// 编译器用 'a 等生命周期参数来追踪引用的有效范围。
//
// 机器视角：每个引用包含一个指针和一个"租期"信息，
// 编译器在编译时计算引用的有效范围，在超出范围时禁止使用。

fn lifetime_scope_demo() {
    let outer_var = String::from("outer");

    {
        let inner_var = String::from("inner");
        // inner_var 的生命周期只在内部块内
        let ref1 = &inner_var;
        println!("inner ref = {}", ref1);
        // 块结束时 inner_var drop，ref1 无效
    }
    // outer_var 仍有效（生命周期更长）
    println!("outer ref = {}", outer_var);

    // 生命周期参数示例
    let longest;
    {
        let s1 = String::from("short");
        let s2 = String::from("very long string");
        longest = longest_string(&s1, &s2); // 'a 被推断为 s1 和 s2 中较短的那个
        println!("longest = {}", longest);
    }
    // longest 的生命周期从创建到块结束
}

// 生命周期标注：告诉编译器两个引用的生命周期关系
fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 【问题5】Rust 的基本数据类型有哪些？
//
// 整数：i8, i16, i32, i64, i128, isize（指针大小）
//       u8, u16, u32, u64, u128, usize
// 浮点：f32, f64
// 布尔：bool（true / false）
// 字符：char（Unicode Scalar Value，4字节）
// 单元：()（表示无返回值或空值）

fn primitive_types() {
    // 整数
    let decimal = 42;          // i32 默认
    let hex_val = 0xFF;       // 十六进制
    let octal = 0o77;         // 八进制
    let binary = 0b1010;      // 二进制
    let byte = b'A';          // u8 字面量（字节）

    // 显式类型标注
    let big: i64 = 9223372036854775807;
    let ptr: usize = 0usize;

    // 浮点
    let pi: f64 = 3.141592653589793;
    let e: f32 = 2.71828;

    // 布尔
    let is_rust_fun: bool = true;
    let not_ready: bool = false;

    // 字符（Unicode）
    let c1: char = 'A';
    let c2: char = '中';
    let emoji: char = '🚀';

    println!("decimal = {}", decimal);
    println!("hex = {:#x}", hex_val);
    println!("binary = {:b}", binary);
    println!("pi = {:.15}", pi);
    println!("emoji = {}", emoji);
    println!("char size: {} bytes", std::mem::size_of::<char>());
}

// 【问题6】Rust 的复合类型（结构体 / 元组）是什么？
//
// 元组（Tuple）：固定长度、异构类型、下标访问
// 结构体（Struct）：命名字段
//
// 元组在栈上分配，适合小型固定集合。
// 结构体可以 impl 方法。

fn compound_types() {
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;              // 解构
    println!("tup.0 = {}, tup.1 = {}, x = {}", tup.0, tup.1, x);

    // 元组作为函数返回值
    fn min_max(arr: &[i32]) -> (i32, i32) {
        let min = *arr.iter().min().unwrap_or(&0);
        let max = *arr.iter().max().unwrap_or(&0);
        (min, max)
    }
    let (min, max) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min = {}, max = {}", min, max);

    // 结构体
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 { self.width * self.height }
        fn new(w: u32, h: u32) -> Self {
            Rectangle { width: w, height: h }
        }
    }

    let rect = Rectangle { width: 30, height: 50 };
    println!("矩形面积 = {} 平方像素", rect.area());

    let sq = Rectangle::new(10, 10);
    println!("正方形面积 = {} 平方像素", sq.area());
}

// 【问题7】Rust 如何处理类型转换？
//
// Rust 没有隐式类型转换（安全性设计）。
// 必须显式使用 as 进行类型转换。
//
// 数值转换规则：
//   - 小类型 → 大类型：通常是安全的（但可能丢失精度）
//   - 大类型 → 小类型：需要 as（可能截断）
//   - 浮点 → 整数：向零截断
//   - 整数 → 浮点：可能丢失大数精度

fn type_conversion() {
    // 整数到整数
    let small: i32 = 1000;
    let big: i64 = small as i64;        // 小→大，安全
    let truncated: i8 = small as i8;   // 大→小，截断（1000 mod 128 = -24）
    println!("big = {}, truncated = {}", big, truncated);

    // 整数到浮点
    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64; // i32 → f64
    println!("int to float = {}", float_val);

    // 浮点到整数（向零截断）
    let pi: f64 = 3.14159;
    let truncated_int: i32 = pi as i32;  // 3（截断小数部分）
    println!("pi truncated = {}", truncated_int);

    // char 转换
    let code: u8 = 65;
    let ch = code as char;              // u8 → char（ASCII）
    println!("code {} = char '{}'", code, ch);

    // 布尔转换
    let t: bool = true;
    let from_bool: i32 = t as i32;     // true → 1, false → 0
    println!("true as i32 = {}", from_bool);

    // 类型推断 + 转换
    let guessed = "42";
    let number: i32 = guessed.trim().parse::<i32>().unwrap();
    println!("parsed: {}", number);
}

// 【问题8】Rust 的空值处理——Option 是什么？为什么没有 null？
//
// Option<T> 是 Rust 处理"可能无值"的惯用方式：
//   enum Option<T> { Some(T), None }
//
// 设计理念：
//   - Tony Hoare 发明了 null，称其为"billion dollar mistake"
//   - null 引用导致运行时错误（NullPointerException）
//   - Rust 在编译时消除 null，通过类型系统确保"无值"状态必须被处理
//
// 编译器视角：Option 的 Some 和 None 是枚举变体，
// match 表达式会进行穷尽检查，确保每个情况都被处理。

fn option_handling() {
    // Option 基本用法
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;

    match some_value {
        Some(v) => println!("got value: {}", v),
        None => println!("no value"),
    }

    // if let 简化
    if let Some(v) = no_value {
        println!("value = {}", v);
    } else {
        println!("was None");
    }

    // map / unwrap_or
    let result: Option<i32> = Some(10);
    let doubled = result.map(|x| x * 2); // Some(20)
    let fallback = result.unwrap_or(0); // 10
    println!("doubled = {:?}, fallback = {}", doubled, fallback);

    // 实战：从 HashMap 中查找
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);

    let alice_score = scores.get("Alice");       // Option<&i32>
    let unknown = scores.get("Bob");             // None
    println!("Alice's score = {:?}", alice_score);
    println!("Bob's score = {:?}", unknown);

    // 安全地处理空值
    let name = scores.get("Charlie").copied().unwrap_or(0);
    println!("Charlie's score (default 0) = {}", name);
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - let 声明变量，默认不可变，mut 可变
//   - 所有权：每个值唯一所有者，move 语义
//   - 借用：& 不可变，&mut 可变，无空引用
//   - Option<T> 替代 null，编译时确保空值处理
//   - 基本类型：i32/u32/f64/bool/char，复合：tuple/struct/enum
//   - 类型转换必须显式 as，无隐式转换

// Python:
//   - x = value，动态类型，可重新赋值
//   - 垃圾回收（引用计数 + 标记清除）
//   - 无借用概念，但可变对象可以通过引用传递
//   - None 表示空值（NoneType）
//   - 基本类型：int/float/bool/str/complex，复合：list/dict/tuple/set
//   - 类型转换可用 int()/str()/float()，但字符串"123" + 1 会报错

// Lua:
//   - local x = value，无类型声明，变量可重新赋值任何类型
//   - 无所有权概念，垃圾回收自动处理
//   - 无借用概念，直接访问变量
//   - nil 表示空值（Lua 唯一"空"的表示）
//   - 基本类型：nil/boolean/number/string/function/table/userdata/thread
//   - 类型转换自动进行（"123" + 1 = 124）

// Go:
//   - var x int = 0，var 声明，:= 短声明，默认零值
//   - 垃圾回收（并发标记清扫）
//   - 指针 &T，T 是引用类型，无借用概念
//   - nil 表示空值（指针/接口/slice/map/channel/function）
//   - 基本类型：int/float/bool/string，复合：array/slice/map/struct
//   - 隐式转换不存在，必须显式类型转换

// C++:
//   - int x = 5，const int x = 5，类型必须声明
//   - 无 GC，手动内存管理或智能指针（unique_ptr/shared_ptr）
//   - 引用 T& 类似于借用，左值引用
//   - nullptr 表示空指针（C++11）
//   - 基本类型：int/float/double/bool/char，复合：array/struct/class
//   - 隐式类型转换可能发生（需注意安全）

fn compare_rust_lua_python() {
    println!("=== 三语言对比：变量与类型 ===");

    // Rust: 所有权 + 借用
    let s = String::from("hello");
    let t = s; // move
    // println!("{}", s); // ❌ 已移动
    println!("t = {}", t);

    // Lua: 无所有权概念
    // local s = "hello"
    // local t = s  -- 引用，同一个字符串
    // print(s)     -- ✅ 仍然有效
    // Lua 的字符串是不可变的（string immutability），所以 copy 语义

    // Python: 引用传递
    // s = "hello"
    // t = s  # 引用同一个字符串对象
    // print(s)  # ✅ 仍然有效
    // Python 字符串也是不可变的

    // Option vs nil vs None
    let opt: Option<i32> = None;
    if opt.is_none() {
        println!("Rust Option is None");
    }
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 解释为什么 Rust 不用 GC 也能保证内存安全，画图说明所有权模型的工作流程
// 2. 编写一个函数，接收两个 &str，返回较长的字符串（附生命周期标注 'a）
// 3. 将 Option<i32> 的 Some 乘以 2，None 转为 0，写出 match 和 if let 两种写法
// 4. 解释 i32 和 i64 的区别，以及 usize 的作用（结合平台差异说明）
// 5. 实现一个自定义枚举 Status，包含 Ok(String) 和 Err(i32)，并用 match 处理

fn main() {
    println!("=== 模块二：变量与数据类型 ===");

    variable_immutability();
    ownership_demo();
    borrowing_demo();
    lifetime_scope_demo();
    primitive_types();
    compound_types();
    type_conversion();
    option_handling();
    compare_rust_lua_python();

    println!("\n✅ 所有示例运行成功！");
}