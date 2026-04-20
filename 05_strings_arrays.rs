// ============================================================
// 模块五：字符串与数组
// 字符串操作 / 数组增删改查 / 排序 / 二分查找
// ============================================================

// 【问题1】Rust 的 String 和 &str 有什么区别？
//
// &str 是字符串切片（指向 UTF-8 字节序列的引用），不可变视图。
// String 是堆上分配的可变字符串，拥有所有权。
//
// 内存视角：
//   - &str：栈上 16 字节（ptr + len），指向堆或常量区的数据
//   - String：栈上 24 字节（ptr + len + capacity），堆上存储实际数据
//
// 编译器视角：&str 实现了 Deref<Target=str>，可以自动转换。

fn string_types() {
    // &str（字符串字面量，存储在二进制常量区）
    let s1: &str = "hello"; // 不可变引用，指向常量区
    println!("s1 = {}, len = {}", s1, s1.len());

    // String（堆上分配，可变）
    let s2 = String::from("hello");
    let s3 = String::new();
    let s4 = "hello".to_string();

    println!("String::from: {}", s2);
    println!("to_string: {}", s4);

    // 转换：String → &str（廉价借用）
    take_str(&s2);
    fn take_str(s: &str) { println!("got &str: {}", s); }

    // 转换：&str → String（分配内存）
    let s5: String = "hello".to_string();
    println!("&str to String: {}", s5);

    // String 的容量（capacity）
    let mut s6 = String::with_capacity(10);
    s6.push_str("hello");
    println!("s6 len={} cap={}", s6.len(), s6.capacity());
}

// 【问题2】Rust 字符串的常用操作有哪些？
//
// Rust 字符串操作：
//   - push/pop（末尾添加/删除字符）
//   - insert/remove（指定位置插入/删除）
//   - split/at（访问子串/字符）
//   - trim/replace（修剪/替换）
//   - contains/starts_with/ends_with（查找）

fn string_operations() {
    let mut s = String::from("hello world");

    // 追加
    s.push('!');              // 末尾加字符
    s.push_str(" goodbye");   // 末尾加字符串
    println!("push: {}", s);

    // 插入
    s.insert(5, ',');         // 在位置5插入
    println!("insert: {}", s);

    // 删除
    s.pop();                  // 删最后一个字符
    println!("pop: {}", s);

    // 替换
    let replaced = s.replace("world", "rust");
    println!("replace: {}", replaced);

    // 分割
    let parts: Vec<&str> = s.split_whitespace().collect();
    println!("split: {:?}", parts);

    // 按分隔符分割
    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("split by comma: {:?}", fruits);

    // 取子串（切片）
    let s2 = "你好世界";
    let sub = &s2[0..3]; // 取第一个汉字（3字节）
    println!("sub: {}", sub);

    // 大小写
    println!("upper: {}", "HeLLo".to_uppercase());
    println!("lower: {}", "HeLLo".to_lowercase());

    // trim
    let padded = "   hello   ";
    println!("trim: '{}'", padded.trim());

    // 查找
    let text = "hello rust, hello world";
    println!("contains 'rust': {}", text.contains("rust"));
    println!("starts_with 'hello': {}", text.starts_with("hello"));
    println!("find 'world': {:?}", text.find("world"));

    // 编码长度（Unicode）
    let emoji = "😀";
    println!("emoji len bytes = {}", emoji.len()); // 4
    println!("emoji char count = {}", emoji.chars().count()); // 1
}

// 【问题3】Rust 的数组（array）和向量（Vec）的区别是什么？
//
// 数组 [T; N]：
//   - 长度固定，编译时确定
//   - 存储在栈上（如果元素是 Copy）
//   - 长度类型是 usize（编译期常量）
//   - 不会动态增长
//
// Vec<T>（向量）：
//   - 动态数组，长度可变化
//   - 堆上分配，栈上只存元数据（ptr, len, capacity）
//   - 可用 push/pop 操作

