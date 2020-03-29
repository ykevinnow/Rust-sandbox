// 
use std::thread;

pub fn fun() {
    let mut c = vec![];

    for i in 0..10 {
        c.push(thread::spawn(move || {
            println!("thread number: {}", i);
        }));
    }

    for j in c {
        j.join();
    }
}