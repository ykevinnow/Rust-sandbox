// 

// from std::Option
// enum Option<T> {
//     Some(T),
//     None,
// }

fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

pub fn run() {
    let res = division(5.0, 7.0);
    match res {
        Some(x) => println!("{:.10}", x),
        None => println!("Cannot divide by 0."),
    }
}
