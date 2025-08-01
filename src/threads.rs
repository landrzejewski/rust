use std::sync::{mpsc, Arc, Barrier, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn channels() {
    let (tx, rx) = mpsc::channel();
    let txl = tx.clone();
    thread::spawn(move || {
        let numbers = vec!["1", "2", "3"];
        for number in numbers {
            _ = &txl.send(number);
            thread::sleep(Duration::from_secs(1));
        }
    });
    _ = tx.send("abc");
    for number in rx {
        // infinite loop due to use/existence of tx references
        println!("New message: {number}");
    }
    // rx.iter()
    //    .map(fn)
}

fn barrier() {
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(10);
    let atomic_barrier = Arc::new(Barrier::new(10));
    for _ in 1..=10 {
        let barrier = Arc::clone(&atomic_barrier);
        handles.push(thread::spawn(move || {
            println!("Before wait ({:?})", thread::current());
            barrier.wait();
            println!("After wait ({:?})", thread::current());
        }));
    }
    for handle in handles {
        _ = handle.join();
    }
}

fn rw_lock() {
    let lock = RwLock::new(5);
    if let Ok(mut val) = lock.write() {
        *val = 6;
    }
    if let Ok(val) = lock.read() {
        println!("{:?}", *val);
    }
}

fn mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..150 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            if let Ok(mut val) = counter.lock() {
                *val += 1;
            }
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter: {}", counter.lock().unwrap());
}

fn threads() {
    let numbers = Arc::new(vec![1, 2, 3, 4]);
    let numbers_copy = numbers.clone();

    let handle = thread::Builder::new()
        .name("Thread 1".to_string())
        /*.spawn(move || {
            println!("{:?}", numbers);
            for  index in 1..10 {
                println!("Index {}, (thread: {:?})", index, thread::current().name().unwrap());
                thread::sleep(Duration::from_secs(1));
            }
            String::from("Success")
        }).unwrap();*/
        //.spawn(move || task(&numbers))
        .spawn(move || numbers)
        .unwrap();

    let result = handle.join().unwrap();
    println!(
        "Result: {:?} (thread: {:?})",
        result,
        thread::current().name().unwrap()
    );
    println!(
        "Number: {:?} (thread: {:?})",
        numbers_copy,
        thread::current().name().unwrap()
    );
    /*thread::spawn(|| {
       loop {
            println!("Hello");
       }
    });
    thread::sleep(Duration::from_secs(60));*/
}

/*fn task(numbers: &Vec<i32>) -> String {
    println!("{:?}", numbers);
    for  index in 1..10 {
        println!("Index {}, (thread: {:?})", index, thread::current().name().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
    String::from("Success")
}*/

fn task(numbers: Arc<Vec<i32>>) -> String {
    println!("{:?}", numbers);
    for index in 1..10 {
        println!(
            "Index {}, (thread: {:?})",
            index,
            thread::current().name().unwrap()
        );
        thread::sleep(Duration::from_secs(1));
    }
    String::from("Success")
}
