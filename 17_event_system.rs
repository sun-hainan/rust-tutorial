// ============================================================
// 模块十七：信息分发与事件系统
// 观察者/EventEmitter/信号槽/消息队列/中间件
// ============================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::fmt::Debug;

// 【问题1】Rust 的观察者模式（Observer Pattern）如何实现？
//
// 观察者模式：主题（Subject）维护观察者列表，状态变化时通知所有观察者。
// 发布-订阅（pub/sub）是一种变体。
//
// 场景：GUI 事件、消息通知、日志系统

fn observer_pattern() {
    // 观察者 trait
    trait Observer<T> {
        fn update(&self, event: &T);
    }

    #[derive(Debug)]
    struct Event {
        message: String,
        timestamp: u64,
    }

    // 具体观察者
    struct Logger;
    struct EmailNotifier;

    impl Observer<Event> for Logger {
        fn update(&self, event: &Event) {
            println!("[LOG] {} at {}", event.message, event.timestamp);
        }
    }

    impl Observer<Event> for EmailNotifier {
        fn update(&self, event: &Event) {
            println!("[EMAIL] Would send notification: {}", event.message);
        }
    }

    // 主题（Subject）
    struct EventSubject {
        observers: Vec<Box<dyn Observer<Event>>>,
    }

    impl EventSubject {
        fn new() -> Self { EventSubject { observers: vec![] } }

        fn subscribe(&mut self, observer: Box<dyn Observer<Event>>) {
            self.observers.push(observer);
        }

        fn unsubscribe(&mut self, index: usize) {
            self.observers.remove(index);
        }

        fn notify(&self, event: &Event) {
            for observer in &self.observers {
                observer.update(event);
            }
        }

        fn publish(&mut self, message: String) {
            let event = Event {
                message,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            self.notify(&event);
        }
    }

    // 使用
    let mut subject = EventSubject::new();
    subject.subscribe(Box::new(Logger));
    subject.subscribe(Box::new(EmailNotifier));

    subject.publish("User logged in".to_string());
    subject.publish("Order placed".to_string());
}

// 【问题2】Rust 的 EventEmitter（事件发射器）如何实现？
//
// EventEmitter：类似 Node.js 的事件系统。
// 支持任意事件类型、多个监听器、一次性监听（once）

fn event_emitter() {
    struct EventEmitter {
        listeners: HashMap<String, Vec<Box<dyn Fn(&str)>>>,
    }

    impl EventEmitter {
        fn new() -> Self {
            EventEmitter { listeners: HashMap::new() }
        }

        fn on(&mut self, event: &str, handler: Box<dyn Fn(&str)>) {
            self.listeners
                .entry(event.to_string())
                .or_insert_with(Vec::new)
                .push(handler);
        }

        fn once(&mut self, event: &str, handler: Box<dyn Fn(&str)>) {
            // 一次性监听器的实现需要包装器
            let mut triggered = false;
            let handler = Box::new(move |msg: &str| {
                if !triggered {
                    triggered = true;
                    handler(msg);
                }
            });
            self.on(event, handler);
        }

        fn emit(&self, event: &str, data: &str) {
            if let Some(handlers) = self.listeners.get(event) {
                for handler in handlers {
                    handler(data);
                }
            }
        }

        fn remove_all(&mut self, event: &str) {
            self.listeners.remove(event);
        }
    }

    let mut emitter = EventEmitter::new();

    emitter.on("data", Box::new(|data| println!("handler1 got: {}", data)));
    emitter.on("data", Box::new(|data| println!("handler2 got: {}", data)));

    emitter.emit("data", "hello world");

    emitter.once("single", Box::new(|_| println!("This fires only once")));
    emitter.emit("single", "first");
    emitter.emit("single", "second"); // 不触发

    println!("EventEmitter 示例完成");
}

// 【问题3】Rust 的信号槽（Signal/Slot）机制是什么？
//
// 信号槽：类似 Qt 的信号与槽机制。
// 信号（Signal）：事件发射点
// 槽（Slot）：事件处理函数
// 连接（Connection）：信号与槽的关联

fn signal_slot() {
    println!(" 信号槽机制");
    println!();
    println!("## Qt 风格的信号槽");
    println!();
    println!("// 信号定义（在类中）");
    println!("signals:");
    println!("    void clicked();");
    println!("    void valueChanged(int newValue);");
    println!();
    println!("// 槽定义");
    println!("public slots:");
    println!("    void handleClick();");
    println!("    void handleValueChange(int value);");
    println!();
    println!("// 连接");
    println!("QObject::connect(button, &QPushButton::clicked,");
    println!("               this, &MyWidget::handleClick);");
    println!();
    println!("## Rust 实现思路");
    println!();
    println!("use std::sync::Arc;");
    println!();
    println!("struct Button {{");
    println!("    click_handlers: Vec<Box<dyn Fn()>>,");
    println!("}}");
    println!();
    println!("impl Button {{");
    println!("    fn connect_click<F: Fn() + 'static>(&mut self, handler: F) {{");
    println!("        self.click_handlers.push(Box::new(handler));");
    println!("    }}");
    println!();
    println!("    fn click(&self) {{");
    println!("        for handler in &self.click_handlers {{");
    println!("            handler();");
    println!("        }}");
    println!("    }}");
    println!("}}");
}

// 【问题4】Rust 的消息队列如何实现？
//
// 消息队列：生产者-消费者模式
// 异步处理、负载均衡、解耦
//
// 实现：channel + worker pool

fn message_queue() {
    // 简单的消息队列
    #[derive(Debug)]
    enum Message {
        Task(String),
        Stop,
    }

    // 生产者
    fn producer(tx: mpsc::Sender<Message>) {
        for i in 0..5 {
            let msg = format!("task-{}", i);
            tx.send(Message::Task(msg)).unwrap();
        }
        tx.send(Message::Stop).unwrap();
    }

    // 消费者
    fn consumer(rx: mpsc::Receiver<Message>) {
        loop {
            match rx.recv() {
                Ok(Message::Task(s)) => println!("processed: {}", s),
                Ok(Message::Stop) => { println!("consumer stopping"); break; }
                Err(_) => break,
            }
        }
    }

    let (tx, rx) = mpsc::channel();

    let p = std::thread::spawn(move || producer(tx));
    let c = std::thread::spawn(move || consumer(rx));

    p.join().unwrap();
    c.join().unwrap();

    // 带优先级的消息队列
    println!("\n优先级队列示例:");
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;

    #[derive(Debug)]
    struct PrioritizedMessage(i32, String);

    impl PartialEq for PrioritizedMessage {
        fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }
    }

    impl Eq for PrioritizedMessage {}

    impl PartialOrd for PrioritizedMessage {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl Ord for PrioritizedMessage {
        fn cmp(&self, other: &Self) -> Ordering { other.0.cmp(&self.0) }
    }

    let mut heap = BinaryHeap::new();
    heap.push(PrioritizedMessage(3, "low priority".into()));
    heap.push(PrioritizedMessage(1, "high priority".into()));
    heap.push(PrioritizedMessage(2, "medium priority".into()));

    while let Some(PrioritizedMessage(priority, msg)) = heap.pop() {
        println!("priority {}: {}", priority, msg);
    }
}

// 【问题5】Rust 的中间件（Middleware）模式是什么？
//
// 中间件：链式处理请求，每个处理器可以修改、传递或终止请求。
// 场景：Web 框架（actix-web、tower）、请求拦截、日志、认证

fn middleware_pattern() {
    println!(" 中间件模式");
    println!();
    println!("## 中间件链");
    println!();
    println!("Request → Logger → Auth → RateLimit → Handler");
    println!("                  ↓         ↓");
    println!("               拒绝       拒绝");
    println!();
    println!("## Web 框架示例（axum/tower）");
    println!();
    println!("use tower::ServiceBuilder;");
    println!();
    println!("let service = ServiceBuilder::new()");
    println!("    .layer(log_layer())");
    println!("    .layer(auth_layer())");
    println!("    .layer(rate_limit_layer())");
    println!("    .service(handler);");
    println!();
    println!("## 自定义中间件");
    println!();
    println!("struct LoggingLayer;");
    println!();
    println!("impl<S> Layer<S> for LoggingLayer {{");
    println!("    type Service = LoggingService<S>;");
    println!();
    println!("    fn layer(&self, inner: S) -> Self::Service {{");
    println!("        LoggingService {{ inner }}");
    println!("    }}");
    println!("}}");
}

// 【问题6】Rust 的 Actor 模型如何实现？
//
// Actor 模型：每个 Actor 是独立的执行单元，通过消息传递通信。
// 状态私有，消息驱动，无共享内存。
// 实现：actix 库

fn actor_model_intro() {
    println!(" Actor 模型");
    println!();
    println!("## Actor 核心概念");
    println!();
    println!("1.  Actor：计算单元，有状态，信箱");
    println!("2.  消息：异步通信，信箱排队");
    println!("3.  地址：用于发送消息给 Actor");
    println!();
    println!("## actix 示例");
    println!();
    println!("use actix::{{Actor, Context, Handler, Message}};");
    println!();
    println!("struct ChatServer {{");
    println!("    sessions: Vec<Addr<ChatSession>>,");
    println!("}}");
    println!();
    println!("impl Actor for ChatServer {{");
    println!("    type Context = Context<Self>;");
    println!();
    println!("    fn started(&mut self, _ctx: &mut Self::Context) {{");
    println!("        println!(\"Chat server started\");");
    println!("    }}");
    println!("}}");
    println!();
    println!("// 定义消息");
    println!("#[derive(Message)]");
    println!("#[rtype(result = \"()\")]");
    println!("struct Connect(pub Addr<ChatSession>);");
    println!();
    println!("impl Handler<Connect> for ChatServer {{");
    println!("    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) {{");
    println!("        self.sessions.push(msg.0);");
    println!("    }}");
    println!("}}");
}

// 【问题7】Rust 的响应式编程（Reactive）如何实现？
//
// 响应式：数据流 + 声明式转换
// 库：futures（Future/Stream）、tokio-stream
//
// Stream：异步生成的值的序列

fn reactive_stream() {
    println!(" 响应式流（Reactive Streams）");
    println!();
    println!("## futures crate");
    println!();
    println!("use futures::stream::{self, StreamExt};");
    println!();
    println!("// 创建流");
    println!("let s = stream::iter(1..=5);");
    println!();
    println!("// 转换流");
    println!("let doubled = s.map(|x| x * 2);");
    println!();
    println!("// 过滤流");
    println!("let evens = doubled.filter(|x| x % 2 == 0);");
    println!();
    println!("// 收集到 Vec");
    println!("let result: Vec<i32> = evens.collect().await;");
    println!("println!(\"{{:?}}\", result); // [2, 4]");
    println!();
    println!("## 与迭代器的区别");
    println!("- 迭代器：同步，pull 模式");
    println!("- 流：异步，push 模式");
    println!("- 流可以在等待时暂停");
}

// 【问题8】Rust 的事件总线（Event Bus）如何实现？
//
// 事件总线：中心化的发布-订阅机制
// 所有组件可以注册监听事件，所有组件可以发布事件

fn event_bus() {
    // 事件总线实现
    #[derive(Debug, Clone)]
    enum AppEvent {
        UserLoggedIn { user_id: u64 },
        OrderPlaced { order_id: u64, amount: f64 },
        PaymentReceived { order_id: u64 },
    }

    struct EventBus {
        subscribers: HashMap<String, Vec<Box<dyn Fn(&AppEvent)>>>,
    }

    impl EventBus {
        fn new() -> Self { EventBus { subscribers: HashMap::new() } }

        fn subscribe<F>(&mut self, event_type: &str, handler: F)
        where F: Fn(&AppEvent) + 'static {
            self.subscribers
                .entry(event_type.to_string())
                .or_insert_with(Vec::new)
                .push(Box::new(handler));
        }

        fn publish(&self, event: &AppEvent) {
            let type_name = match event {
                AppEvent::UserLoggedIn { .. } => "UserLoggedIn",
                AppEvent::OrderPlaced { .. } => "OrderPlaced",
                AppEvent::PaymentReceived { .. } => "PaymentReceived",
            };

            if let Some(handlers) = self.subscribers.get(type_name) {
                for handler in handlers {
                    handler(event);
                }
            }
        }
    }

    let mut bus = EventBus::new();

    // 订阅所有事件（通配符需要特殊处理，这里简化）
    bus.subscribe("UserLoggedIn", Box::new(|e| {
        if let AppEvent::UserLoggedIn { user_id } = e {
            println!("Analytics: user {} logged in", user_id);
        }
    }));

    bus.subscribe("OrderPlaced", Box::new(|e| {
        if let AppEvent::OrderPlaced { order_id, amount } = e {
            println!("Shipping: prepare order {} (${:.2})", order_id, amount);
        }
    }));

    bus.subscribe("PaymentReceived", Box::new(|e| {
        if let AppEvent::PaymentReceived { order_id } = e {
            println!("Fulfillment: payment for order {} received", order_id);
        }
    }));

    // 发布事件
    bus.publish(&AppEvent::UserLoggedIn { user_id: 123 });
    bus.publish(&AppEvent::OrderPlaced { order_id: 456, amount: 99.99 });
    bus.publish(&AppEvent::PaymentReceived { order_id: 456 });

    println!("\n事件总线示例完成");
}

