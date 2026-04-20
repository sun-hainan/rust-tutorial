// ============================================================
// 模块十四：数据结构与算法
// 链表/栈/队列/哈希表/树/排序/查找/复杂度
// ============================================================

// 【问题1】Rust 如何手写链表？单向链表 vs 双向链表？
//
// 单向链表：next 指针
// 双向链表：prev + next
//
// Rust Option + Box 实现空值安全和递归所有权的封装

fn linked_list() {
    // 单向链表节点
    #[derive(Debug)]
    struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        fn new(value: T) -> Self { Node { value, next: None } }

        fn push(&mut self, value: T) {
            match self.next.take() {
                Some(next) => {
                    next.push(value);
                    self.next = Some(next);
                }
                None => self.next = Some(Box::new(Node::new(value))),
            }
        }
    }

    #[derive(Debug)]
    struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
        size: usize,
    }

    impl<T> LinkedList<T> {
        fn new() -> Self { LinkedList { head: None, size: 0 } }

        fn push(&mut self, value: T) {
            let new_node = Box::new(Node { value, next: self.head.take() });
            self.head = Some(new_node);
            self.size += 1;
        }

        fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                self.size -= 1;
                node.value
            })
        }

        fn len(&self) -> usize { self.size }
        fn is_empty(&self) -> bool { self.size == 0 }
    }

    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("list: {:?}", list);

    while let Some(v) = list.pop() {
        print!("{} ", v);
    }
    println!();

    // 反转链表（迭代）
    fn reverse_list(head: Option<Box<Node<i32>>>) -> Option<Box<Node<i32>>> {
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    // 构建测试链表
    fn from_vec(v: &[i32]) -> Option<Box<Node<i32>>> {
        let mut result = None;
        for &x in v.iter().rev() {
            result = Some(Box::new(Node { value: x, next: result }));
        }
        result
    }

    fn to_vec(head: &Option<Box<Node<i32>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut current = head.as_ref();
        while let Some(node) = current {
            result.push(node.value);
            current = node.next.as_ref();
        }
        result
    }

    let original = from_vec(&[1, 2, 3, 4, 5]);
    println!("original: {:?}", to_vec(&original));
    let reversed = reverse_list(original);
    println!("reversed: {:?}", to_vec(&reversed));
}

// 【问题2】Rust 如何实现栈和队列？
//
// 栈：后进先出（LIFO）
// 队列：先进先出（FIFO）
// 双向队列：两端都可操作

fn stack_queue() {
    // 栈（用 Vec 模拟）
    let mut stack: Vec<i32> = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("stack top = {}", stack.last().unwrap());
    println!("stack pop = {:?}", stack.pop());
    println!("stack after pop: {:?}", stack);

    // 队列（用 VecDeque）
    use std::collections::VecDeque;
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    println!("queue front = {:?}", queue.pop_front());
    println!("queue after pop: {:?}", queue);

    // 栈的链式实现
    struct Stack<T> {
        top: Option<Box<StackNode<T>>>,
    }

    #[derive(Debug)]
    struct StackNode<T> {
        value: T,
        next: Option<Box<StackNode<T>>>,
    }

    impl<T> Stack<T> {
        fn new() -> Self { Stack { top: None } }
        fn push(&mut self, v: T) {
            self.top = Some(Box::new(StackNode {
                value: v,
                next: self.top.take(),
            }));
        }
        fn pop(&mut self) -> Option<T> {
            self.top.take().map(|node| {
                self.top = node.next;
                node.value
            })
        }
        fn is_empty(&self) -> bool { self.top.is_none() }
    }

    let mut s = Stack::new();
    s.push(10);
    s.push(20);
    s.push(30);
    while let Some(v) = s.pop() { print!("{} ", v); }
    println!();

    // 括号匹配（栈应用）
    fn is_valid_parentheses(s: &str) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => { if stack.pop() != Some('(') { return false; } }
                ']' => { if stack.pop() != Some('[') { return false; } }
                '}' => { if stack.pop() != Some('{') { return false; } }
                _ => {}
            }
        }
        stack.is_empty()
    }

    println!("'()[]{{}}' valid = {}", is_valid_parentheses("()[]{}"));
    println!("'([)]' valid = {}", is_valid_parentheses("([)]"));
}

// 【问题3】Rust 的树结构（二叉树、BST、堆）如何实现？
//
// 二叉树：left/right 子节点
// BST：左小右大
// 堆：完全二叉树，max-heap 或 min-heap

