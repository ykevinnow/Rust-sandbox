// conditional compilation
#[cfg(test)]
mod my_test {
    #[test]
    fn it_works() {}

    #[test]
    fn check_two() {
        assert!(1 + 1 == 2);
    }

    // attribut: should_panic
    #[test]
    #[should_panic]
    fn test_three() {
        assert!(3 == 4);
    }
}

// advanced usage
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

#[cfg(test)]
mod call_external_fun {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, internal_adder(3, 1));
        assert_eq!(7, add_two(5));
    }

    #[test]
    #[should_panic]
    fn another() {
        assert!(true == false);
    }

    // we can just run this test with "cargo test greeting_contains_name"
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Annabell");
        assert!(result.contains("Annabell"));
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        );
    }
}
