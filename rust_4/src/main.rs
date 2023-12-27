// Basic variable and mutablity
fn main() {
    let x= 4;
    println!("{}",x);
    // This is immutable variable i.e. we can't change this variable and we have to declare new variable 
    // We can redeclare variable in rust
    let x = 5;
    println!("{}",x);

    // To make a variable mutable we have to add mut to the declaration of the variable 
    let mut y = 4;
    println!("{}",y);
    y = 1;
    println!("{}",y);


    // We can either declare a type of variable explicitly or implicitly
    // To declare explicitly we have to add the data type of the variable
    let n : i8 = 10;
    // This will declare a variable name n with 8 bytes integer

    println!("{}",n);


    let n1 = 10;
    let n1 = n1+1;
    println!("{}",n1);//11



    // It is used to define an immutable variable 
    // We cant redeclare or change a const 
    // And we have to give the type explicitly
    const SECOND_IN_MINUTE: i32 = 12;
    println!("Value is: {}",SECOND_IN_MINUTE);
    // const SECOND_IN_MINUTE :i32 = 11;
    println!("Value is: {}",SECOND_IN_MINUTE);
}
