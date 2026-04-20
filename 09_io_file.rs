// ============================================================
// 模块九：IO流与文件
// 路径/字节流/字符流/缓冲流/序列化
// ============================================================

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::Path;
use std::io::prelude::*;

// 【问题1】Rust 的路径处理 Path / PathBuf 是什么？
//
// Path：切片引用，不可变
// PathBuf：拥有的字符串，可变
//
// 跨平台：自动处理 / 和 \ 的差异

fn path_handling() {
    use std::path::Path;

    // 从字符串创建
    let path = Path::new("src/main.rs");
    println!("path = {:?}", path);

    // PathBuf（拥有所有权）
    let mut path_buf = PathBuf::from("workspace");
    path_buf.push("src");
    path_buf.push("lib");
    println!("path_buf = {:?}", path_buf);

    // 路径组件
    println!("file name = {:?}", path_buf.file_name());
    println!("extension = {:?}", path_buf.extension());
    println!("parent = {:?}", path_buf.parent());

    // 路径检查
    let p = Path::new("/home/user/document.txt");
    println!("is absolute = {}", p.is_absolute());
    println!("is relative = {}", p.is_relative());
    println!("starts with /home = {}", p.starts_with("/home"));
    println!("ends with .txt = {}", p.ends_with(".txt"));

    // join（拼接路径）
    let base = Path::new("/home/user");
    let relative = Path::new("documents/report.txt");
    println!("joined = {:?}", base.join(relative));

    // canonicalize（解析符号链接）
    let current = std::env::current_dir().unwrap();
    println!("current dir = {:?}", current);

    // 跨平台注意
    let win_path = PathBuf::from("C:\\Users\\Admin\\Documents");
    let unix_path = PathBuf::from("/home/admin/documents");
    println!("paths are OS-specific (Windows backslash vs Unix slash)");

    // 路径存在性检查
    let doc = Path::new("Cargo.toml");
    println!("'Cargo.toml' exists = {}", doc.exists());
    println!("is file = {}", doc.is_file());
    println!("is dir = {}", doc.is_dir());
}

// 【问题2】Rust 的 File / FileHandle 如何读写文件？
//
// File 是文件句柄，实现了 Read/Write trait。
// BufReader/BufWriter 提供缓冲，减少系统调用。

