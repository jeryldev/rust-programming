fn main() {
    println!("Basic formatting");
    // Single placeholder
    println!("Number: {}", 1);
    // Multiple placeholder
    println!("{} is a {} company", "Educative", "Software");
    // Positional arguments
    println!(
        "Enhance your coding skills from {0} courses. {0} courses are very {1}",
        "Educative", "interactive"
    );
    // Named arguments
    println!(
        "{company} provides {kind} courses.",
        company = "Educative",
        kind = "interactive"
    );
    // Placeholder traits
    println!(
        "Number: 10\nBinary: {:b} Hexadecimal: {:x} Octal: {:o}",
        10, 10, 10
    );
    // Basic math
    println!("{} + {} = {}", 10, 10, 10 + 10);
    // Placeholder for a debug trait
    println!("{:?}", ("This is a Rust Course", 101))
}
