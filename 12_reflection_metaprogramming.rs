// ============================================================
// 模块十二：反射与元编程
// 动态创建/注解/动态代码执行
// ============================================================

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

// 【问题1】Rust 的反射能力如何？有没有像 Java 那样的反射？
//
// Rust 的反射能力非常有限，没有 Java 那样的完整反射机制。
// 原因：Rust 设计哲学强调"静态确定"，反射会破坏编译时保证。
//
// 可用能力：
//   - std::any::Any：类型擦除
//   - TypeId：类型标识
//   - std::mem::size_of / type_name：编译期信息
//   - proc_macro：编译时代码生成

fn reflection_basics() {
    // Any trait：类型擦除
    let boxed: Box<dyn Any> = Box::new(42i32);
    if let Some(num) = boxed.downcast_ref::<i32>() {
        println!("downcasted to i32: {}", num);
    }

    // TypeId 获取类型标识
    println!("i32 type id = {:?}", TypeId::of::<i32>());
    println!("String type id = {:?}", TypeId::of::<String>());

    // type_name 获取类型名字（运行时）
    println!("type of 42 = {}", std::any::type_name_of_val(&42));
    println!("type of 'hello' = {}", std::any::type_name_of_val(&"hello"));

    // size_of 获取类型大小
    println!("size of i32 = {}", std::mem::size_of::<i32>());
    println!("size of f64 = {}", std::mem::size_of::<f64>());
    println!("size of () = {}", std::mem::size_of::<()>());

    // size_of_val 获取值的大小
    let arr = [1, 2, 3, 4, 5];
    println!("size of array = {}", std::mem::size_of_val(&arr));
}

// 【问题2】Rust 的 Any trait 如何实现运行时类型信息（RTTI）？
//
// Any trait 提供了 downcast 方法，可以将 Any 转换回具体类型。
// 这是 Rust 能做到的"最接近反射"的能力。

fn any_trait_usage() {
    // 存储不同类型
    let mut objects: Vec<Box<dyn Any>> = vec![];
    objects.push(Box::new(42_i32));
    objects.push(Box::new("hello".to_string()));
    objects.push(Box::new(vec![1, 2, 3]));

    for obj in objects {
        if let Some(i) = obj.downcast_ref::<i32>() {
            println!("i32: {}", i);
        } else if let Some(s) = obj.downcast_ref::<String>() {
            println!("String: {}", s);
        } else if let Some(v) = obj.downcast_ref::<Vec<i32>>() {
            println!("Vec<i32>: {:?}", v);
        }
    }

    // downcast（获取所有权）
    let obj: Box<dyn Any> = Box::new(100_i32);
    let recovered = obj.downcast::<i32>().unwrap();
    println!("recovered: {}", *recovered);

    // 无法 downcast 时处理
    let obj: Box<dyn Any> = Box::new("not an i32");
    match obj.downcast::<i32>() {
        Ok(n) => println!("i32 = {}", n),
        Err(e) => println!("downcast failed: {:?}", e),
    }
}

// 【问题3】Rust 的过程宏（proc_macro）有哪些类型？
//
// Rust 宏的三种类型：
//   1. declarative macro（声明宏）：macro_rules!
//   2. function-like macro（函数宏）：#[macro_export]
//   3. derive macro（派生宏）：#[derive(...)]
//   4. attribute macro（属性宏）：#[...]

fn macro_types() {
    // 声明宏（macro_rules!）
    macro_rules! vec_eq {
        ($($e:expr),*) => {{
            let mut v = Vec::new();
            $(v.push($e);)*
            v
        }};
    }
    let v = vec_eq![1, 2, 3];
    println!("vec_eq: {:?}", v);

    // 更复杂的声明宏
    macro_rules! hash_map {
        ($($k:expr => $v:expr),*) => {{
            let mut map = std::collections::HashMap::new();
            $(map.insert($k, $v);)*
            map
        }};
    }
    let map = hash_map!["a" => 1, "b" => 2];
    println!("hash_map: {:?}", map);

    // 宏调试：用 log_syntax! 查看展开（nightly）
    // #![feature(log_syntax)]

    // derive 宏工作原理
    // #[derive(Debug, Clone)]
    // 编译器生成 Debug::fmt 和 Clone::clone 的实现代码

    println!("声明宏示例已完成");
}

