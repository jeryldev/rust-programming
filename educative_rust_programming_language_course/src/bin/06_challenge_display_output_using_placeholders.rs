fn main() {
    test();
}

/// Problem statement
/// The task is to display the pyramid of numbers in the following format
/// Output
/// 1
/// 22
/// 333
/// 4444
/// 55555
fn test() {
    println!("{}", 1);
    println!("{}{}", 2, 2);
    println!("{}{}{}", 3, 3, 3);
    println!("{}{}{}{}", 4, 4, 4, 4);
    println!("{}{}{}{}{}", 5, 5, 5, 5, 5);
}
