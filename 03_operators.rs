// ============================================================
// 模块三：运算符与表达式
// 算术 / 赋值 / 关系 / 逻辑 / 位运算 / 三目 / 优先级
// ============================================================

use std::collections::HashMap;

// 【问题1】Rust 的算术运算符与溢出处理是什么？
//
// Rust 的算术运算符：+ - * / %（加减乘除取模）
//
// 溢出行为（Rust 2018+）：
//   - debug 模式：溢出触发 panic（debug assertions）
//   - release 模式：溢出采用二进制补码截断（wrap）
//
// 显式控制：使用 checked_*, wrapping_*, overflowing_*, saturating_* 方法

fn arithmetic_operators() {
    let a: i32 = 10;
    let b: i32 = 3;

    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {} (整数除法)", a / b);  // 3
    println!("a % b = {}", a % b);             // 1

    // 溢出演示
    let max: u8 = 255;
    // debug 模式下 panic，release 模式下 wrap
    let wrapped = max.wrapping_add(1);
    println!("255 + 1 = {} (wrapping)", wrapped); // 0

    // 显式溢出检查
    let checked = 100i32.checked_add(200);
    println!("checked 100 + 200 = {:?}", checked);

    // 除零处理
    let result = a.div_euclid(b);
    println!("a div_euclid b = {}", result);

    // 取模 vs 取余
    println!("-7 % 3 = {}", -7 % 3);         // -1
    println!("-7 div_euclid 3 = {}", (-7i32).div_euclid(3)); // -2
}

// 【问题2】Rust 的赋值运算符与复合赋值是什么？
//
// Rust 的赋值是表达式（返回 ()），不像 C 那样返回赋的值。
// 复合赋值：+= -= *= /= %= 以及位运算复合赋值

fn assignment_operators() {
    let mut counter = 0;

    // 基本赋值
    counter = 10;
    println!("counter = {}", counter);

    // 复合赋值
    counter += 5;  // 等价于 counter = counter + 5
    println!("counter += 5 → {}", counter);

    counter -= 3;
    println!("counter -= 3 → {}", counter);

    counter *= 2;
    println!("counter *= 2 → {}", counter);

    counter /= 4;
    println!("counter /= 4 → {}", counter);

    // 链式赋值（C 风格返回赋值，Rust 不支持）
    // int x = (y = 5); // C 中 x = 5
    // Rust：赋值返回 ()，无法链式赋值
    let mut x = 0;
    let result = { x = 5; x }; // 需要用块表达式
    println!("result of {{ x = 5; x }} = {}", result);
}

// 【问题3】Rust 的关系运算符与类型要求是什么？
//
// 关系运算符：== != < <= > >=
// 所有基本类型都实现了 PartialEq trait（部分相等）和 Ord trait（排序）
// 自定义类型需要 derive 或手动实现这些 trait

fn relational_operators() {
    // 数值比较
    println!("5 > 3: {}", 5 > 3);
    println!("5 == 5.0: {}", 5 == 5.0f64); // i32 和 f64 可以比较

    // 字符串比较
    let s1 = String::from("hello");
    let s2 = String::from("world");
    println!("s1 == s2: {}", s1 == s2);
    println!("s1 < s2: {}", s1 < s2); // 按字典序

    // 布尔比较
    let b1 = true;
    let b2 = false;
    println!("b1 == b2: {}", b1 == b2);

    // 数组/切片比较（必须元素可比较）
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3];
    let arr3 = [1, 2, 4];
    println!("arr1 == arr2: {}", arr1 == arr2);
    println!("arr1 == arr3: {}", arr1 == arr3);

    // 结构体比较（需要 derive PartialEq）
    #[derive(Debug, PartialEq)]
    struct Point { x: i32, y: i32 }
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 0, y: 0 };
    let p3 = Point { x: 1, y: 0 };
    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);
}

// 【问题4】Rust 的逻辑运算符与短路求值是什么？
//
// && 和 || 是短路求值（short-circuit evaluation）：
//   - &&：左边为 false 时，不计算右边
//   - ||：左边为 true 时，不计算右边
//
// 编译器视角：&& 和 || 被翻译成条件分支（brtrue/brfalse），
// 不是简单的按位运算。

