// Modules and loops
pub mod helpers;
// We can declare multiple modules in rust
// For that we first have to make the function pub
// Then we import it in the main file
// In default every file is a private

// To make child module we declare the function in another function and then we can first pass the parent then the child
fn main() {
    let my_inp = &mut String::from("");
    println!("Enter a value: ");
    // We can also take input in this way
    std::io::stdin().read_line(my_inp).expect("Okay");
    println!("{}", my_inp);

    println!("{}", helpers::parent::get_full_name("Lakshay", "Goel"));
    println!("{}", helpers::parent2::get_sum(2, 4));

    // This will give error because it is private function
    // println!("{}", helpers::parent2::get_divide(2, 4));

    // We can use the #[allow(dead_code)]
    // This will not allow the compiler to give warning if we have unused code
    // It will only work for a particular part of code
    // For eg. If there is a function and we have written the allow attribute then it will not give warning for that function only

    // In rust we have only three types of loop
    // 1. loop
    // 2. while
    // 3. for
    let mut c = 0;
    // we can't write any condition after the word loop
    loop {
        println!("Okay😂");
        c += 1;
        if c == 10 {
            break;
        }
    }

    let mut a = 1;
    while a < 10 {
        println!("You are in while loop.");
        a += 1;
    }
    println!("YOu are outside the while loop");

    let age: [i32; 5] = [23, 12, 15, 18, 43];
    let age_to_drive: i32 = 18;
    for value in age {
        if value >= age_to_drive {
            println!("You can drive");
        }
    }
}
