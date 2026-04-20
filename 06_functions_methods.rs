// ============================================================
// 模块六：函数与方法
// 定义/调用/重载/重写/递归/值传递vs引用传递/Lambda/闭包
// ============================================================

// 【问题1】Rust 函数的基本定义与调用机制是什么？
//
// 函数定义：fn 关键字，参数有类型标注，返回值用 -> T
// 函数调用：值传递（Copy 类型）或借用（引用类型）
//
// 编译器视角：函数调用被翻译成 call 指令，
// 参数通过寄存器（SmallInteger）或栈传递，
// 返回值通过寄存器或内存返回。

fn function_basics() {
    // 基本函数
    fn add(a: i32, b: i32) -> i32 { a + b }
    println!("add(3, 4) = {}", add(3, 4));

    // 无返回值函数（返回 ()）
    fn print_sum(a: i32, b: i32) { println!("sum = {}", a + b); }
    print_sum(5, 7);

    // 多返回值（用元组）
    fn min_max(arr: &[i32]) -> (i32, i32) {
        let min = *arr.iter().min().unwrap_or(&0);
        let max = *arr.iter().max().unwrap_or(&0);
        (min, max)
    }
    let (min, max) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min={}, max={}", min, max);

    // 早期返回
    fn abs(n: i32) -> i32 {
        if n < 0 { -n } else { n }
    }
    println!("abs(-5) = {}", abs(-5));

    // 函数作为参数（高阶函数）
    fn apply<F>(f: F, x: i32) -> i32
    where F: Fn(i32) -> i32 {
        f(x)
    }
    let result = apply(|x| x * 2, 21);
    println!("apply double 21 = {}", result);
}

// 【问题2】Rust 支持函数重载吗？多态如何实现？
//
// Rust 不支持同名不同参数类型的重载（无函数重载）。
// 多态通过以下方式实现：
//   - 泛型函数（编译时多态）
//   - Trait 对象（运行时多态）
//   - 枚举变体处理

fn no_overload_but_polymorphism() {
    // 泛型函数：同一函数处理多种类型
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest { largest = item; }
        }
        largest
    }

    let nums = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest i32: {}", largest(&nums));
    println!("largest char: {}", largest(&chars));

    // Trait 约束实现多态
    fn print_debug<T: std::fmt::Debug>(t: &T) {
        println!("{:?}", t);
    }
    print_debug(&42);
    print_debug(&"hello");

    // dyn Trait（运行时多态）
    use std::fmt::Display;
    fn print_any<T: Display>(t: &T) -> &dyn Display { t } // 注意：简化示例
}

// 【问题3】递归函数与尾递归优化是什么？
//
// 递归：函数调用自身。
// 尾递归：递归调用是函数的最后一个操作。
//
// 编译器视角：
//   - 普通递归需要保存栈帧（返回地址、局部变量）
//   - 尾递归可以被优化为跳转到函数开头（Tail Call Optimization）
//   - Rust 不保证尾递归优化，但可以手动优化

fn recursion_and_tail_call() {
    // 普通递归：阶乘
    fn factorial(n: u64) -> u64 {
        if n <= 1 { 1 } else { n * factorial(n - 1) }
    }
    println!("5! = {}", factorial(5)); // 120

    // 尾递归版本：累加器模式
    fn factorial_tail(n: u64) -> u64 {
        fn inner(n: u64, acc: u64) -> u64 {
            if n <= 1 { acc } else { inner(n - 1, n * acc) }
        }
        inner(n, 1)
    }
    println!("5! (tail) = {}", factorial_tail(5));

    // 斐波那契数列
    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
    println!("fib(10) = {}", fibonacci(10));

    // 迭代版本（避免递归栈溢出）
    fn fibonacci_iter(n: u32) -> u32 {
        if n <= 1 { return n; }
        let (mut a, mut b) = (0, 1);
        for _ in 2..=n {
            let next = a + b;
            a = b;
            b = next;
        }
        b
    }
    println!("fib(10) iter = {}", fibonacci_iter(10));

    // 链表递归处理
    #[derive(Debug, Clone)]
    struct Node { val: i32, next: Option<Box<Node>> }

    fn sum_list(head: &Option<Box<Node>>) -> i32 {
        match head {
            None => 0,
            Some(node) => node.val + sum_list(&node.next),
        }
    }

    let list = Some(Box::new(Node {
        val: 1,
        next: Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node { val: 3, next: None })),
        })),
    }));
    println!("sum list = {}", sum_list(&list));
}

// 【问题4】值传递 vs 引用传递在 Rust 中如何区分？
//
// 语义区分：
//   - 值传递（Copy 类型）：复制数据，函数内外独立
//   - 值传递（Move 类型）：移动所有权，函数外变量无效
//   - 引用传递（&T）：借用，不获取所有权
//   - 可变引用（&mut T）：独占借用
//
// 内存视角：
//   - Copy 类型（i32, f64, bool）复制到栈上（廉价）
//   - Move 类型（String, Vec）复制指针，再 free 原位置（所有权转移）
//   - 引用传递传递指针（栈上8或16字节）