fn logical_operators() {
    let a = true;
    let b = false;

    println!("a && b = {}", a && b); // false
    println!("a || b = {}", a || b); // true
    println!("!a = {}", !a);         // false

    // 短路求值演示
    let get_false = || { println!("evaluating false"); false };
    let get_true = || { println!("evaluating true"); true };

    println!("--- testing && ---");
    if get_true() && get_false() {
        println!("both true");
    } else {
        println!("at least one false");
    }

    println!("--- testing || ---");
    if get_false() || get_true() {
        println!("at least one true");
    }

    // 短路与按位的区别
    // Rust: && 是短路逻辑，& 是按位 AND
    //   - 3 && 5 → true && true = true（短路：左边 true 再看右边）
    //   - 3 & 5  → 0b011 & 0b101 = 0b001 = 1（按位）
    println!("3 && 5 = {}", 3 != 0 && 5 != 0); // true
    println!("3 & 5 = {}", 3 & 5);             // 1

    // 常见用法：空值检查 + 默认值
    let maybe_value = Some(42);
    let result = maybe_value.is_some() && maybe_value.unwrap() > 10;
    println!("value check result = {}", result);
}

// 【问题5】Rust 的位运算符是什么？适用于哪些场景？
//
// 位运算符：& | ^ ~ << >>
// 常用于：
//   - 标志位操作（权限、开关）
//   - 图像处理（像素操作）
//   - 网络协议（掩码、位移）
//   - 性能优化（替代乘法/除法）

fn bitwise_operators() {
    let a: u8 = 0b1100; // 12
    let b: u8 = 0b1010; // 10

    println!("a = {:#06b}, b = {:#06b}", a, b);
    println!("a & b  = {:#06b} ({:3})", a & b, a & b); // 1000 (8)
    println!("a | b  = {:#06b} ({:3})", a | b, a | b); // 1110 (14)
    println!("a ^ b  = {:#06b} ({:3})", a ^ b, a ^ b); // 0110 (6)
    println!("!a     = {:#06b} (~a)", !a);             // 按位取反

    // 位移
    let shifted = 1u8 << 4; // 16
    println!("1 << 4 = {}", shifted);

    // 常用技巧：判断某一位是否为 1
    let flags: u8 = 0b00010101; // 21
    let bit2_is_set = (flags & (1 << 2)) != 0;
    println!("flags bit 2 is set: {}", bit2_is_set);

    // 设置某一位为 1
    let mut flags2 = 0u8;
    flags2 |= (1 << 3); // 设置 bit 3
    println!("after setting bit 3: {:#08b}", flags2);

    // 清除某一位（AND NOT）
    flags2 &= !(1 << 3);
    println!("after clearing bit 3: {:#08b}", flags2);

    // 快速乘除（位移优化）
    let num: u32 = 100;
    println!("num * 2 = {}", num << 1);
    println!("num / 2 = {}", num >> 1);
    println!("num * 8 = {}", num << 3);
    println!("num / 8 = {}", num >> 3);
}

// 【问题6】Rust 的三目运算符与条件表达式是什么？
//
// Rust 没有 ?: 三目运算符，但有 if 表达式（if 是表达式）。
//
// if 表达式必须返回同类型，所有分支必须返回相同类型。

fn conditional_expressions() {
    // if 作为表达式
    let condition = true;
    let value = if condition { 10 } else { 20 };
    println!("value = {}", value);

    // 多分支
    let num = 7;
    let label = if num < 0 {
        "negative"
    } else if num == 0 {
        "zero"
    } else if num % 2 == 0 {
        "even positive"
    } else {
        "odd positive"
    };
    println!("num {} is {}", num, label);

    // 常见模式：默认值
    let config_max = Some(128u32);
    let max = if let Some(m) = config_max { m } else { 0 };
    println!("max = {}", max);

    // match 表达式（更强大的条件分支）
    let grade = 85;
    let result = match grade {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        _        => "F",
    };
    println!("grade {} → {}", grade, result);

    // 赋值表达式（if 表达式的返回值必须类型一致）
    let abs = if num < 0 { -num } else { num };
    println!("abs({}) = {}", num, abs);
}

// 【问题7】Rust 的运算符优先级是什么？
//
// Rust 的优先级（从高到低）：
//   1. ! (逻辑非) ~ (按位取反) * (解引用) & (引用) as (类型转换)
//   2. * / % (算术乘除模)
//   3. + - (算术加减)
//   4. << >> (位移)
//   5. & (按位 AND)
//   6. ^ (按位 XOR)
//   7. | (按位 OR)
//   8. == != < <= > >= (比较)
//   9. && (逻辑 AND)
//   10. || (逻辑 OR)
//   11. .. ..= (范围)
//   12. = += -= *= /= %= &= |= ^= <<= >>= (赋值)
//   13. return / break / 闭包
//
// 建议：复杂表达式使用括号，避免歧义。

