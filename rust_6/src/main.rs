// Input and output
use std::io;
fn main() {
    // This declares an empty variable of string format and name input that is mutable
    let mut input = String::new();

    // Then we use the imported library and we use the real line function of it and then put it in the address of the input variable
    // Here we have again used mut because the input address is immutable so to change it we have to make it mut
    // Then we use expect function that acts like a catch varible and then we enters what to print after it catches an error
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    println!("{}", input)
}
