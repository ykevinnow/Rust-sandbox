//
use std::thread;

pub fn run() {
    let v = vec![1, 2, 3];

    // the 'move' keyword makes data from one thread be used in another thread
    let handle = thread::spawn(move || println!("vector: {:?}", v));
    
    // don't work, coz v has been moved
    // println!("{:?}", v);

    handle.join();
}
