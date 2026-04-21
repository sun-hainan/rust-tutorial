// ============================================================
// 模块七：面向对象OOP
// 封装/继承/多态/类与对象/修饰符/抽象类/接口/内部类
// ============================================================

// 【问题1】Rust 如何实现封装？pub 和 private 的可见性规则是什么？
//
// Rust 的可见性：
//   - pub：公开
//   - pub(crate)：crate 内可见
//   - pub(super)：父模块可见
//   - 默认（无修饰）：private（本模块内）

fn encapsulation_demo() {
    // 封装在模块中实现
    mod inner {
        pub struct Counter { count: u32 } // pub 对外，但字段 private
        impl Counter {
            pub fn new() -> Self { Counter { count: 0 } }
            pub fn increment(&mut self) { self.count += 1; }
            pub fn get(&self) -> u32 { self.count }
            // private 方法：只能内部调用
            fn private_helper(&self) -> u32 { self.count * 2 }
        }
    }

    let mut c = inner::Counter::new();
    c.increment();
    c.increment();
    println!("counter = {}", c.get());
    // c.count = 10; // ❌ 编译错误：count 是 private
}

// 【问题2】Rust 的结构体与继承——Rust 为什么没有继承？
//
// Rust 没有继承，用组合（composition）替代。
// 设计理念：
//   - 继承导致脆弱的层次结构（LSP 问题）
//   - Rust 用 Trait 实现共享行为
//   - 组合优于继承（"has-a" vs "is-a"）
//
// 编译器视角：结构体没有父结构体的字段，只有自身字段。

fn composition_over_inheritance() {
    // 组合示例：学生"有"名字
    #[derive(Debug)]
    struct Name { first: String, last: String }
    #[derive(Debug)]
    struct Student { name: Name, grade: u8 }

    let student = Student {
        name: Name { first: "张".into(), last: "三".into() },
        grade: 90,
    };
    println!("{:?}", student);

    // 嵌套结构体模拟"继承"
    #[derive(Debug)]
    struct Base { x: i32 }
    #[derive(Debug)]
    struct Extended { base: Base, y: i32 }

    let ext = Extended { base: Base { x: 1 }, y: 2 };
    println!("extended: {:?}", ext);
    println!("base.x = {}", ext.base.x);

    // "委托"模式
    impl Extended {
        fn from_base(base: Base, y: i32) -> Self { Extended { base, y } }
        fn x(&self) -> i32 { self.base.x }
    }
    println!("ext.x() = {}", ext.x());
}

// 【问题3】Rust 的 Trait（类似接口）如何实现多态？
//
// Trait 定义行为规范，实现 Trait 的类型必须提供方法实现。
// 静态分发（泛型）：编译时确定类型
// 动态分发（dyn Trait）：运行时确定类型

fn trait_polymorphism() {
    // 定义 Trait
    trait Drawable {
        fn draw(&self);
        fn area(&self) -> f64; // 必须实现
    }

    // 实现 Trait
    struct Circle { radius: f64 }
    struct Rectangle { width: f64, height: f64 }

    impl Drawable for Circle {
        fn draw(&self) { println!("Circle(r={:.2})", self.radius); }
        fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    }
    impl Drawable for Rectangle {
        fn draw(&self) { println!("Rectangle({:.2}x{:.2})", self.width, self.height); }
        fn area(&self) -> f64 { self.width * self.height }
    }

    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Circle { radius: 2.5 }),
    ];

    for shape in &shapes {
        shape.draw();
        println!("  area = {:.2}", shape.area());
    }

    // 静态分发（泛型，更高效）
    fn print_drawable<D: Drawable>(d: &D) {
        d.draw();
    }
    print_drawable(&Circle { radius: 0.5 });
    print_drawable(&Rectangle { width: 1.0, height: 2.0 });
}

// 【问题4】Rust 的 Trait 对象与静态分发的区别是什么？
//
// 静态分发（泛型）：
//   - 编译时生成专用代码
//   - 无虚表调用开销
//   - 无法存储异构集合（类型必须明确）
//
// 动态分发（dyn Trait）：
//   - 运行时有虚表查找
//   - 可以存储不同类型（通过指针肥东）
//   - Box<dyn Trait> 额外堆分配

