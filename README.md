# Rust Tutorial

Comprehensive Rust programming tutorial with problem-driven approach, deep principles, and complete system coverage.

## 17 Modules

| Module | File | Core Content |
|--------|------|-------------|
| 模块一 | 01_code_basics.rs | 代码规范与基础（注释/缩进/标识符/关键字） |
| 模块二 | 02_variables_types.rs | 变量与数据类型（变量/常量/作用域/生命周期/基本类型/引用类型/类型转换/空值/进制） |
| 模块三 | 03_operators.rs | 运算符与表达式（算术/赋值/关系/逻辑/位运算/三目/优先级） |
| 模块四 | 04_control_flow.rs | 流程控制（if/else/switch/match/循环/break/continue/异常） |
| 模块五 | 05_strings_arrays.rs | 字符串与数组（字符串操作/数组增删改查/排序/二分查找） |
| 模块六 | 06_functions_methods.rs | 函数/方法（定义/调用/重载/重写/递归/值传递vs引用传递/Lambda/闭包） |
| 模块七 | 07_oop.rs | 面向对象OOP（封装/继承/多态/类与对象/修饰符/抽象类/接口/内部类） |
| 模块八 | 08_collections.rs | 集合框架（List/Set/Map/泛型/工具类） |
| 模块九 | 09_io_file.rs | IO流与文件（路径/字节流/字符流/缓冲流/序列化） |
| 模块十 | 10_exceptions_debug.rs | 异常处理与调试（异常分类/自定义/断点/日志） |
| 模块十一 | 11_concurrency.rs | 并发编程（线程/锁/同步/线程池/死锁） |
| 模块十二 | 12_reflection_metaprogramming.rs | 反射与元编程（动态创建/注解/动态代码执行） |
| 模块十三 | 13_networking.rs | 网络编程（TCP/UDP/Socket/HTTP） |
| 模块十四 | 14_dsa.rs | 数据结构与算法（链表/栈/队列/哈希表/树/排序/查找/复杂度） |
| 模块十五 | 15_database.rs | 数据库编程（SQL/事务/ORM） |
| 模块十六 | 16_engineering_design.rs | 工程化与设计思想（设计模式/代码优化/Git） |
| 模块十七 | 17_event_system.rs | 信息分发与事件系统（观察者/EventEmitter/信号槽/消息队列/中间件） |

## Features

- **问题驱动**：每节 5-8 个核心问题，围绕问题展开讲解
- **深入原理**：从表象到本质，剖析语言设计思想
- **完整体系**：覆盖 Rust 核心知识点
- **五语对比**：与 Python/Lua/Go/C++ 对比，加深理解
- **完整示例**：每个概念都有可运行的代码示例
- **配套练习**：每章 4-5 道实践题目，巩固所学

## 适用人群

| 人群 | 适用理由 |
|------|----------|
| **系统程序员** | Rust 是 C/C++ 的现代替代品，适合操作系统、网络协议、嵌入式等底层开发 |
| **Web 后端开发者** | WebAssembly、异步框架（Axum/Tokio）让 Rust 成为高性能后端选择 |
| **游戏开发者** | 通过 WGPU/WGPU 进行图形渲染，Rust 也被用于 Unity 和 Unreal 的脚本扩展 |
| **数据工程师/科学家** | 性能关键的数据处理、机器学习推理、并行计算 |
| **DevOps/基础设施** | Kubernetes 周边工具、Docker 替代品、CLI 工具 |
| **安全研究员** | 内存安全 + 零成本抽象，适合安全工具和漏洞利用开发 |
| **已有其他语言经验者** | 想深入理解内存管理、类型系统、并发模型 |

**不推荐**：仅作为脚本语言替代 Python/Bash；没有 C/C++ 背景且不想理解底层原理。

## Rust 语言特色总结

### 1. 所有权系统（Ownership）

Rust 的核心创新，通过编译时检查消除：
- **数据竞争**：同一时刻只有一个可变引用
- **use-after-free**：引用必须始终有效
- **双重释放**：所有者离开作用域时自动释放

```rust
let s1 = String::from("hello");
let s2 = s1;           // 所有权从 s1 移动到 s2
// println!("{}", s1); // ❌ 编译错误：s1 已移动
println!("{}", s2);    // ✅
```