// ============================================================
// 【对比】Rust vs Lua vs Python 事件系统
// ============================================================
// Rust:
//   - 观察者：trait + Box<dyn Observer>
//   - EventEmitter：HashMap<String, Vec<Box<dyn Fn>>>
//   - Channel：mpsc 多生产者单消费者
//   - 异步流：futures::StreamExt

// Lua:
//   - 无内置事件系统
//   - 通过 table + function 实现
//   - OpenResty 有 nginx.context 推荐事件

// Python:
//   - asyncio：异步流、事件循环
//   - blinker：信号/事件
//   - pydispatcher：发布-订阅
//   - typed dispatch（Python 3.10+）

fn compare_event_systems() {
    println!("=== 三语言事件系统对比 ===");
    println!();
    println!("| 特性          | Rust       | Python        | Lua           |");
    println!("|---------------|------------|---------------|---------------|");
    println!("| 发布-订阅     | 自定义实现 | blinker       | table 模拟   |");
    println!("| 信号/槽       | 无原生     | 无原生        | 无原生       |");
    println!("| 异步流        | futures    | asyncio       | luv          |");
    println!("| Actor 模型    | actix      | thespian      | 无           |");
    println!("| 中间件        | tower      | wsgi middleware| 无原生      |");
}

// ============================================================
// 练习题
// ============================================================
// 1. 实现一个带 once 功能的事件发射器
// 2. 实现一个线程安全的 EventBus（Arc + Mutex）
// 3. 用 channel 实现一个生产者-消费者消息队列
// 4. 解释响应式流（Stream）和普通迭代器的核心区别

fn main() {
    println!("=== 模块十七：信息分发与事件系统 ===");

    observer_pattern();
    event_emitter();
    signal_slot();
    message_queue();
    middleware_pattern();
    actor_model_intro();
    reactive_stream();
    event_bus();
    compare_event_systems();

    println!("\n✅ 所有示例运行成功！");
}