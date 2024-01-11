// Options
// They are used to declare data type of our own

pub mod helper;
fn main() {
    let result: Option<u8> = helper::helper_option();
    println!("{}", result.unwrap());

    let result_2: Option<String> = helper::helper_option_string();
    println!("{}", result_2.unwrap());

    let result_3 = helper::helper_option_character();
    // println("{}", result_3.unwrap());
    // This will give error because the rust compiler doesn't know what to do with the words in the enum
    // SO we have to implement the to string method

    if result_3.is_some() {
        println!("{}", result_3.unwrap().to_string());
    } else {
        println!("Character Type is None.");
    }
}
