// ============================================================
// 模块八：集合框架
// List/Set/Map/泛型/工具类
// ============================================================

use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap, LinkedList};

// 【问题1】Rust 的 Vec<T>（动态数组）与数组 [T; N] 的区别是什么？
//
// [T; N]：固定长度，存储在栈或静态区
// Vec<T>：动态数组，堆上分配，栈上只存元数据（ptr, len, capacity）
//
// 内存视角：
//   - 数组：编译时确定大小，可能栈溢出
//   - Vec：运行时分配，容量不够时 realloc（可能移动）

fn vec_vs_array() {
    // 固定数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array len = {}", arr.len());

    // 动态数组
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("vec len = {}", vec.len());

    // from 数组切片
    let vec_from_arr: Vec<i32> = arr.to_vec();
    println!("from array: {:?}", vec_from_arr);

    // with_capacity 预分配
    let mut vec2 = Vec::with_capacity(100);
    println!("vec2 cap = {}", vec2.capacity());
    vec2.push(1); // 不需要重新分配

    // reallocate 演示
    let mut growing = Vec::new();
    let mut caps = Vec::new();
    for i in 0..10 {
        growing.push(i);
        caps.push(growing.capacity());
    }
    println!("capacity growth: {:?}", caps);
}

// 【问题2】Rust 的 HashMap<K, V> 如何高效查找？
//
// HashMap 使用哈希表实现 O(1) 平均查找/插入/删除。
// 冲突处理：链地址法（seperate chaining）或开放地址法。
//
// 内存视角：桶数组 + 链表，负载因子超过阈值时 rehash 扩容。

fn hashmap_usage() {
    let mut scores = HashMap::new();

    // 插入
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    scores.insert("Carol", 92);

    // 查找
    match scores.get("Alice") {
        Some(score) => println!("Alice's score = {}", score),
        None => println!("Alice not found"),
    }

    // get with default
    let unknown = scores.get("David").copied().unwrap_or(0);
    println!("David's score (default 0) = {}", unknown);

    // 更新
    scores.entry("Alice").and_modify(|e| *e += 10);
    println!("Alice after update = {:?}", scores.get("Alice"));

    // or_insert（插入或获取）
    let counter = scores.entry("Charlie").or_insert(0);
    *counter += 1;
    println!("Charlie = {:?}", scores.get("Charlie"));

    // 删除
    scores.remove("Bob");
    println!("after remove Bob: {:?}", scores);

    // 遍历
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // from 数组/元组切片
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    println!("from pairs: {:?}", map);

    // 使用自定义类型作为 key
    #[derive(Debug, Hash, Eq, PartialEq)]
    struct Point { x: i32, y: i32 }

    let mut point_map = HashMap::new();
    point_map.insert(Point { x: 0, y: 0 }, "origin");
    point_map.insert(Point { x: 1, y: 1 }, "diagonal");
    println!("point_map = {:?}", point_map);
}

// 【问题3】Rust 的 HashSet<T> 如何保证唯一性？
//
// HashSet 是 HashMap<T, ()> 的封装，保证元素唯一。
// 支持集合运算：union, intersection, difference。

fn hashset_usage() {
    let mut set = HashSet::new();

    // 插入
    set.insert("apple");
    set.insert("banana");
    set.insert("apple"); // 重复，不会添加

    println!("set size = {}", set.len());
    println!("contains apple: {}", set.contains("apple"));

    // 删除
    set.remove("banana");

    // 遍历
    for item in &set {
        print!("{} ", item);
    }
    println!();

    // 集合运算
    let a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    println!("a ∪ b = {:?}", a.union(&b).collect::<HashSet<_>>());
    println!("a ∩ b = {:?}", a.intersection(&b).collect::<HashSet<_>>());
    println!("a - b = {:?}", a.difference(&b).collect::<HashSet<_>>());
    println!("b - a = {:?}", b.difference(&a).collect::<HashSet<_>>());

    // 判断子集
    let subset: HashSet<i32> = [2, 3].iter().cloned().collect();
    println!("subset ⊆ a: {}", subset.is_subset(&a));

    // 去重
    let nums = vec![1, 2, 2, 3, 3, 3, 4, 4, 5];
    let unique: HashSet<i32> = nums.into_iter().collect();
    println!("unique from [1,2,2,3,3,3,4,4,5]: {:?}", unique);
}

// 【问题4】Rust 的 VecDeque<T>（双端队列）适合什么场景？
//
// VecDeque：两端都可以 O(1) 插入/删除的队列。
// 适合场景：
//   - BFS 遍历
//   - 滑动窗口
//   - 生产者-消费者模式
//
// 内存视角：循环缓冲区实现，首尾相连。

