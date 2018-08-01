fn main() {

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    
    // println!("s = '{}'", s)      // This will raise an error:

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    // println!("x = {}", x)        // This won't raise an error:

    let s1 = gives_ownership();         // gives_ownership moves its return
    println!("This is s1: {:?}", s1);
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope
    println!("This is s2: {:?}", s2);

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    println!("This is s3: {:?}", s3);
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    let s4 = String::from("goodbye");       // s4 comes into scope

    let (s5, len) = calculate_length(s4);   // s5 holds the value of s4
                                            // (which goes out of scope)

    println!("The length of '{}' is {}.", s5, len);
    println!("Does it have more than one letter? {}.", bigger_than_one(len));

    // println!("len = {}", len)            // This won't raise an error:

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.
  // Here, both s5 go and len go out of scope. Though len was passed to a function,
  // since it has trait Copy, it remained in scope until this point.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn bigger_than_one(s: usize) -> bool { // s comes into scope
    s > 1
}
