// ============================================================
// 模块四：流程控制
// if/else/switch/match/循环/break/continue/异常
// ============================================================

// 【问题1】Rust 的 if/else 与 if 表达式是什么关系？
//
// if/else 在 Rust 中是表达式，可以返回值。
// 所有分支必须返回相同类型。
//
// 编译器视角：if 表达式被翻译成条件跳转指令（brif），
// 两个分支最终汇合到同一个 phi 节点（SSA 形式）。

fn if_else_expressions() {
    let number = 7;

    // 基本 if/else
    if number % 2 == 0 {
        println!("{} 是偶数", number);
    } else {
        println!("{} 是奇数", number);
    }

    // if 表达式（必须所有分支类型一致）
    let label = if number % 2 == 0 { "even" } else { "odd" };
    println!("number is {}", label);

    // 多分支
    let score = 85;
    let grade = if score >= 90 {
        "A".to_string()
    } else if score >= 80 {
        "B".to_string()
    } else if score >= 70 {
        "C".to_string()
    } else {
        "D".to_string()
    };
    println!("score {} → grade {}", score, grade);

    // 条件赋值
    let max = if 3 > 5 { 3 } else { 5 };
    println!("max = {}", max);

    // 常见模式：空值 + 默认值
    let maybe_config = Some(128u32);
    let value = if let Some(v) = maybe_config { v } else { 0 };
    println!("config value = {}", value);
}

// 【问题2】Rust 的 match 表达式比 switch 强大在哪里？
//
// Rust 没有 switch，而是用 match（模式匹配）。
// 相比 C/Java switch，match 强大得多：
//   - 匹配字面量、范围、枚举、结构体、元组
//   - 穷尽检查（exhaustiveness checking）——编译器确保所有情况被覆盖
//   - 每个分支可以有绑定（destructuring）
//   - 分支_guard（if 条件）

fn match_expressions() {
    let num = 3;

    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"), // 通配符，覆盖所有未匹配情况
    }

    // match 作为表达式
    let description = match num {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("num {} is {}", num, description);

    // 范围匹配
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        _        => "F",
    };
    println!("score {} → grade {}", score, grade);

    // 匹配枚举
    enum Direction {
        North,
        South,
        East,
        West,
    }

    fn get_direction_label(dir: Direction) -> &'static str {
        match dir {
            Direction::North => "北",
            Direction::South => "南",
            Direction::East  => "东",
            Direction::West  => "西",
        }
    }

    let dir = Direction::North;
    println!("方向是 {}", get_direction_label(dir));

    // 匹配多个值（用 |）
    let c = 'a';
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("{} 是元音", c),
        'a'..='z' => println!("{} 是辅音", c),
        _ => println!("其他字符"),
    }

    // 匹配时绑定变量
    let msg = Some(42);
    match msg {
        Some(n) if n % 2 == 0 => println!("偶数: {}", n),
        Some(n) if n % 2 != 0 => println!("奇数: {}", n),
        None => println!("空值"),
        _ => println!("其他"),
    }

    // 穷尽检查示例（故意去掉 default 会报错）
    enum Color { Red, Green, Blue }
    let color = Color::Red;
    match color {
        Color::Red   => println!("红色"),
        Color::Green => println!("绿色"),
        Color::Blue  => println!("蓝色"),
        // 如果注释掉任何分支，编译器会报错：non-exhaustive patterns
    }
}

// 【问题3】Rust 的循环有哪些？while / loop / for 各自的应用场景？
//
// Rust 三种循环：
//   - loop：无限循环，需手动 break
//   - while：条件循环
//   - for：迭代循环（最常用）

fn loop_types() {
    // loop（无限循环，外部控制）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // loop 可以有返回值（break 表达式）
        }
    };
    println!("loop result = {}", result);

    // while 循环
    let mut n = 0;
    while n < 5 {
        print!("{} ", n);
        n += 1;
    }
    println!();

    // for 循环（最常用，用于迭代器）
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // for 遍历集合
    let arr = [10, 20, 30, 40, 50];
    for (index, value) in arr.iter().enumerate() {
        println!("arr[{}] = {}", index, value);
    }

    // 反向迭代
    for i in (0..5).rev() {
        print!("{} ", i);
    }
    println!();

    // for 遍历 HashMap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    scores.insert("Carol", 92);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // for 循环中的break和continue
    let mut sum = 0;
    for i in 0..20 {
        if i % 3 == 0 {
            continue; // 跳过 3 的倍数
        }
        if i > 15 {
            break; // 超过 15 停止
        }
        sum += i;
    }
    println!("sum of 1..15 except multiples of 3 = {}", sum);

    // 嵌套循环中的 break（带标签）
    'outer: for i in 1..4 {
        for j in 1..4 {
            if i == 2 && j == 2 {
                println!("break at i={} j={}", i, j);
                break 'outer; // 跳出外层循环
            }
        }
    }
}

