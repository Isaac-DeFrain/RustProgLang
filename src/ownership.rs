// Ownership in Rust

pub fn main() {
    println!("----- Ownership -----");

    {
        let s1 = String::from("hello"); // s1 comes into scope
        let s2 = s1.clone();              // s2 comes into scope
        let s3 = s1.clone();              // s3 comes into scope

        takes_ownership(s1);           // s1's value moves into the function...
        // ... and so is no longer valid here
        //println!("s1 = {} is still valid", s1); // throws a compile-time error

        takes_ownership(s2);           // s2's value moves into the function...
        // ... and so is no longer valid here
        //println!("s2 = {} is still valid", s2); // throws a compile-time error
        println!("s3 = {} is still valid", s3);

        let x = 5;                           // x comes into scope
        makes_copy(x);                // x would move into the function,
        // but i32 is Copy, so it’s okay to still
        // use x afterward
        println!("x = {} is still valid", x);
    } // Here, x goes out of scope, then s3, s2, and s1.
    // But because s1 and s2's values were moved, nothing special happens.

    {
        let s4 = String::from("hello");                   // s4 comes into scope

        let (s5, len) = calculate_length(s4);       // s4 is moved into calculate_length
                                                                  // the return values are moved into s5 and len

        println!("The length of '{}' is {}.", s5, len);           // s5 and len are still valid, s4 is not
    }
    borrowing_with_mutation();
    string_slices();
    println!("All variables are out of scope now.")
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string.clone());
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn borrowing_with_mutation() {
    // Variables and references are immutable by default.
    // let s = String::from("hello");
    // change(&s);
    // fn change(some_string: &String) {
    //     some_string.push_str(", world");
    // }

    let mut s1 = String::from("hello");
    change1(&mut s1);
    fn change1(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let s2 = String::from("hello");
    change(s2);
    fn change(mut some_string: String) {
        some_string.push_str(", world");
    }
}

fn string_slices() {
    let mut s = String::from("hello world");
    let word = &s[..5]; // immutable borrow
    //s.clear();           // error! -- mutable borrow
    println!("the first word is: {}", word);
    //println!("the first word is: {}", *word); // error -- *word has unknown size at compile-time
    s.clear();
}