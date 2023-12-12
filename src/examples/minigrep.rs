use std::{env, process};

use hello_world::minigrep::{self, Config};

fn main() {
    // env::Args 实现了 Iterator 特征，所以 env::args() 返回的实际上是一个迭代器
    let args: env::Args = env::args();

    let config = Config::build_config(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        // 当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程
        process::exit(1);
    });

    println!("Searching for [{}] In file [{}]", config.key, config.file);

    // 只匹配run函数返回的错误，因为我们不关心Ok值
    if let Err(error) = minigrep::run(config) {
        eprintln!("Failed to run application: {error}");
        process::exit(1);
    }
}