// 【问题4】Rust 的 break 和 continue 与标签（label）是什么？
//
// break 可以：
//   - 退出当前循环
//   - 退出指定标签的外层循环（通过 'label 语法）
//   - 从 loop 中返回值
//
// continue：
//   - 跳过本次循环剩余代码，进入下一次迭代
//
// 编译器视角：break 编译成 jump 指令跳到循环结束标签，
// continue 跳到循环条件检查处。

fn break_continue_labels() {
    // 基本 break
    let mut count = 0;
    loop {
        count += 1;
        if count >= 5 {
            break;
        }
    }
    println!("basic break: count = {}", count);

    // break 带值（loop 表达式）
    let result = loop {
        break 42; // 立即退出并返回值
    };
    println!("break with value: {}", result);

    // continue 跳过本次
    for i in 0..6 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        print!("{} ", i);
    }
    println!();

    // 标签用于多层循环
    let mut matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    'find: for row in 0..3 {
        for col in 0..3 {
            if matrix[row][col] == 5 {
                println!("found 5 at ({}, {})", row, col);
                break 'find; // 跳出外层循环
            }
        }
    }

    // 多层 continue
    'outer2: for i in 1..3 {
        for j in 1..3 {
            for k in 1..3 {
                if k == 2 {
                    continue 'outer2; // 跳到外层循环的下一次
                }
                print!("({}, {}, {}) ", i, j, k);
            }
        }
    }
    println!();
}

// 【问题5】Rust 如何处理异常？panic / Result / Option 的区别？
//
// Rust 的错误处理哲学：
//   - panic：不可恢复的错误（程序 Bug，如数组越界）
//   - Result<T, E>：可恢复的错误（文件不存在、网络超时）
//   - Option<T>：表示"可能存在或不存在"的值
//
// 机器视角：
//   - panic 会展开栈（unwinding）或中止（aborting），调用 drop
//   - Result 是枚举，成功返回 Ok(T)，失败返回 Err(E)

fn error_handling() {
    // panic 场景演示
    // let v = vec![1, 2, 3];
    // v.get(10); // 返回 None，不会 panic
    // v[10];     // 越界访问，直接 panic！

    // Result 基本用法
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    let result = divide(10, 2);
    match result {
        Ok(v)  => println!("10 / 2 = {}", v),
        Err(e) => println!("错误: {}", e),
    }

    let result2 = divide(10, 0);
    if let Err(e) = result2 {
        println!("捕获错误: {}", e);
    }

    // ? 运算符（早期返回）
    fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let num: i32 = s.trim().parse::<i32>()?;
        Ok(num * 2)
    }

    match parse_and_double("42") {
        Ok(v)  => println!("parse_and_double('42') = {}", v),
        Err(e) => println!("解析失败: {}", e),
    }

    // unwrap / expect（仅用于确定不会失败时）
    let arr = vec![1, 2, 3];
    let first = arr.get(0).unwrap(); // 已知数组非空
    println!("first element = {}", first);

    // unwrap_or / unwrap_or_else
    let empty: Vec<i32> = vec![];
    let first_empty = empty.get(0).copied().unwrap_or(-1);
    println!("first of empty = {}", first_empty);

    // 自定义错误类型
    #[derive(Debug)]
    enum MyError {
        InvalidInput(String),
        OutOfRange(i32),
    }

    fn validate_age(age: i32) -> Result<i32, MyError> {
        if age < 0 {
            Err(MyError::InvalidInput("年龄不能为负".to_string()))
        } else if age > 150 {
            Err(MyError::OutOfRange(age))
        } else {
            Ok(age)
        }
    }

    match validate_age(25) {
        Ok(a)  => println!("valid age: {}", a),
        Err(e) => println!("error: {:?}", e),
    }
}

// 【问题6】Rust 的循环返回值与 map/filter/fold 组合是什么？
//
// Rust 的 for 循环没有"返回值"，但可以通过收集（collect）获取结果。
// 迭代器链（Iterator）是函数式编程风格。
//
// 编译器视角：迭代器被lazy求值（lazy evaluation），
// 只有 collect 或 for 消费时才会真正执行。