fn file_io() {
    // 创建文件
    let mut file = File::create("test_output.txt").unwrap();
    file.write_all(b"Hello, Rust!\n").unwrap();
    file.write_all(b"Second line\n").unwrap();
    println!("created test_output.txt");

    // 读取整个文件
    let content = fs::read_to_string("test_output.txt").unwrap();
    println!("file content:\n{}", content);

    // 打开文件读取
    let mut file = File::open("test_output.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    println!("read to string: {}", buffer);

    // 追加模式
    let mut file = OpenOptions::new()
        .append(true)
        .open("test_output.txt")
        .unwrap();
    file.write_all(b"Appended line\n").unwrap();
    println!("appended to file");

    // 读取二进制
    let bytes = fs::read("test_output.txt").unwrap();
    println!("read {} bytes", bytes.len());

    // 写字节
    let mut file = File::create("binary.bin").unwrap();
    file.write_all(&[0x48, 0x65, 0x6C, 0x6C, 0x6F]).unwrap();
    println!("wrote binary file");
}

// 【问题3】Rust 的 BufReader / BufWriter 缓冲流是什么？
//
// BufReader：包装 File，减少 read() 系统调用次数
// BufWriter：缓冲 write()，批量写入减少系统调用
//
// 性能差异：大量小读写时，缓冲可提升数十倍性能。

fn buffered_io() {
    // BufReader 示例
    let file = File::open("Cargo.toml").unwrap();
    let reader = BufReader::new(file);

    // 按行读取
    for (i, line) in reader.lines().enumerate() {
        if i >= 5 { break; }
        println!("{}: {}", i + 1, line.unwrap());
    }

    // 手动缓冲读取
    let file = File::open("Cargo.toml").unwrap();
    let mut reader = BufReader::with_capacity(1024, file);
    let mut buf = [0u8; 256];
    let n = reader.read(&mut buf).unwrap();
    println!("read {} bytes via BufReader", n);
    println!("first 50 chars: {:?}", String::from_utf8_lossy(&buf[..std::cmp::min(50, n)]));

    // BufWriter 示例
    let file = File::create("buffered_output.txt").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(b"Line 1\n").unwrap();
    writer.write_all(b"Line 2\n").unwrap();
    writer.write_all(b"Line 3\n").unwrap();
    writer.flush().unwrap(); // 手动刷新
    println!("wrote via BufWriter");

    // BufWriter 自动缓冲（drop 时刷新）
    {
        let file = File::create("buffered_output2.txt").unwrap();
        let mut writer = BufWriter::new(file);
        for i in 0..1000 {
            writeln!(writer, "Line {}", i).unwrap();
        }
        // drop 时自动 flush
    }
    println!("wrote 1000 lines via BufWriter (auto flush on drop)");

    // 组合使用
    let input_file = File::open("Cargo.toml").unwrap();
    let output_file = File::create("copy.toml").unwrap();
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    io::copy(&mut reader, &mut writer).unwrap();
    println!("copied Cargo.toml to copy.toml");
}

// 【问题4】Rust 的命令行参数和标准输入/输出如何处理？
//
// 标准库 std::env 处理命令行参数。
// stdin/stdout/stderr 是标准流。

fn cli_and_streams() {
    // 命令行参数
    let args: Vec<String> = std::env::args().collect();
    println!("args count = {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("  arg[{}] = {}", i, arg);
    }

    // 程序名和单个参数解析
    let prog = std::env::current_exe().unwrap();
    println!("program = {:?}", prog);

    // 标准输出
    print!("print without newline ");
    println!("println with newline");
    eprintln!("stderr output");

    // 标准输入
    print!("Enter something: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    // 注意：在交互环境需要 stdin
    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => println!("You entered: {}", input.trim()),
    //     Err(e) => eprintln!("Error: {}", e),
    // }

    // 格式化输出
    let name = "Alice";
    let age = 30;
    println!("{name} is {age} years old");
    println!("{0} {1} {0}", "Hello", "World");
    println!("{:>10}", "right");
    println!("{:.<10}", "dotted");
    println!("{:?}", (1, 2, "three"));
    println!("{:#?}", vec![1, 2, 3]);

    // 写格式化的字符串
    let formatted = format!("PI ≈ {:.4}", std::f64::consts::PI);
    println!("formatted: {}", formatted);
}

// 【问题5】Rust 的序列化与反序列化 serde 如何使用？
//
// serde 是 Rust 的序列化框架。
// Serialize（序列化）/ Deserialize（反序列化）。
// 支持 JSON, YAML, TOML, Bincode 等格式。

fn serialization_demo() {
    // 使用 serde_json
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Person {
        name: String,
        age: u32,
        emails: Vec<String>,
    }

    let person = Person {
        name: "张三".into(),
        age: 30,
        emails: vec!["zhang@example.com".into(), "personal@example.com".into()],
    };

    // 序列化为 JSON
    let json = serde_json::to_string_pretty(&person).unwrap();
    println!("JSON:\n{}", json);

    // 反序列化
    let parsed: Person = serde_json::from_str(&json).unwrap();
    println!("deserialized: {:?}", parsed);

    // 处理大型结构
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Config {
        app_name: String,
        debug: bool,
        max_connections: u32,
    }

    let config = Config {
        app_name: "MyApp".into(),
        debug: true,
        max_connections: 100,
    };
    let toml_str = toml::to_string(&config).unwrap();
    println!("TOML:\n{}", toml_str);

    // 二进制序列化（更高效）
    let encoded: Vec<u8> = bincode::serialize(&person).unwrap();
    println!("bincode size: {} bytes", encoded.len());

    let decoded: Person = bincode::deserialize(&encoded).unwrap();
    println!("bincode decoded: {:?}", decoded);
}

// 【问题6】Rust 的错误处理（io::Error）如何区分不同错误？
//
// io::ErrorKind 枚举表示不同类型的 I/O 错误：
//   - NotFound：文件不存在
//   - PermissionDenied：权限错误
//   - AlreadyExists：文件已存在
//   - WouldBlock：非阻塞操作会阻塞
//   - 其他：UnexpectedEof, BrokenPipe, ...

fn io_error_handling() {
    fn read_config() -> Result<String, io::Error> {
        let path = Path::new("config.json");
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "config.json not found",
            ));
        }
        fs::read_to_string(path)
    }

    match read_config() {
        Ok(content) => println!("config: {}", content),
        Err(e) => {
            println!("error: {}", e);
            match e.kind() {
                io::ErrorKind::NotFound => println!("  → 文件不存在"),
                io::ErrorKind::PermissionDenied => println!("  → 权限拒绝"),
                io::ErrorKind::AlreadyExists => println!("  → 已存在"),
                _ => println!("  → 其他错误"),
            }
        }
    }

    // 通用的错误传播 ?
    fn read_or_default(path: &str) -> Result<String, io::Error> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    // 遍历目录
    fn list_dir(path: &str) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let is_dir = entry.file_type()?.is_dir();
            println!("{} ({})", file_name.to_string_lossy(), if is_dir { "dir" } else { "file" });
        }
        Ok(())
    }

    if let Err(e) = list_dir(".") {
        eprintln!("list dir error: {}", e);
    }
}