fn deque_usage() {
    let mut deque = VecDeque::new();

    // 从后端入队
    deque.push_back(1);
    deque.push_back(2);
    // 从前端入队
    deque.push_front(0);
    deque.push_front(-1);

    println!("deque: {:?}", deque);

    // 从前后端出队
    let front = deque.pop_front();
    let back = deque.pop_back();
    println!("pop_front = {:?}, pop_back = {:?}", front, back);
    println!("deque now: {:?}", deque);

    // 队列操作
    while let Some(item) = deque.pop_front() {
        print!("{} ", item);
    }
    println!();

    // VecDeque 用于 BFS
    let mut queue = VecDeque::new();
    queue.push_back((0, 0)); // 起点
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push_back((nx, ny));
            }
        }
    }
    println!("visited count: {}", visited.len());
}

// 【问题5】Rust 的 BinaryHeap<T>（二叉堆）如何实现优先队列？
//
// BinaryHeap 是最大堆，根节点是最大元素。
// 用于：Dijkstra 最短路、哈夫曼编码、Top-K 问题。

fn binary_heap_usage() {
    let mut heap = BinaryHeap::new();

    // 入堆
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    println!("heap size = {}", heap.len());
    println!("peek max = {:?}", heap.peek());

    // 出堆（最大优先）
    while let Some(top) = heap.pop() {
        print!("{} ", top);
    }
    println!();

    // 自定义排序（通过 reverse）
    let mut min_heap: BinaryHeap<std::cmp::Reverse<i32>> = BinaryHeap::new();
    min_heap.push(std::cmp::Reverse(3));
    min_heap.push(std::cmp::Reverse(1));
    min_heap.push(std::cmp::Reverse(4));
    min_heap.push(std::cmp::Reverse(1));

    println!("min heap order:");
    while let Some(std::cmp::Reverse(v)) = min_heap.pop() {
        print!("{} ", v);
    }
    println!();

    // Top-K 问题
    fn top_k<T: Clone + PartialOrd>(items: Vec<T>, k: usize) -> Vec<T> {
        let mut heap: BinaryHeap<T> = items.into_iter().collect();
        (0..k)
            .filter_map(|_| heap.pop())
            .collect()
    }
    let numbers = vec![1, 5, 2, 8, 3, 9, 1, 6];
    println!("top 3: {:?}", top_k(numbers, 3));
}

// 【问题6】Rust 的 LinkedList<T>（链表）的特点和适用场景是什么？
//
// 双向链表：
//   - 优点：O(1) 头尾插入/删除
//   - 缺点：随机访问 O(n)，缓存不友好
//
// 适用场景：需要频繁在任意位置插入/删除，或者需要稳定的迭代器。

fn linkedlist_usage() {
    let mut list = LinkedList::new();

    // 头尾操作
    list.push_front(1);
    list.push_back(2);
    list.push_front(0);
    list.push_back(3);

    println!("list: {:?}", list);

    // pop
    let front = list.pop_front();
    let back = list.pop_back();
    println!("pop_front = {:?}, pop_back = {:?}", front, back);
    println!("list now: {:?}", list);

    // 遍历
    for v in &list {
        print!("{} ", v);
    }
    println!();

    // 迭代器修改
    let mut list2: LinkedList<i32> = [1, 2, 3].iter().cloned().collect();
    for v in list2.iter_mut() {
        *v *= 2;
    }
    println!("doubled: {:?}", list2);

    // 使用迭代器进行分割
    let mut list3: LinkedList<i32> = (1..=5).collect();
    let second_half: LinkedList<i32> = list3.split_off(2);
    println!("first: {:?}, second: {:?}", list3, second_half);
}

// 【问题7】Rust 的 BTreeMap/TreeSet 是什么？与 HashMap/HashSet 的区别？
//
// BTreeMap：基于 B 树的有序映射，按 key 排序。
// TreeSet：基于 BTreeMap 的有序集合。
//
// 适用场景：
//   - 需要有序遍历
//   - 范围查询
//   - 内存效率（比 Hash 更省空间）
//
// 复杂度：O(log n) 查找/插入/删除（比 Hash 的 O(1) 慢，但有序）

fn btree_usage() {
    use std::collections::BTreeMap;

    let mut btree = BTreeMap::new();
    btree.insert(3, "three");
    btree.insert(1, "one");
    btree.insert(4, "four");
    btree.insert(1, "ONE"); // 更新

    // 有序遍历
    for (k, v) in &btree {
        println!("{} -> {}", k, v);
    }

    // 范围查询
    println!("range 1..=3:");
    for (k, v) in btree.range(1..=3) {
        print!("{} ", k);
    }
    println!();

    // BTreeSet
    use std::collections::BTreeSet;
    let mut bset = BTreeSet::new();
    bset.insert(3);
    bset.insert(1);
    bset.insert(4);
    bset.insert(1);

    println!("BTreeSet sorted: {:?}", bset);

    // 找到最小/最大的元素
    println!("first = {:?}", bset.first());
    println!("last = {:?}", bset.last());

    // split 分割
    let (low, high) = bset.split(&3);
    println!("< 3: {:?}, >= 3: {:?}", low, high);
}

