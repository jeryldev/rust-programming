/// This is a Doc comment outside the function
/// Generate docs for the following item.
/// This shows my code outside a module or a function
fn main() {
    //! This is a doc comment that is inside the function
    //! This comment shows my code inside a module or a function
    //! Generate docs for the enclosing item
    println!("{} can support {} notation", "Doc comment", "markdown");

    // The line comment is the recommended comment style
    println!("hello World"); // Print hello World to the screen

    /*
    The block comment is extremely useful for temporary disabling
    a large chunk of code. /* Block comments can also be /* nested, */ */
    To comment a large block just write in between /* text */
    */
    println!("hello World"); // Print hello World to the screen
}