fn tree_structures() {
    // 二叉树节点
    #[derive(Debug)]
    struct TreeNode<T> {
        value: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    }

    impl<T> TreeNode<T> {
        fn new(v: T) -> Box<Self> { Box::new(TreeNode { value: v, left: None, right: None }) }
    }

    // BST 实现
    #[derive(Debug)]
    struct BST<T: std::cmp::Ord> {
        root: Option<Box<TreeNode<T>>>,
    }

    impl<T: std::cmp::Ord> BST<T> {
        fn new() -> Self { BST { root: None } }

        fn insert(&mut self, v: T) {
            match self.root.take() {
                None => self.root = Some(TreeNode::new(v)),
                Some(mut node) => {
                    if v < node.value {
                        node.left = Self::insert_into(node.left, v);
                    } else if v > node.value {
                        node.right = Self::insert_into(node.right, v);
                    }
                    self.root = Some(node);
                }
            }
        }

        fn insert_into(node: Option<Box<TreeNode<T>>>, v: T) -> Option<Box<TreeNode<T>>> {
            match node {
                None => Some(TreeNode::new(v)),
                Some(mut n) => {
                    if v < n.value { n.left = Self::insert_into(n.left.take(), v); }
                    else if v > n.value { n.right = Self::insert_into(n.right.take(), v); }
                    Some(n)
                }
            }
        }

        fn search(&self, v: &T) -> bool {
            let mut current = self.root.as_deref();
            while let Some(node) = current {
                if v < &node.value { current = node.left.as_deref(); }
                else if v > &node.value { current = node.right.as_deref(); }
                else { return true; }
            }
            false
        }

        fn inorder(&self) {
            Self::inorder_recursive(self.root.as_deref());
            println!();
        }

        fn inorder_recursive(node: Option<&Box<TreeNode<T>>>) {
            if let Some(n) = node {
                Self::inorder_recursive(n.left.as_deref());
                print!("{} ", n.value);
                Self::inorder_recursive(n.right.as_deref());
            }
        }
    }

    let mut bst = BST::new();
    for v in [5, 3, 7, 1, 4, 6, 8] { bst.insert(v); }
    println!("BST inorder:");
    bst.inorder();
    println!("search 4 = {}", bst.search(&4));
    println!("search 9 = {}", bst.search(&9));

    // 二叉树层序遍历（BFS）
    fn level_order(root: Option<&Box<TreeNode<i32>>>) {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        if let Some(r) = root { queue.push_back(*r); }

        while let Some(node) = queue.pop_front() {
            print!("{} ", node.value);
            if let Some(left) = node.left { queue.push_back(*left); }
            if let Some(right) = node.right { queue.push_back(*right); }
        }
        println!();
    }
    level_order(bst.root.as_deref());
}

// 【问题4】Rust 的排序算法如何实现？归并排序 vs 快速排序？
//
// 手写排序实现：
//   - 冒泡：O(n²)
//   - 选择：O(n²)
//   - 插入：O(n²)
//   - 归并：O(n log n)，稳定
//   - 快速：O(n log n)，不稳定

fn sorting_algorithms() {
    // 冒泡排序
    fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let n = arr.len();
        for i in 0..n {
            for j in 0..n - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    // 插入排序
    fn insertion_sort<T: Ord>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    // 归并排序
    fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
        if arr.len() <= 1 { return arr.to_vec(); }
        let mid = arr.len() / 2;
        let left = merge_sort(&arr[..mid]);
        let right = merge_sort(&arr[mid..]);
        merge(&left, &right)
    }

    fn merge<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(a.len() + b.len());
        let mut i = 0; let mut j = 0;
        while i < a.len() && j < b.len() {
            if a[i] <= b[j] { result.push(a[i].clone()); i += 1; }
            else { result.push(b[j].clone()); j += 1; }
        }
        result.extend_from_slice(&a[i..]);
        result.extend_from_slice(&b[j..]);
        result
    }

    // 快速排序
    fn quick_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
        if arr.len() <= 1 { return arr.to_vec(); }
        let pivot = arr[0].clone();
        let (left, right) = arr[1:].partition(|x| x < &pivot);
        let mut result = quick_sort(&left);
        result.push(pivot);
        result.extend(quick_sort(&right));
        result
    }

    let mut data = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut data);
    println!("bubble sort: {:?}", data);

    let data = vec![64, 34, 25, 12, 22, 11, 90];
    println!("merge sort: {:?}", merge_sort(&data));

    let data = vec![64, 34, 25, 12, 22, 11, 90];
    println!("quick sort: {:?}", quick_sort(&data));
}

