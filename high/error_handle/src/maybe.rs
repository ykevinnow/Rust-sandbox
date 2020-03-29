//
// enum Result<T， E> {
//     Ok(T),
//     Err(E)
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

fn exit(x: Option<i32>) {
    match x {
        Some(0) => panic!("we got a 0"),
        Some(x) => println!("we got a {} things are fine!", x),
        None => println!("we got nothing."),
    }
}

pub fn run() {
    exit(Some(1));
    exit(Some(10));
    exit(None);
    // trigger the panic
    // exit(Some(0));
}
