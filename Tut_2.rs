// Different Formatting Types
fn main(){
    println!("{} is the decimal of 20",20);
    println!("{:b} is the binary of 20",20);
    println!("{:o} is the octal of 20",20);
    println!("{:x} is the hexadecimal of 20",20);


    // This will add spaces to the left of 10
    println!("{number:>5}",number=10);

    // This will add 5 zeros to the right of 10
    println!("{number:0<5}",number=10);

    // This will add 5 zeros to the left of 10
    println!("{number:0>5}",number=10);
    println!("{number:9>5}",number=10);

    // This is dynamic allocation of the width that is given
    // Note that it counts the number also for the zeros or the provided number
    println!("{number:0>width$}",number=1000,width=30);
    
    // In this we can specify the dynamic memory by providing the index of the values
    println!("{0}, {1} {0}","Lakshay","Rahul");


    let x2:f32 = 50.0;
    let x3:usize = 21;
    println!("{x2:2<x3$}");//502222222222222222222   
}