# Rust Tutorial

Comprehensive Rust programming tutorial with problem-driven approach, deep principles, and complete system coverage.

## 17 Modules

| # | 文件 | 标题 |
|---|------|------|
| 1 | `01_代码规范与基础.md` | 注释/缩进/标识符/关键字 |
| 2 | `02_变量与数据类型.md` | 所有权/借用/生命周期/基本类型 |
| 3 | `03_运算符与表达式.md` | 算术/位运算/短路求值/优先级 |
| 4 | `04_流程控制.md` | if/match/loop/Result/迭代器链 |
| 5 | `05_字符串与数组.md` | String/&str/Vec/排序/二分 |
| 6 | `06_函数与方法.md` | 闭包/泛型/Trait约束/递归 |
| 7 | `07_面向对象.md` | Trait/组合/静态vs动态分发 |
| 8 | `08_集合框架.md` | HashMap/HashSet/VecDeque/Heap |
| 9 | `09_IO流与文件.md` | File/BufReader/serde/路径 |
| 10 | `10_异常处理与调试.md` | panic/Result/unwind/测试 |
| 11 | `11_并发编程.md` | thread/Mutex/channel/Send/Sync |
| 12 | `12_反射与元编程.md` | Any/宏/PhantomData/const泛型 |
| 13 | `13_网络编程.md` | TCP/UDP/Socket/HTTP |
| 14 | `14_数据结构与算法.md` | 链表/树/排序/查找/图 |
| 15 | `15_数据库.md` | SQL/事务/rusqlite/SQLx |
| 16 | `16_工程化与设计.md` | Builder/RAII/Cargo/CI |
| 17 | `17_信息分发与事件系统.md` | Observer/EventEmitter/Actor |

## 特色

- **问题驱动**：每节 5-8 个核心问题，围绕问题展开
- **深入原理**：从表象到本质，剖析语言设计思想
- **完整体系**：覆盖 Rust 核心知识点
- **五语对比**：与 Python/Lua/Go/C++ 对比
- **完整示例**：每个概念都有可运行的代码
- **配套练习**：每章 4-5 道实践题目

## 学习路线

```
阶段一：入门（模块 1-8）
  模块1: 语法基础 → 模块2: 所有权（核心）→ 模块3-5: 语法要素
  模块6: 函数/闭包 → 模块7: Trait → 模块8: 集合

阶段二：进阶（模块 9-12）
  模块9: IO → 模块10: 错误处理 → 模块11: 并发 → 模块12: 宏

阶段三：应用（模块 13-17）
  模块13: 网络 → 模块14: 数据结构 → 模块15: 数据库
  模块16: 工程化 → 模块17: 事件系统
```

## 源码

原始 `.rs` 源文件位于 `_src/` 目录，对应 Markdown 教程在同级目录。

## License

MIT License