// 【问题5】Rust 的查找算法——二分查找如何实现？
//
// 二分查找：O(log n)，要求已排序数组
// 变体：查找上界、下界、插入位置

fn binary_search() {
    // 标准二分查找
    fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < *target { left = mid + 1; }
            else if arr[mid] > *target { right = mid; }
            else { return Some(mid); }
        }
        None
    }

    let sorted = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!("binary_search 7: {:?}", binary_search(&sorted, &7));
    println!("binary_search 8: {:?}", binary_search(&sorted, &8));

    // 查找插入位置
    fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < *target { left = mid + 1; }
            else { right = mid; }
        }
        left
    }

    let arr = [1, 3, 5, 7, 9];
    println!("lower_bound of 4: {}", lower_bound(&arr, &4)); // 2
    println!("lower_bound of 5: {}", lower_bound(&arr, &5)); // 2

    // 查找第一个 >= target 的位置（partition_point）
    use std::collections::VecDeque;
    fn partition_point<T: Ord>(arr: &[T], target: &T) -> usize {
        arr.iter().position(|x| x >= target).unwrap_or(arr.len())
    }
    println!("partition_point for 6: {}", partition_point(&arr, &6)); // 3
}

// 【问题6】Rust 的哈希表（HashMap）内部原理是什么？
//
// 哈希表：
//   - 哈希函数：将 key 映射到桶索引
//   - 冲突处理：链地址法（separate chaining）
//   - 扩容：当负载因子超过阈值，rehash 扩容
//
// 复杂度：
//   - 理想情况：O(1) 平均查找/插入/删除
//   - 最坏情况：O(n)（所有 key 冲突）

fn hashmap_principles() {
    use std::collections::HashMap;

    // 基础用法
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);

    // get 返回 Option
    match map.get("apple") {
        Some(v) => println!("apple count = {}", v),
        None => println!("apple not found"),
    }

    // or_insert
    let entry = map.entry("cherry").or_insert(0);
    *entry += 1;
    println!("cherry count = {}", map.get("cherry").unwrap());

    // 自定义 key
    #[derive(Debug, Hash, Eq, PartialEq)]
    struct Point { x: i32, y: i32 }

    let mut point_map = HashMap::new();
    point_map.insert(Point { x: 0, y: 0 }, "origin");
    point_map.insert(Point { x: 1, y: 1 }, "diagonal");
    println!("origin = {:?}", point_map.get(&Point { x: 0, y: 0 }));

    // 哈希一致性：确保 Eq 和 Hash 逻辑一致
    // 错误实现会导致查找失败
}

// 【问题7】Rust 的图（Graph）如何存储和遍历？
//
// 邻接表：用 Vec<Vec<usize>> 存储
// BFS/DFS 遍历

fn graph_algorithms() {
    // 邻接表表示
    #[derive(Debug)]
    struct Graph {
        adj: Vec<Vec<usize>>,
    }

    impl Graph {
        fn new(n: usize) -> Self {
            Graph { adj: vec![vec![]; n] }
        }

        fn add_edge(&mut self, from: usize, to: usize) {
            self.adj[from].push(to);
            // 无向图: self.adj[to].push(from);
        }

        fn bfs(&self, start: usize) {
            use std::collections::VecDeque;
            let mut visited = vec![false; self.adj.len()];
            let mut queue = VecDeque::new();
            queue.push_back(start);
            visited[start] = true;

            while let Some(u) = queue.pop_front() {
                print!("{} ", u);
                for &v in &self.adj[u] {
                    if !visited[v] {
                        visited[v] = true;
                        queue.push_back(v);
                    }
                }
            }
            println!();
        }

        fn dfs(&self, start: usize) {
            let mut visited = vec![false; self.adj.len()];
            self.dfs_recursive(start, &mut visited);
            println!();
        }

        fn dfs_recursive(&self, u: usize, visited: &mut [bool]) {
            visited[u] = true;
            print!("{} ", u);
            for &v in &self.adj[u] {
                if !visited[v] { self.dfs_recursive(v, visited); }
            }
        }
    }

    let mut g = Graph::new(5);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(2, 3);
    g.add_edge(3, 4);

    println!("BFS from 0:");
    g.bfs(0);
    println!("DFS from 0:");
    g.dfs(0);

    // 最短路径（BFS）
    fn shortest_path(g: &Graph, from: usize, to: usize) -> Option<Vec<usize>> {
        use std::collections::VecDeque;
        let mut visited = vec![false; g.adj.len()];
        let mut parent = vec![None; g.adj.len()];
        let mut queue = VecDeque::new();
        queue.push_back(from);
        visited[from] = true;

        while let Some(u) = queue.pop_front() {
            if u == to {
                let mut path = vec![];
                let mut cur = to;
                while cur != from {
                    path.push(cur);
                    cur = parent[cur].unwrap();
                }
                path.push(from);
                path.reverse();
                return Some(path);
            }
            for &v in &g.adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = Some(u);
                    queue.push_back(v);
                }
            }
        }
        None
    }

    println!("shortest path 0→4: {:?}", shortest_path(&g, 0, 4));
}

