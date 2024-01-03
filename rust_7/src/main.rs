use std::io;

fn main() {
    let var1 = 100;
    let var2 = 2;
    println!("{}", var1 / var2);
    println!("{}", var1 + var2);
    println!("{}", var1 - var2);
    println!("{}", var1 * var2);
    println!("{}", var1 % var2);

    // We can only perform arithematic operation on the same type of variable

    let var3 = 100 as i64; //This is type casting
    let var4 = 1000_i64; //This is also type casting
    let var5 = 10_00i64; //This is also type casting
    let var6 = 10i64;

    // We can type casting at the time of operation only

    println!("{}", var3 * var4);
    println!("{}", var5 / var6);
    println!("{}", var5 / var6 as i64);

    // Converting String to Integer type

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected Line");
    // This is just input

    let int_input: i32 = input.trim().parse().unwrap();
    // This first intialises a variable of int type then take the string as a value and then remove the spaces from end and begining and the parse it in the int and then unwrap function is used just like expect in input

    print!("{}", input);
    print!("{}", int_input + 6)
}
