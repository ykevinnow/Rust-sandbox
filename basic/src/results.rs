// 

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
use std::fs::File;

pub fn run() {
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };
}