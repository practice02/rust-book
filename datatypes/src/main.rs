fn main() {
    let x = 2.0; // f64
    println!("x = {}", x);

    let y: f32 = 3.0; // f32
    println!("y = {}", y);

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // Booleans
    let t = true;
    println!("It's all {}", t);
    let f: bool = false; // with explicit type annotation
    println!("I knew it was {}", f);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("(x, y, z) = {:?}", tup);
    println!("\tx = {}\n\ty = {}\n\tz = {}", x, y, z);

    println!("\ttup.0 = {}\n\ttup.1 = {}\n\ttup.2 = {}", tup.0, tup.1, tup.2);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    println!("The first element of a is {} (a[0])", a[0]);
    let months = ["January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"];
    println!("The third month is {}", months[2]);
}
