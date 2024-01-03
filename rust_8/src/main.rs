use std::io;
fn main() {
    // Operators in rust
    // <,>,<=,>=,!=,==

    // We can put logical statement in a varialble and then print whether it is true or false
    // We have to use the same type in left and right hand side of the operator
    let cond = 2 < 3;

    // These logical operators are same as java && || !
    let cond2 = cond && true;

    println!("{}", cond);
    println!("{}", cond2);

    let mut var_1 = String::new();
    io::stdin()
        .read_line(&mut var_1)
        .expect("Enter Valid String");
    // This input also get a space in it so if we have to equal to a particular string we have to trim it
    print!("{}", var_1);
    let var_1 = var_1.trim();
    if var_1 == "Lakshay" {
        println!("Hello Lakshay");
    } else if var_1 == "Naman" {
        println!("Hello Naman");
    } else {
        println!("Who are you");
    }
}
