// 
use std::sync::{Mutex, Arc};
use std::thread;

pub fn run() {
    // Arc - atomically reference counted, thread-safe,
    // the type Arc<T> provides a shared ownership of a value of type T
    let c = Arc::new(Mutex::new(5));
    let mut hs = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&c);
        let h = thread::spawn(move || {
            // nm is a smart pointer
            let mut nm = c.lock().unwrap();
            // so we need to derefer nm
            *nm += 1;
            println!("nm is {}", nm);
        });
        hs.push(h);
    }

    for h in hs {
        // for unlock Result
        h.join().unwrap();
    }

    println!("Result: {}", *c.lock().unwrap());
}