// 【问题7】Rust 的临时文件和目录处理如何实现？
//
// std::env::temp_dir() 返回系统临时目录。
// 可以用 tempfile crate 创建安全的临时文件。

fn temp_files() {
    // 获取临时目录
    let temp_dir = std::env::temp_dir();
    println!("temp dir = {:?}", temp_dir);

    // 手动创建临时文件（避免冲突）
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_path = temp_dir.join(format!("myapp_{}.tmp", timestamp));

    let mut file = File::create(&temp_path).unwrap();
    file.write_all(b"temporary content").unwrap();
    println!("created temp file: {:?}", temp_path);

    // 清理临时文件
    fs::remove_file(&temp_path).unwrap();
    println!("cleaned up temp file");

    // 使用 tempfile crate（更安全）
    // let temp_file = tempfile::NamedTempFile::new().unwrap();
    // temp_file.write_all(b"temp")?;
    // temp_file.keep()?; // 保留文件
    // drop(temp_file); // 自动删除
}

// 【问题8】Rust 的 WalkDir 如何递归遍历目录？
//
// walkdir crate 提供递归目录遍历。
// 包含过滤、深度限制等功能。

fn directory_walking() {
    println!("=== 目录遍历 ===");

    // 手动递归实现
    fn walk_dir_recursive(path: &Path, depth: usize) {
        if depth > 2 { return; } // 限制深度

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let indent = "  ".repeat(depth);
                if entry_path.is_dir() {
                    println!("{}📁 {}", indent, entry_path.display());
                    walk_dir_recursive(&entry_path, depth + 1);
                } else {
                    println!("{}📄 {}", indent, entry_path.display());
                }
            }
        }
    }

    walk_dir_recursive(Path::new("."), 0);

    // 统计文件数量
    fn count_files(path: &Path) -> usize {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    count += count_files(&entry.path());
                } else {
                    count += 1;
                }
            }
        }
        count
    }
    println!("\ntotal files: {}", count_files(Path::new(".")));
}

// ================================================================
// 【对比】Rust vs Python vs Lua vs Go vs C++
// ================================================================
// Rust:
//   - File/BufReader/BufWriter 分层抽象
//   - Read/Write trait 统一接口
//   - serde 序列化框架（需要 derive）
//   - 错误类型明确（io::ErrorKind）
//   - 同步 IO，无内置异步（用 tokio）

// Python:
//   - open() 返回文件对象，统一读写
//   - 内置 JSON 支持（json 模块）
//   - pickle 二进制序列化
//   - pathlib 提供面向对象路径处理

// Lua:
//   - io.open / io.read / io.write 基本 IO
//   - 无缓冲 IO，需要手动缓存
//   - 无原生序列化，用 lua-cjson 或其他库

// Go:
//   - os.Open / os.Create 返回 File
//   - bufio 提供缓冲 IO
//   - encoding/json 原生 JSON 支持
//   - ioutil 简化常见操作

// C++:
//   - fstream / ifstream / ofstream 文件流
//   - iostream 标准输入输出
//   - <fstream> 文件操作
//   - 序列化用第三方库（Boost.Serialization）

fn compare_io() {
    println!("=== 三语言IO对比 ===");

    // 文件读取模式对比
    // Rust: let content = fs::read_to_string("file.txt")?
    // Lua:  local f = io.open("file.txt", "r") local content = f:read("*a") f:close()
    // Python: with open("file.txt", "r") as f: content = f.read()

    // 序列化
    // Rust: serde_json.to_string(&obj)?
    // Python: json.dumps(obj)
}

// ================================================================
// 【练习题】
// ================================================================
// 1. 实现一个文件复制函数，支持大文件（使用 buffered IO，不一次性加载到内存）
// 2. 实现一个配置解析器，支持 JSON/TOML/ENV 三种格式，根据文件扩展名自动选择解析方式
// 3. 遍历当前目录找到所有 .rs 文件并统计总行数（排除空行和注释行）
// 4. 实现一个日志写入器，带缓冲（BufWriter）和日志轮转（每小时新建一个文件）
// 5. 用 WalkDir 实现一个递归目录删除函数（删除空目录和所有文件）

fn main() {
    println!("=== 模块九：IO流与文件 ===");

    path_handling();
    file_io();
    buffered_io();
    cli_and_streams();
    //serialization_demo(); // 需要 serde 依赖
    io_error_handling();
    temp_files();
    directory_walking();
    compare_io();

    println!("\n✅ 所有示例运行成功！");
}