struct Car {
    color: String,
    year: i32,
}
pub fn test_closures() {
    let closure_called = || println!("Closures is called!!");
    // This closure don't accept any argument we can also make a arguement closure
    closure_called();

    let add = |x: i8, y: i8| println!("Sum of {0} and {1} is {2}", x, y, x + y);
    add(2, 3);

    // A big advantange of closure over normal function is that we dont have to specify the data type in the closure
    // It will take the data types on the basis of the input passed when the function is called
    // This will take the input from the first call
    let diff = |x, y| x - y;
    // And to get a return type we just pass the value at the end
    diff(5, 2);
    // println!("{}", diff(3.4, 1.0));

    let multi = |x, y| {
        println!("Product of {} and {} is {}", x, y, x * y);
        x * y
    };
    let result = multi(2, 1);
    // We can use the variable in closures that are declared in parent
    let square = || println!("Square of product is {}", result * result);
    square();

    // Mutable closure
    // These are used to mutate a variable
    // For this we first define a struct
    let mut car1 = Car {
        color: "black".to_string(),
        year: 2001,
    };

    // To change a year we first have to declare the object as a mutable and then we have to make the closure mutable also
    let mut change_year = |new_year| car1.year = new_year;
    change_year(2001);
    println!(
        "{} is the color of car and is of year {}",
        car1.color, car1.year
    );

    // If we again pass this it will throw an error because we are again borrowing the variable and mutating it
    // We have to print the variable at the very end
    // change_year(2001);
}
