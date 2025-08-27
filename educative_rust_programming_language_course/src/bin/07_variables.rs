fn main() {
    // declare a variable
    let language_0;

    // initialize a variable
    let language_1 = "Rust"; // define a variable
    println!("Language: {}", language_1); // print the variable

    // make a variable mutable
    let mut language_2 = "Rust"; // define a mutable variable using an identifier for
    // a mutable variable
    println!("Language: {}", language_2); // print the variable
    language_2 = "Elixir"; // Update the variable
    println!("Language: {}", language_2); // print the updated value of variable

    // assigning multiple variable
    let (course, category) = ("Rust", "beginner"); // assign multiple
    // values
    println!("This is a {} course in {}.", category, course); // print the value
}
