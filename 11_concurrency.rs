// ============================================================
// 模块十一：并发编程
// 线程/锁/同步/线程池/死锁
// ============================================================

use std::thread;
use std::sync::{Arc, Mutex, RwLock, Condvar, Barrier, mpsc};
use std::time::Duration;
use std::collections::HashMap;

// 【问题1】Rust 的线程创建与线程间通信是什么？
//
// Rust 标准库提供 std::thread，使用 spawn 创建线程。
// 线程通信：共享内存（Mutex/RwLock）或消息传递（mpsc channel）。
//
// 编译器视角：spawn 调用系统 API（pthread_create / CreateThread）创建OS线程。

fn thread_creation() {
    // 基本线程创建
    let handle = thread::spawn(|| {
        println!("子线程执行中...");
        42
    });

    // 等待线程结束并获取返回值
    let result = handle.join().unwrap();
    println!("子线程返回: {}", result);

    // 多线程
    let handles: Vec<_> = (0..4).map(|i| {
        thread::spawn(move || {
            println!("线程 {} 执行", i);
            i * 2
        })
    }).collect();

    for h in handles {
        let r = h.join().unwrap();
        println!("线程返回: {}", r);
    }

    // 线程间共享数据
    let shared = Arc::new(vec![1, 2, 3]);
    let shared2 = Arc::clone(&shared);
    thread::spawn(move || {
        println!("shared2 in thread: {:?}", shared2);
    }).join().unwrap();
    println!("shared in main: {:?}", shared);
}

// 【问题2】Rust 的 Mutex 如何实现线程安全？
//
// Mutex（互斥锁）：同一时刻只允许一个线程访问数据。
// 使用 Arc（原子引用计数）实现多线程共享所有权。
//
// 内存视角：
//   - MutexGuard 是 RAII 模式，drop 时自动释放锁
//   - 锁争用导致线程阻塞（上下文切换）

fn mutex_usage() {
    // Arc + Mutex 实现多线程安全共享
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("线程 {} 增加计数到 {}", i, *num);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());

    // RwLock（读写锁）：读多写少场景更高效
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // 读操作（多个读者）
    let r1 = Arc::clone(&data);
    let reader1 = thread::spawn(move || {
        let v = r1.read().unwrap();
        println!("reader1: {:?}", v);
    });

    let r2 = Arc::clone(&data);
    let reader2 = thread::spawn(move || {
        let v = r2.read().unwrap();
        println!("reader2: {:?}", v);
    });

    reader1.join().unwrap();
    reader2.join().unwrap();

    // 写操作（独占）
    let mut w = data.write().unwrap();
    w.push(4);
    println!("after write: {:?}", w);
}

// 【问题3】Rust 的消息传递（Channel）如何实现？
//
// mpsc（多生产者单消费者）channel：
//   - sender.send(value)：发送
//   - receiver.recv()：接收（阻塞）
//   - receiver.try_recv()：非阻塞尝试

fn channel_communication() {
    // 创建 channel
    let (tx, rx) = mpsc::channel();

    // 生产者线程
    let producer = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程接收
    while let Ok(val) = rx.recv() {
        println!("received: {}", val);
    }

    producer.join().unwrap();
    println!("producer finished");

    // 多生产者
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    let p1 = thread::spawn(move || {
        for i in 0..3 {
            tx.send(format!("P1: {}", i)).unwrap();
        }
    });

    let p2 = thread::spawn(move || {
        for i in 0..3 {
            tx2.send(format!("P2: {}", i)).unwrap();
        }
    });

    drop(tx); // 确保所有发送者都关闭
    drop(tx2);

    for received in rx {
        println!("got: {}", received);
    }
}

// 【问题4】Rust 的条件变量（Condvar）和屏障（Barrier）是什么？
//
// Condvar：等待某个条件成立
// Barrier：等待所有线程都到达某点