// 【问题4】Rust 的泛型特化与关联类型是什么？
//
// Rust 目前没有完整的特化（specialization），但有部分功能。
// 关联类型在 trait 中定义，impl 时确定具体类型。

fn generics_and_associated_types() {
    // 关联类型示例
    trait Container {
        type Item;
        fn get(&self, idx: usize) -> Option<&Self::Item>;
        fn len(&self) -> usize;
    }

    impl Container for Vec<i32> {
        type Item = i32;
        fn get(&self, idx: usize) -> Option<&Self::Item> { self.get(idx) }
        fn len(&self) -> usize { self.len() }
    }

    let v = vec![1, 2, 3];
    println!("Container<Vec<i32>> len = {}", v.len());
    println!("Container<Vec<i32>> get(1) = {:?}", v.get(1));

    // 泛型参数 vs 关联类型
    // 泛型参数：Container<T>，实现时指定 T
    // 关联类型：Container<Item=...>，实现时确定 Item

    // 泛型方法
    impl<T> Vec<T> {
        fn map<U, F>(&self, f: F) -> Vec<U>
        where F: Fn(&T) -> U
        {
            self.iter().map(f).collect()
        }
    }

    let strings: Vec<String> = v.map(|x| format!("num: {}", x));
    println!("mapped: {:?}", strings);
}

// 【问题5】Rust 的 Zero-Size Types（ZST）和 PhantomData 是什么？
//
// ZST：大小为 0 的类型，如 ()、Never、PhantomData
// PhantomData：标记"类型所有者"，不占用空间但影响生命周期

fn zst_phantomdata() {
    // ZST 示例
    struct NoData;
    println!("size of NoData = {}", std::mem::size_of::<NoData>());

    // 单元结构体
    struct Unit;
    println!("size of Unit = {}", std::mem::size_of::<Unit>());

    // 类型标签
    struct TypedVec<T> {
        data: Vec<i32>,
        _phantom: std::marker::PhantomData<T>,
    }

    let tv: TypedVec<String> = TypedVec {
        data: vec![1, 2, 3],
        _phantom: std::marker::PhantomData,
    };
    println!("TypedVec<String> size = {}", std::mem::size_of::<TypedVec<String>>());
    println!("data size = {}", std::mem::size_of::<Vec<i32>>());

    // PhantomData 用途
    // 1. 标记泛型
    // 2. 避免编译器优化掉"无用"字段
    // 3. 在 unsafe 中模拟引用

    // Never 类型（发散函数）
    fn infinite_loop() -> ! {
        loop { /* 永不返回 */ }
    }

    // ZST 的特殊优化
    let nothing: () = ();
    let _ = nothing;
    println!("ZST is optimized away: ()");
}

// 【问题6】Rust 的 const 泛化（const generics）是什么？
//
// const 泛化：泛型参数可以是常量。
// 例如：Array<T, N>，N 是数组长度。
//
// Rust 1.51+ 支持。

fn const_generics() {
    // 固定大小数组
    fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }

    print_array([1, 2, 3]);
    print_array(['a', 'b', 'c', 'd']);

    // const 泛型约束
    struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T: Copy + Default, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        fn new() -> Self {
            Matrix { data: [[T::default(); COLS]; ROWS] }
        }

        fn get(&self, r: usize, c: usize) -> Option<&T> {
            self.data.get(r).and_then(|row| row.get(c))
        }
    }

    let m: Matrix<i32, 2, 3> = Matrix::new();
    println!("2x3 matrix created");

    // 数值约束
    struct Constrained<T, const N: usize>([T; N])
    where [T; N]: Sized; // 需要这个约束

    println!("const generics 正常工作");
}

// 【问题7】Rust 如何实现运行时动态方法分发？
//
// Rust 没有真正的动态方法分发（类似 Java 的 Method.invoke）。
// 可以通过 trait object + HashMap 模拟。