fn operator_precedence() {
    let result = 2 + 3 * 4;           // 14（乘法优先级高于加法）
    println!("2 + 3 * 4 = {}", result);

    let result = (2 + 3) * 4;         // 20（括号优先）
    println!("(2 + 3) * 4 = {}", result);

    let a = 5;
    let b = 10;
    let c = 2;

    // 复杂表达式
    let result = a + b * c - 20 / 4 + 1;
    // 计算顺序：b*c=20, 20/4=5, a+20=25, 25-5=20, 20+1=21
    println!("a + b * c - 20 / 4 + 1 = {}", result);

    // 位运算优先级陷阱
    let flags = 0b1001u8;
    let mask = 0b0100u8;
    // & 优先级低于 ==，所以要加括号
    let is_set = (flags & mask) != 0;
    println!("flags & mask != 0 = {}", is_set); // false

    // && 优先级低于 ==
    let cond = 5 == 5 && 3 < 4;
    println!("5 == 5 && 3 < 4 = {}", cond); // true

    // 逻辑非 vs 按位取反
    let x = 0u8;
    println!("!x = {}", !x);        // 逻辑非：true → false → 1（非零）
    println!("~x = {:#010b}", ~x); // 按位取反 → 0b11111111 = 255
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - 算术：+ - * / %（整数除法 5/2=2），溢出分 debug/release 行为
//   - 位运算：& | ^ ! << >>
//   - 逻辑：&& || !（短路求值）
//   - 比较：== != < <= > >=（类型需实现 trait）
//   - 无三目运算符，if 是表达式（要求所有分支同类型）
//   - 赋值不返回值（返回 ()）
//   - 优先级明确，建议用括号

// Python:
//   - 算术：+ - * / // %（地板除法）
//   - 位运算：& | ^ ~ << >>
//   - 逻辑：and or not（关键字）
//   - 比较：== != < <= > >=
//   - 有三目运算符：x if cond else y
//   - False 值：None, False, 0, "", [], {}, set()
//   - and/or 返回实际值（短路求值）

// Lua:
//   - 算术：+ - * / %（浮点除法 5/2=2.5），整数除法用 //
//   - 位运算：& | ~ ^ << >>（Lua 5.3+ 原生支持）
//   - 逻辑：and or not（Lua 用关键字，不是符号）
//   - 比较：== ~= < <= > >=（~= 是不等于）
//   - 无三目运算符
//   - nil 和 false 是假，其他都是真（包括 0 和空字符串）
//   - and/or 可以用作表达式（返回参数本身，不是布尔值）

// Go:
//   - 算术：+ - * / %（整数除法）
//   - 位运算：& | ^ &^ << >>
//   - 逻辑：&& || !（短路求值）
//   - 比较：== != < <= > >=
//   - 无三目运算符（Go 1.13 也没有）
//   - nil 表示空值，false 和 nil 是假，其他都是真

// C++:
//   - 算术：+ - * / %（整数除法）
//   - 位运算：& | ^ ~ << >>
//   - 逻辑：&& || !（短路求值）
//   - 比较：== != < <= > >=
//   - 有三目运算符：condition ? true_val : false_val
//   - 赋值返回赋值结果（可用于链式赋值）
//   - 优先级与 C 相同（较复杂）

fn compare_operators() {
    println!("=== 三语言运算符对比 ===");

    // Rust 整数除法 vs Lua 浮点除法 vs Python 地板除
    let rust_div = 5 / 2;
    // Lua: 5 / 2 = 2.5, 5 // 2 = 2
    // Python: 5 / 2 = 2.5, 5 // 2 = 2

    // 短路求值
    // Rust: && || !
    // Lua: and or not
    // Python: and or not

    // 逻辑非的陷阱
    // Rust: !x（非零 → 0，但 !true = false）
    // Lua: not x（not true = false，not 1 = false，not nil = true）
    // Python: not x（not 1 = False，not 0 = True）
    println!("Rust: !0 = {}", !0u8);
    println!("Python equivalent: not 0 = {}", if 0 { false } else { true });

    // 赋值表达式
    // Rust: let x = 5;（赋值不返回值）
    // Lua: x = 5（赋值返回 nil）
    // Python: x = 5（赋值返回 None）
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 写一个函数，用位移实现乘以 8 和除以 8（不用 * 和 / 运算符）
// 2. 实现一个 is_power_of_two 函数，用位运算判断一个 u32 是否是 2 的幂
// 3. 比较 0.1 + 0.2 和 0.3 的相等性，解释浮点数精度问题及其在 Rust 中的处理方式
// 4. 用 match 实现一个简易计算器（加/减/乘/除），输入 "3 + 4" 输出 7
// 5. 解释 Rust 中 && 和 & 的区别，以及 || 和 | 的区别，并给出实际场景示例

fn main() {
    println!("=== 模块三：运算符与表达式 ===");

    arithmetic_operators();
    assignment_operators();
    relational_operators();
    logical_operators();
    bitwise_operators();
    conditional_expressions();
    operator_precedence();
    compare_operators();

    println!("\n✅ 所有示例运行成功！");
}