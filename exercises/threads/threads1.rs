use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    
    // 创建10个线程
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250)); // 模拟线程工作
            println!("thread {} is complete", i);
            start.elapsed().as_millis() // 返回线程运行的时间
        }));
    }

    let mut results: Vec<u128> = vec![];
    
    // 收集线程返回值
    for handle in handles {
        // 调用 join() 方法来获取每个线程的返回值
        let result = handle.join().expect("Thread panicked"); // 处理线程中的 panic
        results.push(result); // 将返回值存入 results 向量
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    
    // 输出每个线程的执行时间
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