// 【问题8】Rust 的算法复杂度分析——如何评估代码效率？
//
// 大 O 记号：
//   - O(1)：常数
//   - O(log n)：对数
//   - O(n)：线性
//   - O(n log n)：线性对数
//   - O(n²)：平方
//   - O(2ⁿ)：指数

fn complexity_analysis() {
    // O(1) - 常数时间
    fn constant(arr: &[i32]) -> i32 { arr[0] }

    // O(log n) - 对数时间（二分查找）
    fn logarithmic(n: usize) -> usize {
        let mut count = 0;
        let mut len = n;
        while len > 0 {
            len /= 2;
            count += 1;
        }
        count
    }
    println!("log2(1024) = {}", logarithmic(1024));

    // O(n) - 线性时间（遍历）
    fn linear(arr: &[i32]) -> i32 { arr.iter().sum() }

    // O(n log n) - 线性对数（归并排序）
    fn nlogn_overhead(arr: &[i32]) -> usize {
        let mut result = 0;
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 { j /= 2; result += 1; }
        }
        result
    }
    let data = vec![1; 1000];
    println!("n log n operations for n=1000: {}", nlogn_overhead(&data));

    // 时间复杂度对比
    println!();
    println!("复杂度对比（n=1000时）:");
    println!("  O(1):      1 次操作");
    println!("  O(log n):  ~10 次操作");
    println!("  O(n):      1000 次操作");
    println!("  O(n log n): ~10000 次操作");
    println!("  O(n²):     1000000 次操作");
}

// ============================================================
// 【对比】Rust vs Lua vs Python 数据结构与算法
// ============================================================
// Rust:
//   - 手动实现数据结构是学习的好方式
//   - Option/Result 提供安全的空值处理
//   - Box 用于堆分配（递归结构）
//   - 借用检查保证内存安全

// Lua:
//   - table 是唯一复合类型（数组+映射）
//   - 需要手动实现链表/树等结构
//   - 无泛型，类型安全靠约定

// Python:
//   - list/tuple/dict/set 是内建数据结构
//   - bisect 提供二分查找
//   - heapq 提供堆操作
//   - dataclass 简化结构体定义

fn compare_dsa() {
    println!("=== 三语言数据结构对比 ===");

    // 内置数据结构
    // Rust: Vec, HashMap, HashSet, BTreeMap (std)
    // Python: list, dict, set, collections
    // Lua: table (唯一的复合类型)

    // 排序
    // Rust: .sort() O(n log n)
    // Python: sorted() / list.sort()
    // Lua: table.sort (标准库)

    // 自定义实现
    // Rust: 显式堆分配（Box），安全
    // Python: 随意，但无编译时检查
}

// ============================================================
// 练习题
// ============================================================
// 1. 实现一个双向链表，支持在任意位置插入/删除
// 2. 实现一个 BST，支持插入、删除、查找（递归和迭代）
// 3. 用 BFS 实现无权图的最短路径
// 4. 分析以下代码的时间复杂度：嵌套循环 + early return

fn main() {
    println!("=== 模块十四：数据结构与算法 ===");

    linked_list();
    stack_queue();
    tree_structures();
    sorting_algorithms();
    binary_search();
    hashmap_principles();
    graph_algorithms();
    complexity_analysis();
    compare_dsa();

    println!("\n✅ 所有示例运行成功！");
}