fn static_vs_dynamic_dispatch() {
    trait Speak { fn speak(&self) -> String; }

    struct Dog;
    struct Cat;
    struct Cow;

    impl Speak for Dog { fn speak(&self) -> String { "汪汪".into() } }
    impl Speak for Cat { fn speak(&self) -> String { "喵".into() } }
    impl Speak for Cow { fn speak(&self) -> String { "哞".into() } }

    // 动态分发：trait 对象
    let animals: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Cow),
    ];
    for animal in &animals {
        print!("{} ", animal.speak());
    }
    println!();

    // 静态分发：泛型
    fn make_speak_all<T: Speak>(animals: &[T]) -> String {
        let mut result = String::new();
        for animal in animals {
            result.push_str(&animal.speak());
            result.push(' ');
        }
        result
    }

    let dogs = [Dog, Dog];
    println!("静态分发 dogs: {}", make_speak_all(&dogs));

    // 权衡：类型 erasure 的内存成本
    println!("dyn Speak size: {:?}", std::mem::size_of::<Box<dyn Speak>>()); // 16 bytes
    println!("Dog size: {:?}", std::mem::size_of::<Dog>()); // 0 bytes (ZST)
}

// 【问题5】Rust 的抽象类型（default implementation）和 trait bounds 如何使用？
//
// Trait 可以提供默认方法实现。
// trait 可以有关联类型（associated types）和泛型约束。

fn trait_bounds_defaults() {
    trait Printable {
        fn format(&self) -> String;
        // 默认实现
        fn print(&self) { println!("{}", self.format()); }
        fn print_with_prefix(&self, prefix: &str) {
            println!("{}: {}", prefix, self.format());
        }
    }

    #[derive(Debug)]
    struct Point { x: i32, y: i32 }
    impl Printable for Point {
        fn format(&self) -> String { format!("({:?})", self) }
    }

    let p = Point { x: 1, y: 2 };
    p.print();
    p.print_with_prefix("Point");

    // 带泛型约束的方法
    impl Point {
        fn dist_from_origin<T: std::ops::Add<Output = T>>(&self) -> T {
            // 使用泛型约束进行数学运算
            let xx = self.x * self.x;
            let yy = self.y * self.y;
            // 简化示例：返回平方和
            (xx + yy) as i32 // 这里简化处理
        }
    }
    println!("dist squared = {}", p.dist_from_origin());
}

// 【问题6】Rust 的枚举与模式匹配如何替代继承层次？
//
// 枚举可以表示"一个类型可以是多种变体之一"。
// 模式匹配可以处理所有变体（穷尽检查）。

fn enum_pattern_matching() {
    // 枚举表示状态
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    impl Message {
        fn process(&self) {
            match self {
                Message::Quit => println!("退出程序"),
                Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
                Message::Write(text) => println!("写入: {}", text),
                Message::ChangeColor(r, g, b) => println!("颜色 RGB({}, {}, {})", r, g, b),
            }
        }
    }

    Message::Quit.process();
    Message::Move { x: 10, y: 20 }.process();
    Message::Write("Hello".into()).process();
    Message::ChangeColor(255, 128, 0).process();

    // 枚举配合 trait 实现多态
    trait Action { fn execute(&self); }

    enum Command {
        Run(String),
        Stop,
    }

    impl Action for Command {
        fn execute(&self) {
            match self {
                Command::Run(name) => println!("running {}", name),
                Command::Stop => println!("stopping"),
            }
        }
    }

    let cmds: Vec<Box<dyn Action>> = vec![
        Box::new(Command::Run("task1".into())),
        Box::new(Command::Stop),
        Box::new(Command::Run("task2".into())),
    ];
    for cmd in &cmds { cmd.execute(); }
}

// 【问题7】Rust 有内部类吗？嵌套结构如何实现？
//
// Rust 没有内部类，但可以通过模块或嵌套结构实现类似功能。
// 常见模式：
//   - 模块嵌套（namespace）
//   - 结构体嵌套（组合）
//   - 枚举内部嵌套结构