fn value_vs_reference() {
    // Copy 类型（i32）
    let a = 5;
    fn consume_i32(x: i32) { println!("consumed i32: {}", x); }
    consume_i32(a);
    println!("a still valid: {}", a); // ✅ i32 是 Copy

    // Move 类型（String）
    let s1 = String::from("hello");
    fn consume_string(s: String) { println!("consumed: {}", s); }
    consume_string(s1);
    // println!("s1 still valid: {}", s1); // ❌ 已移动
    println!("s1 moved, consumer has it");

    // 引用传递
    let s2 = String::from("world");
    fn borrow_string(s: &String) { println!("borrowed: {}", s); }
    borrow_string(&s2);
    println!("s2 still valid: {}", s2); // ✅ 借用

    // 可变引用
    let mut s3 = String::from("before");
    fn mutate_string(s: &mut String) { s.push_str(" + after"); }
    mutate_string(&mut s3);
    println!("mutated: {}", s3);

    // 借用规则验证
    let mut vec = vec![1, 2, 3];
    let first = &vec[0]; // 不可变借用开始
    println!("first = {}", first);
    // vec.push(4); // ❌ 编译错误：同时有不可变和可变借用
    drop(first); // 显式结束借用
    vec.push(4); // 现在可以了
    println!("after push: {:?}", vec);
}

// 【问题5】Rust 的闭包（closure）是什么？
//
// 闭包：可以捕获环境的匿名函数。
// 语法：|params| expr 或 |params| { block }
//
// 闭包类型：
//   - Fn：可以多次调用，不修改环境
//   - FnMut：可变借用环境，可以修改
//   - FnOnce：一次性消费环境
//
// 编译器视角：闭包是匿名结构体，实现了对应的 trait。
// 环境通过捕获的方式（move、immutable borrow、mutable borrow）传递给闭包。

fn closure_mechanics() {
    // 基本闭包
    let add = |a, b| a + b;
    println!("add(3, 4) = {}", add(3, 4));

    // 闭包类型推断
    let square = |x| x * x;
    println!("square(5) = {}", square(5));

    // 捕获环境变量
    let factor = 10;
    let multiply = |x| x * factor;
    println!("multiply(3) = {}", multiply(3));

    // mut 闭包（修改捕获的变量）
    let mut counter = 0;
    let mut increment = || { counter += 1; counter };
    println!("increment() = {}", increment());
    println!("increment() = {}", increment());
    println!("counter = {}", counter);

    // move 闭包（强制获取所有权）
    let data = vec![1, 2, 3];
    let consumer = move || {
        println!("consumed data: {:?}", data);
        // data 在闭包内有效
    };
    consumer();
    // println!("data still valid: {:?}", data); // ❌ 已移动到闭包
    println!("data moved into closure");

    // 闭包作为函数参数
    fn apply<F>(f: F, val: i32) -> i32
    where F: Fn(i32) -> i32 {
        f(val)
    }
    let result = apply(|x| x + 1, 10);
    println!("apply x+1 to 10 = {}", result);

    // FnMut 示例
    fn apply_twice<F>(mut f: F, val: i32) -> i32
    where F: FnMut(i32) -> i32 {
        f(f(val))
    }
    println!("apply_twice x+1 to 0 = {}", apply_twice(|x| x + 1, 0));
}

// 【问题6】Rust 的泛型函数与 Trait 约束是什么？
//
// 泛型允许编写与类型无关的代码。
// Trait 约束（bounds）指定泛型类型必须实现的行为。
//
// 常见约束：
//   - Clone：可克隆
//   - Copy：可复制（隐式）
//   - Debug：可打印调试信息
//   - PartialEq / Eq：可比较相等
//   - PartialOrd / Ord：可排序
//   - Display：可格式化为字符串
//   - Iterator：可迭代

