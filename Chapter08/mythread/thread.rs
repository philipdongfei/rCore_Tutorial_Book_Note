use std::thread;

fn mythread(args: i64) -> i64 {
    println!("thread: {}", args);
    return args + 1;
}

fn main() {
    let handle = thread::spawn(|| mythread(100));
    let rvalue = handle.join().unwrap();
    println!("main: {}", rvalue);
}

