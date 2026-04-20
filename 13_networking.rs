// ============================================================
// 模块十三：网络编程
// TCP/UDP/Socket/HTTP
// ============================================================

use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr};
use std::io::{Read, Write};
use std::time::Duration;

// 【问题1】Rust 的 TCP 连接如何建立和读写？
//
// TcpListener：服务器端监听
// TcpStream：客户端/服务器连接
// 标准库是同步 IO，异步需要 tokio

fn tcp_basics() {
    // TCP 服务器
    fn start_server() -> std::thread::JoinHandle<()> {
        std::thread::spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
            println!("server listening on :8080");

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("client connected from {}", addr);
                    let mut buf = [0u8; 1024];
                    match stream.read(&mut buf) {
                        Ok(n) => {
                            let msg = String::from_utf8_lossy(&buf[..n]);
                            println!("received: {}", msg);
                            stream.write_all(b"Hello from server!\n").unwrap();
                        }
                        Err(e) => println!("read error: {}", e),
                    }
                }
                Err(e) => println!("accept error: {}", e),
            }
        })
    }

    // TCP 客户端
    fn start_client() -> std::thread::JoinHandle<()> {
        std::thread::spawn(|| {
            std::thread::sleep(Duration::from_millis(100)); // 等待服务器启动
            let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
            println!("connected to server");

            stream.write_all(b"Hello from client!\n").unwrap();

            let mut buf = vec![0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            let response = String::from_utf8_lossy(&buf[..n]);
            println!("server said: {}", response);
        })
    }

    let server = start_server();
    let client = start_client();

    client.join().unwrap();
    server.join().unwrap();
    println!("TCP 通信完成");
}

// 【问题2】Rust 的 UDP Socket 如何使用？
//
// UDP：无连接、无确认、不保证顺序
// 适合：DNS 查询、视频流、实时游戏

fn udp_basics() {
    // UDP 服务器
    fn server() {
        let socket = UdpSocket::bind("127.0.0.1:8081").unwrap();
        println!("UDP server listening on :8081");

        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let msg = String::from_utf8_lossy(&buf[..size]);
                println!("from {}: {}", src, msg);
                socket.send_to(b"UDP response", src).unwrap();
            }
            Err(e) => println!("recv error: {}", e),
        }
    }

    // UDP 客户端
    fn client() {
        std::thread::sleep(Duration::from_millis(50));
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        socket.send_to(b"UDP client message", "127.0.0.1:8081").unwrap();
        println!("sent UDP message");

        let mut buf = [0u8; 1024];
        let (size, _) = socket.recv_from(&mut buf).unwrap();
        let response = String::from_utf8_lossy(&buf[..size]);
        println!("server response: {}", response);
    }

    let server_handle = std::thread::spawn(server);
    let client_handle = std::thread::spawn(client);

    client_handle.join().unwrap();
    server_handle.join().unwrap();
    println!("UDP 通信完成");
}

// 【问题3】Rust 的 HTTP 请求如何发送？（无第三方库）
//
// 标准库没有完整的 HTTP 客户端。
// 可以手动构造 HTTP/1.1 请求。
// 完整 HTTP 支持需要 reqwest / ureq 等库。

fn http_request() {
    // 手动构造 HTTP 请求
    let mut stream = TcpStream::connect("httpbin.org:80").unwrap();
    stream.write_all(b"GET /get HTTP/1.1\r\nHost: httpbin.org\r\n\r\n").unwrap();

    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();

    let text = String::from_utf8_lossy(&response);
    // 打印前500字符
    println!("HTTP response (first 500 chars):\n{}", &text[..500.min(text.len())]);
}

// 【问题4】Rust 的网络超时和选项如何配置？
//
// TcpStream::set_read_timeout / set_write_timeout
// TcpStream::set_nonblocking

fn socket_options() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    // 连接超时
    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();

    // 非阻塞模式
    stream.set_nonblocking(true).unwrap();

    // 保持连接
    stream.set_keepalive(Some(Duration::from_secs(30))).unwrap();

    println!("socket options configured");

    // 获取本地/远程地址
    let local = stream.local_addr().unwrap();
    let peer = stream.peer_addr().unwrap();
    println!("local = {}, peer = {}", local, peer);

    // TCP Nodelay（禁用 Nagle 算法）
    // 需要 socket2 crate
}

// 【问题5】Rust 的域名解析和 IP 地址如何处理？
//
// SocketAddr：IP + 端口
// ToSocketAddrs trait：提供域名解析

fn address_resolution() {
    // 解析域名
    use std::net::ToSocketAddrs;

    let addresses: Vec<SocketAddr> = "example.com:80"
        .to_socket_addrs()
        .unwrap()
        .collect();
    println!("example.com:80 resolves to {:?}", addresses);

    // 直接使用 IP
    let addr: SocketAddr = "192.168.1.1:8080".parse().unwrap();
    println!("parsed addr: {}", addr);

    // IpAddr 类型
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    let v4: IpAddr = Ipv4Addr::new(127, 0, 0, 1).into();
    let v6: IpAddr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).into();

    println!("IPv4 loopback = {}", v4);
    println!("IPv6 loopback = {}", v6);

    // 检查 IP 类型
    if v4.is_ipv4() { println!("v4 is IPv4"); }
    if v6.is_ipv6() { println!("v6 is IPv6"); }
}

