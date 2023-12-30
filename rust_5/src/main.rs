// DataTypes
fn main() {
    // There are two types of variables 1. scalar and 2. compound
    // Scalar are basic 1 element type variable like int char boolean and float
    // Compound is coplex element likes tuple and arrays

    // Scalar
    let var_1: i8 = 100;
    let var_2: i16 = 1000;
    let var_3: i32 = 10000; //This is the default type if we dont define it
    let var_4: i64 = 100000;
    let var_5: i128 = 1000000;
    println!("{0}\n{1}\n{2}\n{3}\n{4}", var_1, var_2, var_3, var_4, var_5);

    // We also have unsigned integer that only takes positive values
    let var_6: u8 = 212;
    println!("{}", var_6);

    // We have floating point number also
    // By default we have f64 as floating number
    let var_7: f32 = 21.3;
    println!("{}", var_7);

    // Boolean is just true and false value
    let var_8: bool = true;
    let var_9: bool = false;
    println!("{0}\n{1}", var_8, var_9);

    // We also have a character type of data
    let var_10: char = 'a';
    println!("{}", var_10);

    // Compound
    // Tuple is a fixed size of element array and it is immutable

    // We dont use square brackets to get the element we use the .
    // We can make it mutable by adding mut keyword in the declaration statement
    let var_11: (i32, bool, char) = (1, true, 's');
    println!("{}", var_11.1);

    let var_12: (i32, bool, f32) = (1, false, 2.4);
    // let mut var_12: (i32, bool, f32) = (1, false, 2.4);
    // var_12 = (12, true, 2.3);
    // We cant add more element we can just rewrite the exiting one
    println!("{}{}{}", var_12.0, var_12.1, var_12.2);

    let var_13: [i32; 4] = [1, 3, 5, 6];
    println!("{}", var_13[1]);
    // We can't intialise it empty like in other languages and access it

    let var_14: i32 = 23;
    // let var_15:i64 = var_14;//This will give error
    let var_15: i32 = var_14;
    println!("{}", var_15)
}
