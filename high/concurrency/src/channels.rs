use std::thread;
// mpsc - multi-producer, single-consumer
// https://doc.rust-lang.org/std/sync/mpsc/
use std::sync::mpsc;

pub fn run() {
    // destruct a tuple
    let (tx, rx) = mpsc::channel();

    // unwrap the Result
    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    // recv() is blocking, try_recv() is non-blocking
    println!("got {}", rx.recv().unwrap());
}
