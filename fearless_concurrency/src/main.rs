use std::{
    process,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
    // Main thread completes. Spawned thread just exit.

    // In case you want threads to complete, Join.
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // Value cannot be borrowed inside thread. The value may be dropped before the thread complete.
    // Uncomment to see errors
    // thread::spawn(|| {
    //     for i in v {
    //         println!("{i}");
    //     }
    // })

    // Use move to make thread take ownership and drop the value.
    let another_handle = thread::spawn(move || {
        for i in v {
            println!("{i}");
        }
    });

    another_handle.join().unwrap();

    // v cannot be used after the thread is spawned.
    // mpsc is used to communicate between threads.
    let (tx, rx) = mpsc::channel(); // tx = transmitter, rx = receiver.
    thread::spawn(move || {
        let val = String::from("hi");
        // When you send the value, you also move it out.
        if let Err(er) = tx.send(val) {
            eprintln!("error: {er}");
            process::exit(1);
        }
    });

    let received = rx.recv().unwrap();
    println!("Got from channel: {received}");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("Spawned"),
            String::from("Thread"),
        ];
        for v in val {
            tx.send(v).unwrap();
        }
    });
    // rx can be used as iterators.
    for v in rx {
        println!("Got {v} from rx");
    }

    // Mutex == Mutual exclusive
    // It allows only one thread to access a data at a time.
    let m = Mutex::new(9);
    {
        let mut val_from_mutex = m.lock().unwrap();
        *val_from_mutex += 1;
    }
    println!("m = {:?}", m);

    // In Rust, only one piece can own a value. In the previous lession, we use Rc. However, Rc is
    // not thread-safe. It is not safe to share Rc between different thread. We will use Arc
    // instead.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("Increase from thread {i}");
            *num += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    // About send and sync trait, please read the book.
}