fn condvar_and_barrier() {
    // Condvar 示例：生产者-消费者
    let queue = Arc::new(Mutex::new(Vec::<i32>::new()));
    let not_empty = Arc::new(Condvar::new());

    let producer = thread::spawn({
        let queue = Arc::clone(&queue);
        let not_empty = Arc::clone(&not_empty);
        move || {
            for i in 0..5 {
                let mut q = queue.lock().unwrap();
                q.push(i);
                not_empty.notify_one(); // 通知等待者
                println!("produced: {}", i);
                thread::sleep(Duration::from_millis(50));
            }
        }
    });

    let consumer = thread::spawn({
        let queue = Arc::clone(&queue);
        let not_empty = Arc::clone(&not_empty);
        move || {
            let mut count = 0;
            while count < 5 {
                let mut q = queue.lock().unwrap();
                while q.is_empty() {
                    q = not_empty.wait(q).unwrap();
                }
                let item = q.remove(0);
                println!("consumed: {}", item);
                count += 1;
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    // Barrier 示例
    println!("\nBarrier 示例:");
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let b = Arc::clone(&barrier);
        let h = thread::spawn(move || {
            println!("线程 {} 到达屏障", i);
            b.wait(); // 等待其他线程
            println!("线程 {} 跨过屏障", i);
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }
    println!("全部跨过屏障");
}

// 【问题5】Rust 的线程安全集合如何使用？
//
// 常用线程安全集合：
//   - Arc<Mutex<T>>：包装任意类型
//   - Rayon：并行迭代器库
//   - crossbeam：额外并发原语

fn thread_safe_collections() {
    // 线程安全的计数器
    let counter = Arc::new(Mutex::new(0i32));

    let handles: Vec<_> = (0..10).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("final counter: {}", *counter.lock().unwrap());

    // 线程安全的 HashMap
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let m = Arc::clone(&map);
        let h = thread::spawn(move || {
            let mut map = m.lock().unwrap();
            map.insert(i, format!("value_{}", i));
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }
    println!("map: {:?}", *map.lock().unwrap());

    // Once（一次性初始化）
    use std::sync::OnceLock;
    static INIT: OnceLock<i32> = OnceLock::new();
    fn get_singleton() -> i32 {
        *INIT.get_or_init(|| {
            println!("initializing...");
            42
        })
    }
    println!("singleton: {}", get_singleton());
    println!("singleton again: {}", get_singleton()); // 不再初始化
}

// 【问题6】Rust 的死锁如何避免？
//
// 死锁四个必要条件（系统层面）：
//   1. 互斥：资源只能被一个线程持有
//   2. 占有并等待：线程持有资源同时等待其他资源
//   3. 不可抢占：资源不能被强制夺走
//   4. 循环等待：形成线程环形依赖
//
// 避免策略：
//   - 固定加锁顺序（防止循环等待）
//   - 使用 scoped thread（不超过范围）
//   - 减少锁的持有时间

fn deadlock_prevention() {
    // 错误示例：循环等待导致死锁
    // let a = Arc::new(Mutex::new(1));
    // let b = Arc::new(Mutex::new(2));
    // thread1: lock a, lock b → 执行操作
    // thread2: lock b, lock a → 死锁！

    // 正确做法：固定顺序
    let a = Arc::new(Mutex::new(1));
    let b = Arc::new(Mutex::new(2));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    // 线程1：先 a 后 b
    let t1 = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _b = b1.lock().unwrap();
        println!("thread1: got both locks");
    });

    // 线程2：也是先 a 后 b（固定顺序避免循环）
    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let t2 = thread::spawn(move || {
        let _a = a2.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _b = b2.lock().unwrap();
        println!("thread2: got both locks");
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("成功避免死锁");

    // 尽量减少锁持有时间
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let d = Arc::clone(&data);

    // 快速获取、快速释放
    let id = thread::spawn(move || {
        let v = d.lock().unwrap(); // 获取
        let sum: i32 = v.iter().sum(); // 使用
        drop(v); // 释放（RAII 自动）
        println!("sum = {}", sum);
        // 复杂的长时间操作在锁外进行
    });
    id.join().unwrap();
}

// 【问题7】Rust 的 scoped thread（线程作用域）是什么？
//
// 标准库 thread::spawn 需要 'static 生命周期（因为线程可能逃逸）。
// scoped thread（crossbeam 或标准库受限）可以在作用域内创建线程。

fn scoped_threads() {
    // 标准库不支持 scoped thread，但 crossbeam 支持
    use crossbeam::scope;

    let data = vec![1, 2, 3, 4, 5];
    let mut results = vec![];

    scope(|s| {
        for chunk in data.chunks(2) {
            s.spawn(|_| {
                let sum: i32 = chunk.iter().sum();
                results.push(sum); // 外部变量需要安全借用
            });
        }
    }).unwrap();

    println!("scoped results: {:?}", results);

    // 标准库的方式：JoinHandle 立即 join
    let data = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = (0..3).map(|i| {
        let d = Arc::clone(&data);
        thread::spawn(move || {
            d.lock().unwrap().push(i * i);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("results: {:?}", *data.lock().unwrap());
}

// 【问题8】Rust 的线程池实现和 work-stealing 是什么？
//
// 线程池：预先创建一组线程，复用执行多个任务。
// work-stealing：空闲线程从其他线程的队列"偷"任务。
//
// 常用库：rayon（并行迭代），tokio（异步任务），threadpool crate

fn threadpool_demo() {
    // 使用 threadpool crate 简单线程池
    use std::thread;

    let (tx, rx) = mpsc::channel();

    // 手动简单线程池
    let num_threads = 4;
    let (task_tx, task_rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();

    // 启动 worker 线程
    let mut workers = vec![];
    for i in 0..num_threads {
        let rx = Arc::new(Mutex::new(task_rx.clone()));
        let tx = tx.clone();
        let worker = thread::spawn(move || {
            loop {
                let task = {
                    let guard = rx.lock().unwrap();
                    guard.recv()
                };
                match task {
                    Ok(f) => {
                        f();
                        tx.send(i).unwrap();
                    }
                    Err(_) => break, // channel 关闭
                }
            }
        });
        workers.push(worker);
    }

    // 提交任务
    for i in 0..8 {
        let task = Box::new(move || {
            println!("task {} executing", i);
        });
        task_tx.send(task).unwrap();
    }

    drop(task_tx); // 关闭任务 channel

    // 收集结果
    let mut completions = vec![];
    for _ in 0..8 {
        completions.push(rx.recv().unwrap());
    }
    println!("completed by workers: {:?}", completions);

    // 优雅关闭
    // drop(task_tx) 已经触发 worker 退出循环
    for w in workers {
        w.join().unwrap();
    }
    println!("thread pool shut down");
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - std::thread 创建线程，Arc<Mutex<T>> 共享数据
//   - mpsc channel 消息传递
//   - 编译时借用检查 + 类型系统保证线程安全
//   - Send/Sync trait 标记可跨线程传递的类型
//   - 无内置异步，用 tokio 异步运行时

// Python:
//   - threading 模块（真正的线程，受 GIL 限制）
//   - multiprocessing 模块（绕过 GIL 的多进程）
//   - asyncio 异步编程
//   - concurrent.futures 提供线程/进程池

// Lua:
//   - 协程（coroutine）是轻量级并发，不是并行
//   - 多线程需要 ffi 调用系统 API（如 pthread）
//   - Lua VM 是单线程的，不支持真正的多线程

// Go:
//   - goroutine：轻量级并发，go 关键字创建
//   - channel：消息传递（带缓冲/不带缓冲）
//   - sync.Mutex / sync.WaitGroup 同步原语
//   - 内置调度器（M:N 映射到 OS 线程）

// C++:
//   - std::thread 创建线程
//   - std::mutex / std::condition_variable 同步
//   - std::atomic 原子操作
//   - std::async / std::future 异步
//   - 无内置异步运行时

fn compare_concurrency() {
    println!("=== 三语言并发对比 ===");

    // 线程安全
    // Rust: 类型系统保证（Send/Sync trait）
    // Python: GIL 保证，但效率受限
    // Lua: 不支持原生多线程

    // 共享内存
    // Rust: Arc<Mutex<T>>
    // Python: threading.Lock / multiprocessing.Manager
    // Lua: 无

    // 消息传递
    // Rust: mpsc channel
    // Python: queue.Queue
    // Lua: 无
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 实现一个线程安全的生产消费队列，支持多生产者多消费者
// 2. 实现一个读写锁（支持多个读者或单个写者），用 RwLock 实现
// 3. 用消息传递实现一个并行 word count，统计多个文件的单词频率
// 4. 解释 Send/Sync trait 的意义，为什么它们是 unsafe trait，哪些类型默认实现了它们？
// 5. 实现一个哲学家就餐问题（死锁场景），然后修复它（避免死锁）

fn main() {
    println!("=== 模块十一：并发编程 ===");

    thread_creation();
    mutex_usage();
    channel_communication();
    condvar_and_barrier();
    thread_safe_collections();
    deadlock_prevention();
    scoped_threads();
    threadpool_demo();
    compare_concurrency();

    println!("\n✅ 所有示例运行成功！");
}