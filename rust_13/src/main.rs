// Match (Switch)
// In rust we have match in place of switch statements
// We have to have all the possible combinations in match statement
fn main() {
    let age: u32 = 21;
    let y: u8 = 10;

    // If we pass the signed data type then we have to include the negatives also
    match age {
        // We can use or statement by single pipe character
        0 | 1 => {
            println!("You are infant!");
        }

        // This means between 12 and 20 including 20
        2..=20 => {
            println!("You are not old enough");
        }

        // We can also if else condition in a case
        21 if y == 10 => println!("You are eligible to drive and y is 10"),
        21 if y != 10 => println!("You are eligible to drive and y is not 10"),
        // We also have to include all the possibilities of the if statement and then a default case also
        21 => println!("You are eligible to drive"),

        // We can add range element also
        22.. => {
            println!("You are old enough");
        } // _ is used as a default specifier
          // We dont need default now
          // _ => {
          //     println!("You are not eligible");
          // }
    }

    // String match
    let mobile_brand = "Vivo";
    match mobile_brand {
        "Vivo" => println!("You have a vivo mobile"),
        "Redmi" => println!("You have a redmi phone"),
        "Samsung" => println!("You have a samsung phone"),
        "One Plus" => println!("You have one plus phone"),
        _ => println!("You dont have a mobile"),
    }

    // Array Match
    let name_arr = ["Lakshay", "Rahul", "Sohan", "Mohan"];
    match name_arr[1..=3] {
        ["Lakshay", "Rahul"] => println!("You are allowed to enter"),
        ["Rahul", ..] => println!("You dont have the permit"), //This will run
        ["Mohan", "Sohan"] => println!("You are not allowed to enter"),
        _ => println!("Enter valid name"),
    };
}