fn array_and_vec() {
    // 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}", arr);
    println!("len = {}, size = {} bytes", arr.len(), std::mem::size_of_val(&arr));

    // 默认值初始化
    let zeros = [0; 10]; // 10 个 0
    println!("zeros len = {}", zeros.len());

    // 索引访问（越界会 panic）
    println!("arr[0] = {}", arr[0]);
    println!("arr[4] = {}", arr[4]);

    // 切片
    let slice = &arr[1..4];
    println!("slice [1..4]: {:?}", slice);

    // Vec（向量）
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vec: {:?}", vec);

    // Vec::from
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("vec! macro: {:?}", vec2);

    // 动态操作
    vec.push(6);
    println!("after push: {:?}", vec);
    let last = vec.pop();
    println!("pop: {:?}, vec now: {:?}", last, vec);

    // insert/remove
    vec.insert(0, 0);
    println!("after insert 0 at 0: {:?}", vec);
    let removed = vec.remove(0);
    println!("removed {}: {:?}", removed, vec);

    // 容量
    let mut vec3 = Vec::with_capacity(10);
    vec3.push(1);
    println!("vec3 len={} cap={}", vec3.len(), vec3.capacity());

    // 安全访问：get 返回 Option
    println!("vec.get(0) = {:?}", vec.get(0));
    println!("vec.get(100) = {:?}", vec.get(100));
}

// 【问题4】Rust 的数组排序和查找如何实现？
//
// Rust 标准库提供：
//   - sort / sort_by（原地排序，稳定/不稳定）
//   - sort_unstable_by（更快的非稳定排序）
//   - binary_search（二分查找）
//
// 编译器视角：sort 使用 TimSort（稳定）或 IP（introsort，不稳定）

fn sorting_searching() {
    let mut nums = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    println!("排序前: {:?}", nums);

    // 原地排序
    nums.sort();
    println!("排序后: {:?}", nums);

    // 降序
    nums.sort_by(|a, b| b.cmp(a));
    println!("降序: {:?}", nums);

    // 字符串排序
    let mut names = vec!["Charlie", "Alice", "Bob", "David"];
    names.sort();
    println!("字符串排序: {:?}", names);

    // 按长度排序
    let mut words = vec!["long", "short", "medium", "tiny"];
    words.sort_by_key(|w| w.len());
    println!("按长度排序: {:?}", words);

    // 二分查找（必须先排序）
    let sorted = vec![1, 3, 5, 7, 9, 11, 13, 15];
    match sorted.binary_search(&7) {
        Ok(idx) => println!("找到 7 在索引 {}", idx),
        Err(idx) => println!("未找到，插入位置 {}", idx),
    }

    // 查找范围
    let idx = sorted.partition_point(|&x| x < 7);
    println!("partition_point for 7: idx = {}", idx);

    // 自定义类型排序
    #[derive(Debug, Clone)]
    struct Person { name: String, age: u32 }
    let mut people = vec![
        Person { name: "Alice".into(), age: 30 },
        Person { name: "Bob".into(), age: 25 },
        Person { name: "Charlie".into(), age: 35 },
    ];
    people.sort_by_key(|p| p.age);
    println!("按年龄排序: {:?}", people);

    // stable_sort（保持相等元素的相对顺序）
    let mut pairs = vec![(3, "a"), (1, "b"), (3, "c"), (2, "d")];
    pairs.sort_by_key(|p| p.0);
    println!("stable sort: {:?}", pairs); // (1,"b"), (2,"d"), (3,"a"), (3,"c")
}

// 【问题5】Rust 的多维数组和字符串转换如何处理？
//
// Rust 的多维数组通过嵌套 Vec 或固定数组实现。
// 字符串转换用 parse 和 format。

fn multidimensional_conversion() {
    // 固定二维数组
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("matrix: {:?}", matrix);
    for row in &matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }

    // 动态二维向量（行列数可变）
    let mut grid: Vec<Vec<i32>> = Vec::new();
    grid.push(vec![1, 2, 3]);
    grid.push(vec![4, 5]);
    grid.push(vec![6, 7, 8, 9]);
    println!("jagged grid: {:?}", grid);

    // 字符串到数值的转换
    let s = "42";
    let n: i32 = s.parse().unwrap();
    println!("parse '{}' = {}", s, n);

    // 数值到字符串
    let n2 = 255;
    let s2 = n2.to_string();
    println!("to_string {} = '{}'", n2, s2);

    // 带进制转换
    let hex_str = "FF";
    let hex_val = u32::from_str_radix(hex_str, 16).unwrap();
    println!("from_str_radix '{}' = {}", hex_str, hex_val);

    // format!（模板字符串）
    let formatted = format!("name={}, age={}", "Alice", 30);
    println!("format: {}", formatted);

    // String 转 &str 再转回 String
    let s3 = "hello".to_string();
    let r: &str = &s3;
    let s4 = r.to_string();
    println!("String -> &str -> String: {}", s4);

    // split_once（分割一次）
    let kv = "key=value";
    if let Some((k, v)) = kv.split_once('=') {
        println!("key='{}', value='{}'", k, v);
    }
}