// 【问题8】Rust 的泛型工具函数 collect / Iterator 方法有哪些？
//
// collect：将迭代器转换为集合类型（Vec, HashMap, HashSet 等）。
// 常用方法：map, filter, fold, reduce, zip, enumerate, chain

fn iterator_collectors() {
    // Vec 收集
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("squares: {:?}", squares);

    // HashMap 收集
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    let word_counts: std::collections::HashMap<_, _> =
        words.iter().fold(
            std::collections::HashMap::new(),
            |mut acc, w| { *acc.entry(w).or_insert(0) += 1; acc }
        );
    println!("word counts: {:?}", word_counts);

    // HashSet 收集（去重）
    let unique: std::collections::HashSet<_> = (1..=10)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("even numbers: {:?}", unique);

    // 元组收集
    let pairs: Vec<(i32, i32)> = (1..=3)
        .flat_map(|i| std::iter::once((i, i * i)))
        .collect();
    println!("pairs: {:?}", pairs);

    // String 收集
    let concatenated: String = ["hello", " ", "world"].iter().collect();
    println!("concatenated: {}", concatenated);

    // Option 收集
    let results = vec![Ok(1), Ok(2), Err(()), Ok(4)];
    let successful: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
    println!("successful: {:?}", successful);

    // partition（分割）
    let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=10).partition(|x| x % 2 == 0);
    println!("evens: {:?}, odds: {:?}", evens, odds);
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - Vec<T>: 动态数组，push/pop O(1)
//   - HashMap<K, V>: 哈希映射，O(1) 平均
//   - HashSet<T>: 哈希集合（HashMap<T, ()>）
//   - VecDeque<T>: 双端队列
//   - BinaryHeap<T>: 二叉堆（最大堆）
//   - LinkedList<T>: 双向链表
//   - BTreeMap/TreeSet: 有序映射/集合
//   - 标准库统一通过 Iterator trait 支持链式调用

// Python:
//   - list: 动态数组，append O(1)，中间插入 O(n)
//   - dict: 哈希映射，O(1) 平均
//   - set: 哈希集合
//   - collections.deque: 双端队列
//   - heapq: 堆（需要手动用 list 实现）
//   - collections.OrderedDict: 有序字典（Python 3.7+ 保持插入顺序）

// Lua:
//   - table: 唯一的复合类型，可作数组/映射/集合
//   - 无 HashMap，但 table 可以模拟
//   - 无 Set，用 table.insert(t, value) + 遍历去重
//   - 数组部分用数字索引，映射部分用字符串索引

// Go:
//   - slice: 动态数组（头指针+长度+容量）
//   - map: 哈希映射
//   - struct: 结构体（组合而非继承）
//   - list: 无内置双向链表
//   - container/list: 双向链表
//   - 排序用 sort 包

// C++:
//   - std::vector: 动态数组
//   - std::unordered_map: 哈希映射
//   - std::unordered_set: 哈希集合
//   - std::deque: 双端队列
//   - std::priority_queue: 堆（基于 binary_heap）
//   - std::list: 双向链表
//   - std::map/std::set: 有序红黑树

fn compare_collections() {
    println!("=== 三语言集合对比 ===");

    // Rust HashMap vs Python dict
    // 两者都是哈希表，性能类似
    // Rust 需要声明类型，Python 是动态的

    // 迭代器链式调用
    // Rust: iter.map().filter().collect()
    // Python: filter(lambda x: x > 0, map(lambda x: x*2, data))
    // Lua: 需要用 for 循环手动实现

    // HashMap 键要求
    // Rust: 需要 Hash + Eq trait
    // Python: 需要可哈希（自定义类要实现 __hash__ 和 __eq__）
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 实现一个词频统计程序，统计给定文本中每个单词出现的频率（用 HashMap）
// 2. 用 VecDeque 实现一个滑动窗口最大值函数，输入 [1,3,-1,-3,5,3,6,7] 和 k=3，输出 [3,3,5,5,6,7]
// 3. 实现一个 LRU 缓存（用 HashMap + LinkedList 或 VecDeque），包含 get 和 put 方法
// 4. 用 BTreeMap 实现一个区间合并函数，合并重叠的区间 [[1,3],[2,6],[8,10],[15,18]] → [[1,6],[8,10],[15,18]]
// 5. 对比 Vec 和 LinkedList 的性能，说明各自适合的场景

fn main() {
    println!("=== 模块八：集合框架 ===");

    vec_vs_array();
    hashmap_usage();
    hashset_usage();
    deque_usage();
    binary_heap_usage();
    linkedlist_usage();
    btree_usage();
    iterator_collectors();
    compare_collections();

    println!("\n✅ 所有示例运行成功！");
}