fn generic_functions() {
    // 单约束
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest { largest = item; }
        }
        largest
    }
    println!("largest [3,1,4,1,5,9] = {}", largest(&[3, 1, 4, 1, 5, 9]));
    println!("largest ['a','z','m'] = {}", largest(&['a', 'z', 'm']));

    // 多约束
    fn print_debug_display<T: std::fmt::Debug + std::fmt::Display>(t: &T) {
        println!("Debug: {:?}, Display: {}", t, t);
    }
    print_debug_display(&42);
    print_debug_display(&"hello");

    // where 子句（复杂约束更清晰）
    fn debug_pair<K, V>(pair: &(K, V)) -> String
    where
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        format!("{:?}: {:?}", pair.0, pair.1)
    }
    println!("debug pair: {}", debug_pair(&(1, "one")));

    // 关联类型（Trait 中的类型占位符）
    fn first_from_iter<I>(iter: I) -> Option<I::Item>
    where I: Iterator { iter.next() }
    let result = first_from_iter(vec![1, 2, 3].into_iter());
    println!("first from iterator: {:?}", result);

    // 泛型实现
    struct Stack<T> {
        items: Vec<T>,
    }
    impl<T> Stack<T> {
        fn new() -> Self { Stack { items: Vec::new() } }
        fn push(&mut self, item: T) { self.items.push(item); }
        fn pop(&mut self) -> Option<T> { self.items.pop() }
        fn is_empty(&self) -> bool { self.items.is_empty() }
    }
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    println!("stack pop: {:?}", int_stack.pop());
}

// 【问题7】Rust 方法（method）与关联函数（associated function）的区别？
//
// 方法：有 self 参数的函数，通过 instance.method() 调用。
// 关联函数：没有 self 的函数，通过 Type::function() 调用（如构造函数）。
//
// 编译器视角：方法调用是语法糖，self 参数被编译为隐式参数。

fn methods_and_associates() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 关联函数（类似构造器）
        fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        // 静态方法（不需要实例）
        fn square(size: u32) -> Self {
            Rectangle { width: size, height: size }
        }

        // 方法（有 &self）
        fn area(&self) -> u32 { self.width * self.height }

        // 可变方法（有 &mut self）
        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }

        // 获取引用
        fn width(&self) -> u32 { self.width }

        // 方法重载？不支持，但可以同名关联函数
        fn set_width(&mut self, w: u32) { self.width = w; }
    }

    let mut rect = Rectangle::new(30, 50);
    println!("width={}, height={}", rect.width(), rect.height);
    println!("area={}", rect.area());

    Rectangle::square(10); // 静态调用
    rect.scale(2);
    println!("after scale: {:?}", rect);

    // 链式方法调用
    #[derive(Debug)]
    struct Counter { value: u32 }
    impl Counter {
        fn new() -> Self { Counter { value: 0 } }
        fn increment(&mut self) -> &mut Self { self.value += 1; self }
        fn get(&self) -> u32 { self.value }
    }
    let mut c = Counter::new();
    c.increment().increment().increment();
    println!("counter = {}", c.get());
}

// ============================================================
// 【对比】Rust vs Lua vs Python
// ============================================================
// Rust:
//   - 函数用 fn 定义，支持泛型、Trait 约束
//   - 不支持函数重载（用泛型或 Trait 对象实现多态）
//   - 闭包 |x| expr 是语法糖，实现 Fn/FnMut/FnOnce trait
//   - move 闭包捕获环境变量所有权
//   - 方法用 impl Type {} 定义，第一个参数是 self/&self/&mut self

// Lua:
//   - 函数用 function 关键字定义，语法：function name(params) body end
//   - 函数是一等公民，可以赋值给变量、作为参数传递
//   - 闭包是自然的（函数可以访问外部局部变量）
//   - 没有方法语法糖，所有方法调用都是 table.method(obj, ...) 形式
//   - 不支持泛型，但可以传任意类型参数

// Python:
//   - 函数用 def 定义，lambda 用于匿名函数
//   - 支持函数重载（靠默认参数和 *args 模拟）
//   - 闭包通过 nonlocal 关键字修改外层变量
//   - 方法第一个参数是 self
//   - 装饰器是函数式编程的语法糖

fn compare_functions() {
    println!("=== 三语言函数对比 ===");

    // Rust 闭包 vs Lua 闭包 vs Python lambda
    // Rust: let f = |x| x * 2;
    // Lua:  local f = function(x) return x * 2 end
    // Python: f = lambda x: x * 2

    // Rust 泛型 vs Lua/Python 动态类型
    // Rust: fn largest<T: PartialOrd>(...) 必须约束 T
    // Lua/Python: 直接写函数，类型运行时决定

    // Rust 方法 vs Python 方法
    // Rust: impl T { fn method(&self, ...) } → t.method()
    // Python: def method(self, ...): → t.method()
}

// ============================================================
// 练习题
// ============================================================
// 1. 实现一个泛型函数，交换两个引用的值
// 2. 用闭包实现一个缓存函数（记忆化）
// 3. 实现链表反转（递归和迭代两个版本）
// 4. 解释 Fn vs FnMut vs FnOnce 的区别

fn main() {
    println!("=== 模块六：函数与方法 ===");

    function_basics();
    no_overload_but_polymorphism();
    recursion_and_tail_call();
    value_vs_reference();
    closure_mechanics();
    generic_functions();
    methods_and_associates();
    compare_functions();

    println!("\n✅ 所有示例运行成功！");
}