use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;

fn start() {
    let mut child = Command::new("tail")
        .arg("-f")
        .arg("/Users/fuhui/file.txt")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child process");

    let pid = child.id();
    println!("process id {}", pid);

    let stdout = child.stdout.take().expect("child did not have a stdout");
    let reader = BufReader::new(stdout);

    // 创建一个线程，专门处理输出
    let h = thread::spawn(move || {
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    });

    // 主线程继续执行其他任务，例如：
    h.join().unwrap();
    println!("子进程正在运行，输出结果将在另一个线程中显示");
}