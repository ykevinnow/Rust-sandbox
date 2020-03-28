// 

pub fn run() {
    let mut age = Some(0);

    loop {
        match age {
            Some(i) => if i > 19 {
                println!("Quit");
                age = None;
            } else {
                println!("{}", i);
                age = Some(i + 2);
            },
            _ => {
                break;
            }
        }
    }

    while let Some(i) = age {
        if i > 19 {
            println!("Quit");
            age = None;
        } else {
            println!("{}", i);
            age = Some(i + 2);
        }
    }
}