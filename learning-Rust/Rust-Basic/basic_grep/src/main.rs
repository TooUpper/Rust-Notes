use std::env;
use std::process;

use basic_grep::Config;
use basic_grep::run;

fn main() {

    // env::args(): 获取命令行参数的迭代器。这里返回了一个实现了 Iterator trait 的struct
    // "target\\debug\\basic_grep.exe",  "addr", "对的", 第一个是程序的可执行路径，从第二个开始就是我们传入的参数  

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("参数解析时出现错误: {err}");
        process::exit(1);
    });

    println!("查询的内容 {}", config.query);
    println!("所查询的文件 {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("参数读取错误: {e}");
        process::exit(1);
    }
}