fn iterator_pipeline() {
    // 计算 1..100 中 3 的倍数的和
    let sum: i32 = (1..100)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("1..100 中 3 的倍数的和 = {}", sum);

    // 找出第一个符合条件的元素
    let first = (1..)
        .filter(|x| x * x > 1000)
        .find(|x| *x % 7 == 0);
    println!("第一个平方>1000且被7整除的数 = {:?}", first);

    // 链式操作
    let words = vec!["hello", "rust", "world", "code"];
    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 4)
        .map(|w| w.to_uppercase())
        .collect();
    println!("长度>4的词转为大写: {:?}", result);

    // fold（折叠）
    let product = (1..=5).fold(1, |acc, x| acc * x);
    println!("1*2*3*4*5 = {}", product); // 120

    // zip（组合两个迭代器）
    let names = vec!["Alice", "Bob", "Carol"];
    let scores = vec![100, 85, 92];
    let pairs: Vec<(&str, i32)> = names.iter().zip(scores.iter()).collect();
    for (name, score) in &pairs {
        println!("{}: {}", name, score);
    }
}

// 【问题7】Rust 的 if let 和 while let 是什么？
//
// if let：简化单分支 match
// while let：循环处理 Option 值直到 None
//
// 编译器视角：两者都是 match 的语法糖

fn if_let_while_let() {
    // if let 简化
    let maybe_value = Some(42);
    // 传统写法：
    match maybe_value {
        Some(v) => println!("got {}", v),
        None => {},
    }
    // if let 简化：
    if let Some(v) = maybe_value {
        println!("if let: got {}", v);
    }

    // while let 循环
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // if let 绑定 + 条件
    let msg = Some(42);
    if let Some(n) = msg {
        if n > 10 {
            println!("n = {} > 10", n);
        }
    }

    // 复杂模式匹配
    let point = (0, 5);
    if let (0, y) = point {
        println!("点在对角线上，y = {}", y);
    }
}

// ============================================================
// 【对比】Rust vs Lua vs Python
// ============================================================
// Rust:
//   - if/else 是表达式，可以返回值，所有分支类型必须一致
//   - match 是模式匹配，支持字面量/范围/枚举/结构体/守卫
//   - loop/while/for 三种循环，break 可返回值，continue 跳过本次
//   - 标签语法 'label 支持多层循环控制
//   - panic/Result/Option 分离了不可恢复错误和可恢复错误
//   - ? 运算符简化错误传播

// Lua:
//   - if/elseif/else 是语句，不是表达式（不能直接返回值）
//   - 没有 match，用 if-elseif-else 链代替
//   - while/repeat/for 三种循环，没有 C 风格的 for 循环
//   - repeat...until 是后条件循环（至少执行一次）
//   - 没有 break/continue，用 goto 或包装函数代替
//   - 错误处理用 pcall/xpcall + 手动返回值
//   - 不用 {} 缩进表示代码块，用 end 关键字

// Python:
//   - if/elif/else 是语句，不是表达式
//   - 没有 match（3.10+ 有 match-case，结构类似）
//   - while/for 循环，for 遍历可迭代对象
//   - 有 break/continue，标签循环用嵌套函数或 itertools
//   - 异常用 try/except/finally/raise
//   - 有三目运算符：x if cond else y

fn compare_control_flow() {
    println!("=== 三语言流程控制对比 ===");

    // Rust 的 match 穷尽检查
    // Rust: 编译器确保所有枚举变体被匹配，否则报错
    // Lua: 手动写所有分支，没有穷尽检查
    // Python: match-case 也有穷尽检查（Python 3.10+）

    // 循环中的所有权问题
    // Rust: for 循环中借用集合，修改需要 &mut
    // Lua: 循环中直接修改表（安全的，但要注意迭代器失效）
    // Python: for 循环直接遍历引用，修改列表会影响迭代
}

// ============================================================
// 练习题
// ============================================================
// 1. 用 match 实现一个简易计算器（输入 "3 + 4" 输出 7）
// 2. 实现一个函数，将 Option<i32> 转换为 Result<i32, String>
// 3. 用迭代器链找出 100 以内所有既是 3 又是 5 的倍数的数
// 4. 解释为什么 Rust 的 match 要穷尽检查，这和 Java switch 有什么区别

fn main() {
    println!("=== 模块四：流程控制 ===");

    if_else_expressions();
    match_expressions();
    loop_types();
    break_continue_labels();
    error_handling();
    iterator_pipeline();
    if_let_while_let();
    compare_control_flow();

    println!("\n✅ 所有示例运行成功！");
}