### 2. 生命周期（Lifetime）

编译器追踪引用的有效范围，确保：
- 无悬垂引用（引用指向的内存仍然有效）
- 引用的生命周期不会超过它们引用的数据

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

### 3. 零成本抽象（Zero-Cost Abstractions）

高级特性不引入运行时开销：
- **泛型**：编译时单态化，无动态分发
- **迭代器**：链式调用，编译器优化为手写循环
- **闭包**：实现为结构体，无额外分配
- **Trait**：可静态分发（泛型）或动态分发（dyn Trait）

### 4. 类型系统

- **代数数据类型**：枚举 + match 实现穷尽检查
- **模式匹配**：解构复杂数据结构
- **Result/Option**：强制错误处理，无异常
- **Send/Sync**：编译时保证线程安全

### 5. 工具链

| 工具 | 用途 |
|------|------|
| `cargo` | 包管理、构建、测试 |
| `rustfmt` | 代码格式化 |
| `clippy` | Lint 检查 |
| `rustdoc` | 文档生成 |
| `miri` | 未定义行为检测 |
| `cargo bench` | 性能基准测试 |

## 学习路线图

```
阶段一：入门（第 1-8 章）
├── 模块一：代码规范与基础
│   └── 学习目标：理解 Rust 语法风格、注释、标识符规则
├── 模块二：变量与数据类型
│   └── 学习目标：掌握所有权、借用、生命周期（核心）
├── 模块三：运算符与表达式
│   └── 学习目标：熟悉 Rust 运算符和表达式特性
├── 模块四：流程控制
│   └── 学习目标：掌握 if/match/loop，理解穷尽检查
├── 模块五：字符串与数组
│   └── 学习目标：理解 String/&str、Vec/[T;N] 的区别
├── 模块六：函数与方法
│   └── 学习目标：掌握函数定义、闭包、泛型
├── 模块七：面向对象
│   └── 学习目标：理解 struct/Trait/impl vs class
└── 模块八：集合框架
    └── 学习目标：掌握 Vec/HashMap/HashSet 等集合

阶段二：进阶（第 9-12 章）
├── 模块九：IO流与文件
│   └── 学习目标：File/BufReader/serde 序列化
├── 模块十：异常处理与调试
│   └── 学习目标：Result/panic/调试技术
├── 模块十一：并发编程
│   └── 学习目标：线程/锁/channel/Send/Sync
└── 模块十二：反射与元编程
    └── 学习目标：宏系统/const 泛型/Any trait

阶段三：应用（第 13-17 章）
├── 模块十三：网络编程
│   └── 学习目标：TCP/UDP/HTTP + async 简介
├── 模块十四：数据结构与算法
│   └── 学习目标：手写常见数据结构
├── 模块十五：数据库编程
│   └── 学习目标：SQL/Rusqlite/Diesel/SQLx
├── 模块十六：工程化
│   └── 学习目标：设计模式/测试/Cargo workspace
└── 模块十七：事件系统
    └── 学习目标：观察者/EventEmitter/Actor

阶段四：项目实践
├── Web 服务（Axum + SQLx）
├── CLI 工具（clap）
├── WebAssembly 组件
└── 嵌入式系统（no_std）
```

### 学习建议

1. **循序渐进**：不要跳过所有权和生命周期，这是 Rust 的核心
2. **动手实践**：每章练习题必须亲手实现，不要只看答案
3. **编译错误是朋友**：前几个月的 Rust 编程就是和编译器"搏斗"，它是严格的老师
4. **善用工具**：`rust-analyzer`（IDE 插件）和 `rustlings`（交互练习）非常有帮助
5. **阅读标准库源码**：理解惯用写法，参考 `std::` 和 `crates.io` 的 crate

## Running

```bash
# Compile and run
rustc 01_code_basics.rs -o 01_code_basics
./01_code_basics

# Or with Cargo
cargo run --bin 01_code_basics
```

## Structure

Each file follows the same structure:
1. 核心问题 - 本章要解决的关键问题
2. 问题解答 - 直接回答核心问题
3. 深入原理 - 剖析底层的实现机制
4. 对比学习 - 与其他语言的对比
5. 完整示例 - 可运行的代码示例
6. 练习题 - 巩固所学知识

## License

MIT License