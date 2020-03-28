pub fn run() {
    println!("Hello, world!");

    // Formatting
    println!("{}", 123);
    // Basic formatting
    println!("{} likes {}", "ykevin", "coding");
    // Positional Arguments
    println!(
        "{0} is from {1}; {0} likes {2}",
        "ykevin", "China", "coding"
    );
    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "ykevin",
        activity = "Baseball",
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "there"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