// 【问题6】Rust 如何实现端口扫描（简单的 TCP 连接测试）？
//
// 遍历端口列表，尝试 TcpStream::connect。

fn port_scanner() {
    let host = "127.0.0.1";
    let ports = [21, 22, 23, 25, 80, 443, 8080, 9000];

    for port in ports {
        let addr = format!("{}:{}", host, port);
        match TcpStream::connect_timeout(
            &addr.parse().unwrap(),
            Duration::from_millis(200)
        ) {
            Ok(_) => println!("port {} OPEN", port),
            Err(_) => println!("port {} CLOSED", port),
        }
    }
}

// 【问题7】Rust 的 Tokio 异步网络编程是什么？（简要介绍）
//
// Tokio 是 Rust 最流行的异步运行时。
// async/await 语法让异步代码看起来像同步代码。
//
// 概念：
//   - async fn：异步函数
//   - .await：等待异步操作完成
//   - Tokio::spawn：启动异步任务

fn async_intro() {
    // 注意：这只是语法展示，实际运行需要 tokio 依赖
    // fn example() {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     rt.block_on(async {
    //         let data = fetch_from_network().await;
    //         println!("got: {}", data);
    //     });
    // }

    println!(" Tokio 异步示例（需要 tokio 依赖）");
    println!("// 引入: tokio = {{ features = ['full'] }}");
    println!("// 使用: async fn + .await + tokio::spawn");
    println!();
    println!("fn fetch_data() {{");
    println!("    tokio::spawn(async {{");
    println!("        // 异步操作");
    println!("        let result = some_async_op().await;");
    println!("    }});");
    println!("}}");
}

// 【问题8】Rust 的 WebSocket 如何实现？
//
// WebSocket 是基于 HTTP 升级的双向通信协议。
// 完整实现需要 tungstenite / tokio-tungstenite 库。

fn websocket_intro() {
    println!(" WebSocket 需要第三方库");
    println!();
    println!("常用库:");
    println!("  - tokio-tungstenite (异步)");
    println!("  - tungstenite (同步)");
    println!();
    println!("基本流程:");
    println!("  1. 建立 HTTP 连接");
    println!("  2. 发送 Upgrade 请求");
    println!("  3. 交换 WebSocket 帧");
    println!();
    println!("// tokio-tungstenite 示例:");
    println!("// let (ws_stream, _) = connect(\"ws://echo.websocket.org\").await?;");
    println!("// let (write, read) = ws_stream.split();");
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - 标准库提供 TcpListener/TcpStream/UdpSocket
//   - 同步 IO，异步需要 tokio/async-std
//   - HTTP 需要第三方库（reqwest）
//   - 类型安全，编译时检查

// Python:
//   - socket 标准库支持 TCP/UDP
//   - urllib/requests 提供 HTTP
//   - asyncio 提供异步网络
//   - aiohttp/httpx 异步 HTTP

// Lua:
//   - luasocket 提供同步 socket
//   - lua-nginx 支持异步
//   - 无内置 HTTP，需要库

// Go:
//   - net 标准库提供 TCP/UDP
//   - net/http 提供 HTTP 服务器和客户端
//   - 原生异步支持（goroutine）
//   - context 用于超时控制

// C++:
//   - asio 库提供异步网络（Boost.Asio / standalone asio）
//   - socket API 需要平台适配
//   - HTTP 需要第三方库（curl, libcurl）

fn compare_networking() {
    println!("=== 三语言网络编程对比 ===");

    // TCP 服务器
    // Rust: TcpListener::bind + accept
    // Python: socket + accept
    // Lua: require('socket').tcp()

    // 异步
    // Rust: tokio + async/await
    // Python: asyncio + await
    // Lua: nginx cosocket / luv

    // HTTP
    // Rust: reqwest (推荐), curl, http req
    // Python: requests / urllib
    // Lua: lua-http / luaresty
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 实现一个 TCP echo 服务器，回显客户端发送的每一行内容
// 2. 实现一个 UDP 聊天程序，支持多个客户端互相发送消息（服务器转发）
// 3. 写一个函数获取指定 URL 的 HTTP 状态码（使用 std::net TcpStream 手动构造请求）
// 4. 实现一个简单的端口扫描器，并行检测多个端口（使用 thread spawn）
// 5. 解释 TCP 和 UDP 的区别，以及各自的适用场景（至少3个例子）

fn main() {
    println!("=== 模块十三：网络编程 ===");

    // tcp_basics(); // 启动需要单独的服务器/客户端，实际运行会冲突
    udp_basics();
    http_request();
    address_resolution();
    port_scanner();
    async_intro();
    websocket_intro();
    compare_networking();

    println!("\n✅ 所有示例运行成功！");
}