// 【问题6】Rust 的字符串模式匹配和正则表达式如何用？
//
// 标准库提供 starts_with/ends_with/contains/find 等基础模式匹配。
// 复杂正则表达式需要 regex crate。

fn pattern_matching() {
    let text = "The quick brown fox jumps over the lazy dog";

    // 基础查找
    if text.contains("fox") {
        println!("found 'fox'");
    }

    // 查找位置
    if let Some(idx) = text.find("the") {
        println!("'the' starts at index {}", idx);
    }

    // 替换
    let replaced = text.replace("fox", "cat");
    println!("replace: {}", replaced);

    // 多次替换
    let normalized = text.replace("The", "A").replace("the", "a");
    println!("normalize: {}", normalized);

    // trim 边界空白
    let with_spaces = "   hello   ";
    println!("trim: '{}'", with_spaces.trim());

    // 分割
    let parts: Vec<&str> = "a,b,c".split(',').collect();
    println!("split: {:?}", parts);

    // 保留分隔符
    let with_delim: Vec<&str> = "a,b,c".split_inclusive(',').collect();
    println!("split_inclusive: {:?}", with_delim);

    // rsplit（反向分割）
    let path = "/home/user/documents/file.txt";
    let components: Vec<&str> = path.rsplit('/').collect();
    println!("rsplit: {:?}", components);

    // split_at（按位置切分）
    let (left, right) = path.split_at(5);
    println!("split_at 5: '{}' | '{}'", left, right);

    // lines（按行分割）
    let multiline = "line1\nline2\nline3";
    for line in multiline.lines() {
        println!("line: {}", line);
    }
}

// ============================================================
// 【对比】Rust vs Lua vs Python
// ============================================================
// Rust:
//   - String（堆，可变）和 &str（切片引用）分离
//   - Vec<T> 是动态数组，数组是固定长度 [T; N]
//   - 字符串是 UTF-8 编码，索引按字节而非字符
//   - sort 是原地排序，binary_search 需要已排序数组
//   - 标准库提供基础模式，正则需要 regex crate

// Lua:
//   - 字符串是不可变字节序列，支持模式匹配（Lua 自己的模式）
//   - table 是 Lua 的数组/映射，可以作为动态数组使用
//   - 没有内置 sort，但 table.sort 可以排序数组部分
//   - string.sub 按字节索引，不是 Unicode 字符
//   - 字符串拼接用 ..（性能考虑用 table.concat）

// Python:
//   - str 是 Unicode 不可变字符串
//   - list 是动态数组，支持增删改查
//   - sorted() 返回新数组，list.sort() 原地排序
//   - bisect 模块提供二分查找
//   - re 模块提供正则表达式

fn compare_string_array() {
    println!("=== 三语言字符串数组对比 ===");

    // Rust 字符串索引陷阱
    let s = "你好";
    println!("Rust: '你好' len={}, chars={}", s.len(), s.chars().count());
    // Python: len('你好') = 2（字符数）
    // Lua: #"你好" = 6（字节数）

    // Rust 字符串切片（按字节）
    let greeting = "hello world";
    println!("greeting[0..5] = '{}'", &greeting[0..5]);
    // Python: greeting[0:5] = 'hello'
    // Lua: greeting:sub(1, 5) = 'hello'
}

// ============================================================
// 练习题
// ============================================================
// 1. 实现一个函数，统计字符串中单词数量（空格分隔）
// 2. 实现字符串反转函数（处理 Unicode）
// 3. 实现一个函数，对 Vec<i32> 进行快速排序（手写，不调用 sort）
// 4. 写一个函数，实现二分查找（返回索引或插入位置）

fn main() {
    println!("=== 模块五：字符串与数组 ===");

    string_types();
    string_operations();
    array_and_vec();
    sorting_searching();
    multidimensional_conversion();
    pattern_matching();
    compare_string_array();

    println!("\n✅ 所有示例运行成功！");
}