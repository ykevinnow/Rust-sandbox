//
use std::fs::File;
// use std::io::ErrorKind;

use std::io;
use std::io::Read;

/*
fn open_file() -> Result<File, io::Error> {
    let f = File::open("text.txt");
    // unwrap or expect
    // let f = File::open("text.txt").expect("could not open file.");

    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("test.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("could not create file: {:?}", e),
        },
        Err(error) => {
            panic!("could not open the file: {:?}", error);
        }
    };
}
*/

// keep in mind: '?' can not be used in main() function
fn read_file() -> Result<String, io::Error> {
    // 1st version
    let f = File::open("text.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

    // 2nd version
    // let f = File::open("text.txt")?;
    // let mut s = String::new(s);
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // 3rd version
    // let mut s = String::new();
    // File::open("text.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}

pub fn run() {
    println!("{:?}", read_file());
}
