// This is the first rust code 
// From here i have started rust language 
fn main(){
// This is the main function from where the compiler start the program

    println!("Hello Lakshay");
    print!("Namaste Bharat  ");//This will not print next line
    println!("Namaste Bharat");//This will not print next line

    println!("{} This is stringified",20);

// This is how we declare a variable 
// First the name then the type and then the value
    let a:i32 = 10;
    println!("Value of X is: {}",a);

// Rust also has a scope fundamental
    {
        let y:i32= 20;
        println!("{} Value of y inside the scope ",y);
    }
    // This will throw an error
    // println!("{} Value of y outside the scope ",y);
    

}