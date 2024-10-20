use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> Arc<Mutex<u32>> {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx1 = tx.clone();
    let tx2 = tx;

    let counter = Arc::new(Mutex::new(0u32));
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        let mut num = counter1.lock().unwrap();
        *num += 1;
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        let mut num = counter2.lock().unwrap();
        *num += 1;
    });

    counter
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let counter = send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    // Wait for both threads to finish
    while *counter.lock().unwrap() < 2 {
        thread::sleep(Duration::from_millis(100));
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}