fn nested_structures() {
    // 模块作为"命名空间"
    mod graphics {
        pub mod shapes {
            pub struct Circle { pub radius: f64 }
            impl Circle {
                pub fn new(r: f64) -> Self { Circle { radius: r } }
                pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
            }
        }
        pub mod colors {
            #[derive(Debug)]
            pub struct Color(pub u8, pub u8, pub u8);
            pub const RED: Color = Color(255, 0, 0);
            pub const GREEN: Color = Color(0, 255, 0);
        }
    }

    let c = graphics::shapes::Circle::new(1.0);
    println!("circle area = {:.2}", c.area());
    println!("red = {:?}", graphics::colors::RED);

    // 嵌套结构体（组合）
    #[derive(Debug)]
    struct Outer { pub inner: Inner }
    #[derive(Debug)]
    struct Inner { pub value: i32 }
    let nested = Outer { inner: Inner { value: 42 } };
    println!("nested.inner.value = {}", nested.inner.value);

    // 枚举内部的结构体变体
    #[derive(Debug)]
    enum Tree {
        Leaf(i32),
        Node { left: Box<Tree>, right: Box<Tree> },
    }

    let tree = Tree::Node {
        left: Box::new(Tree::Leaf(1)),
        right: Box::new(Tree::Node {
            left: Box::new(Tree::Leaf(2)),
            right: Box::new(Tree::Leaf(3)),
        }),
    };

    fn sum_tree(t: &Tree) -> i32 {
        match t {
            Tree::Leaf(v) => *v,
            Tree::Node { left, right } => sum_tree(left) + sum_tree(right),
        }
    }
    println!("tree sum = {}", sum_tree(&tree));
}

// 【问题8】Rust 的关联类型（associated types）vs 泛型参数的区别？
//
// 关联类型：在 trait 中用 Item 占位，实现时确定具体类型。
// 泛型参数：在 trait 定义中用泛型参数，实现时外部指定。
//
// 使用场景：
//   - 一个类型只能有一种对应类型 → 关联类型
//   - 同一个类型可以有多种对应类型 → 泛型参数

fn associated_types_demo() {
    // 关联类型：每个实现只能有一种 Item 类型
    trait Container {
        type Item;
        fn get(&self, index: usize) -> Option<&Self::Item>;
        fn len(&self) -> usize;
    }

    impl Container for Vec<i32> {
        type Item = i32;
        fn get(&self, index: usize) -> Option<&Self::Item> { self.get(index) }
        fn len(&self) -> usize { self.len() }
    }

    let v = vec![1, 2, 3];
    println!("v.get(1) = {:?}", v.get(1));

    // 泛型参数：可以有多种实现
    trait ContainerGen<T> {
        fn get(&self, index: usize) -> Option<&T>;
    }

    impl<T> ContainerGen<T> for Vec<T> {
        fn get(&self, index: usize) -> Option<&T> { self.get(index) }
    }
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - 无类和对象，有 struct + impl
//   - 无继承，用 Trait + 组合替代
//   - pub 控制可见性，支持 pub(crate)、pub(super)
//   - Trait 类似接口，支持默认实现
//   - dyn Trait 提供运行时多态（需要 Box）
//   - 枚举 + match 实现状态机

// Python:
//   - 有类（class），支持单继承
//   - 多态通过继承和方法重写实现
//   - 抽象基类（ABC）定义接口
//   - 可见性靠约定（_protected / __private）
//   - mixin 通过多继承实现

// Lua:
//   - 没有类，用 table + metatable 模拟 OOP
//   - 通过 metatable 链实现"继承"（__index）
//   - 封装靠约定（_private 字段）
//   - 没有接口，用约定代替

// Go:
//   - 无类，有 struct + 方法
//   - 无继承，有接口（interface{}）和组合
//   - 首字母大写导出，小写私有
//   - interface 定义行为组合
//   - 无泛型（1.18+ 有泛型）

// C++:
//   - 有类，支持单继承和多继承
//   - 虚函数表实现运行时多态
//   - public/protected/private 控制可见性
//   - 抽象基类定义接口
//   - 模板支持泛型编程

fn compare_oop() {
    println!("=== 三语言OOP对比 ===");

    // Rust 没有继承，用 Trait 做接口
    // Lua 用 metatable 模拟类
    // Python 有 class 和继承

    // 封装：Rust 用可见性，Lua/Python 用约定
    // 多态：Rust 用 Trait 对象，Python 用继承链
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 实现一个 Person 结构体，包含 name/age，实现 Greeting trait，greet() 方法返回问候语
// 2. 用枚举实现一个简易计算器（Add/Sub/Mul/Div），包含 match 处理各变体
// 3. 解释为什么 Rust 推荐组合而非继承，以及组合如何实现"is-a"关系的替代
// 4. 实现一个 Container<T> trait，支持 push/pop/length，用 Vec<T> 实现
// 5. 用 Trait 对象实现一个事件系统，支持注册多个处理函数

fn main() {
    println!("=== 模块七：面向对象OOP ===");

    encapsulation_demo();
    composition_over_inheritance();
    trait_polymorphism();
    static_vs_dynamic_dispatch();
    trait_bounds_defaults();
    enum_pattern_matching();
    nested_structures();
    associated_types_demo();
    compare_oop();

    println!("\n✅ 所有示例运行成功！");
}