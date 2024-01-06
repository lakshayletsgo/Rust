use std::io;
// Here i have faced a problem i pass same variable of int to same function and it is showing no error
// While when i pass a string to it, it is showing errror

fn main() {
    let x: i32 = 21;
    let y: i32 = 12;
    println!("Hello, world!");
    test();
    sum(x, y);
    division(x, y, 23.2);

    let s: String = "Lakshay".to_string();
    // display(s);
    display(s);
    display2(x);
    display2(x);
    display2(y);

    // The declaration statement doesn't return a statement so we cant put it inside another variable
    // Just like in python we can x=y=4;

    // let c = (let b=10);
    // Error

    // This is expression
    let var_2 = 4 > 3;
    println!("{}", var_2);

    // We can put multiple expression inside a variable
    // We have put an expression inside a block, the block should return something otherwise it won't work
    // Here we have returned the value x+1 as we have not put the semicolon at the end of x because we are returning it
    let var_3 = {
        let x = 2;
        x + 1
    };
    println!("{}", var_3);

    let var_4 = {
        let mut s = String::new();
        let y: i32;
        let x: i32;

        io::stdin().read_line(&mut s).expect("Invalid input");
        y = s.trim().parse().unwrap();
        let mut s2 = String::new();
        io::stdin().read_line(&mut s2).expect("Invalid input");
        x = s2.trim().parse().unwrap();

        y + x
    };
    println!("{}", var_4);

    println!("{}", add_number(32, 43));
}

// We can't pass two same variable to different functions
fn test() {
    println!("This is first function with no parameter");
}
fn display(s: String) {
    println!("{}", s);
}
fn display2(s: i32) {
    println!("{}", s);
}

fn sum(x: i32, y: i32) {
    println!("Sum of two numbers is : {}", x + y);
}

// We can pass different type of variable to the same function
fn division(x: i32, y: i32, z: f32) {
    println!("Division of two number is : {}", (x / y) as f32 + z);
}

// Returning function
// We have to not use semicolon if we don't use return word else we can use return x+y;
fn add_number(x: i32, y: i32) -> i32 {
    x + y
}