fn dynamic_dispatch() {
    use std::any::Any;
    use std::collections::HashMap;

    // 注册表模式
    struct CommandRegistry {
        commands: HashMap<String, Box<dyn Fn(String)>>,
    }

    impl CommandRegistry {
        fn new() -> Self {
            CommandRegistry { commands: HashMap::new() }
        }

        fn register<F>(&mut self, name: &str, f: F)
        where F: Fn(String) + 'static {
            self.commands.insert(name.to_string(), Box::new(f));
        }

        fn execute(&self, name: &str, arg: String) -> Option<String> {
            self.commands.get(name).map(|cmd| cmd(arg))
        }
    }

    let mut registry = CommandRegistry::new();
    registry.register("upper", |s| s.to_uppercase());
    registry.register("lower", |s| s.to_lowercase());
    registry.register("reverse", |s| s.chars().rev().collect());

    println!("{}", registry.execute("upper", "hello").unwrap());
    println!("{}", registry.execute("reverse", "rust").unwrap());
}

// 【问题8】Rust 的 instrument/instrument_macro 宏如何使用？
//
// 跟踪宏用于在函数调用时打印日志。
// 可以用 proc_macro 实现，也可以用声明宏。

fn instrumentation_macro() {
    // 简单跟踪宏
    macro_rules! trace {
        ($($arg:tt)*) => {
            println!(concat!("TRACE ", $($arg)*));
        };
    }

    fn my_function(x: i32) -> i32 {
        trace!("my_function called with x = {}", x);
        let result = x * 2;
        trace!("my_function returning {}", result);
        result
    }

    my_function(21);

    // 属性宏示例（伪代码）
    // #[trace_calls]
    // fn my_function(x: i32) { ... }
    // 宏展开后在函数入口出口插入日志代码

    println!("跟踪宏示例完成");
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - 反射能力有限（Any/TypeId）
//   - 强大的宏系统（编译时代码生成）
//   - const 泛型支持数值泛型
//   - PhantomData 处理类型标记
//   - proc_macro 实现编译器扩展

// Python:
//   - 完整反射：type(), getattr, setattr
//   - 装饰器（decorator）是元编程核心
//   - exec/eval 动态执行代码
//   - dataclasses/attrs 自动生成代码

// Lua:
//   - 完整反射：type() 返回类型，getmetatable/setmetatable
//   - 动态执行：loadstring/load
//   - metatable 链实现行为修改
//   - 非常灵活的元编程能力

// Go:
//   - 反射有限：reflect 包
//   - 无宏，通过接口和代码生成（go generate）
//   - 动态执行困难（无 eval）
//   - 标签（struct tag）用于元数据

// C++:
//   - 模板元编程（template metaprogramming）
//   - type_traits 提供类型信息
//   - constexpr 编译时计算
//   - 无运行时间反射（需要手动实现）

fn compare_metaprogramming() {
    println!("=== 三语言元编程对比 ===");

    // Lua 的元编程能力最强（动态语言本质）
    // Python 通过装饰器提供编译时代码生成
    // Rust 通过宏提供编译时代码生成

    // 反射能力
    // Rust: 有限（Any）
    // Python: 完整（type, inspect）
    // Lua: 完整（type, getmetatable）
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 实现一个命令注册表，支持动态注册和执行闭包，通过 HashMap<String, Box<dyn Fn>> 实现
// 2. 用 PhantomData 实现一个"拥有某种类型的所有权"的结构体，解释为什么需要它
// 3. 实现一个 const 泛型 Vec，编译时检查长度不超过固定值
// 4. 解释 derive 宏是如何工作的，编写一个简单的 #[derive(Debug)] 替代品
// 5. 用 Any trait 实现一个类型注册表，根据类型 ID 动态创建对象实例

fn main() {
    println!("=== 模块十二：反射与元编程 ===");

    reflection_basics();
    any_trait_usage();
    macro_types();
    generics_and_associated_types();
    zst_phantomdata();
    const_generics();
    dynamic_dispatch();
    instrumentation_macro();
    compare_metaprogramming();

    println!("\n✅ 所有示例运行成功！");
}