#[allow(unused_variables)]
fn main() {
    let a = 10; //This variable is not used and this will generate warning
                // To stop this we can either declare it with _ to remove the unused variable warning for this one
                // To remove this permentally we can write #[allow(unused_variables)] at the begining

    let _y = 21;

    // We can also destructure the tuple and then store it in variables
    // let ( x,  y) = (1, 2);  In this we have both variables as immutables
    let (mut x, mut y) = (1, 2);
    x += 2;
    y += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 4);

    // let x2: i32 = 5;
    // print!("{} type of", type_of(&x2));
    //  I can't use the type of function and don't know the reason
    // assert_eq!("u32".to_string(), type_of(&x2));
    println!("{} max of i8", i8::MAX);
    println!("{} max of i16", i16::MAX);
    println!("{} max of i32", i32::MAX);
    println!("{} max of i64", i64::MAX);

    // We can also type cast like this

    let m: i16 = 31_i16;
    println!("{}", m);

    // We can create array with this also
    let new_ages: [i32; 5] = [2, 8, 1, 6, 3];
    println!("{:?}", new_ages);

    let new_ages2: &[i32] = &new_ages[1..4];
    // In this the 4 index is excluded
    println!("{:?}", new_ages2);

    let new_ages3: &[i32] = &new_ages[1..=4];
    // This will include the last element also
    println!("{:?}", new_ages3);
}
