#[allow(dead_code)]
mod basic;

// don't compile at all in non-linux platform
// this target_os attribut can be very useful in command line scripts
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("running linux.");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("not running linux.");
}

fn main() {
    are_you_on_linux();

    println!("check os again");
    if cfg!(target_os = "linux") {
        println!("definitely linux");
    } else {
        println!("not linux");
    }
}
