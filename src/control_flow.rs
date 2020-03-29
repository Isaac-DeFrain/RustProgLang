// Control Flow
#![allow(overlapping_patterns)]

// If statement
// if expression syntax:
// if bool_exp { true_branch_exp }
// if bool_exp { true_branch_exp } else { false_branch_exp }
// if bool_exp1 { bool_exp1_true_branch_exp } else if bool_exp2 { bool_exp2_true_branch_exp }

fn if_statement() {
    println!("----- 3.1. If statement -----");
    let temp = -5;
    let weather = if temp > 30 {"hot"} else if temp < 10 {"cold"} else {"nice"};
    println!("the weather is {}", weather);

    println!("{}",
             if temp < 10 {
                 if temp < 0 {"super cold!"} else {"cold"}
             } else if temp > 20 { if temp > 30 {"super hot!"} else {"hot"}
             } else {"nice"})
}

// While and Loop
fn while_and_loop() {
    println!("----- 3.2. While and loop -----");

    // while
    let mut x = 1;
    while x < 1000
    {
        x *= 2;
        if x == 64 { continue; } // goes back to top of while
        println!("x = {}", x);
    }

    // loop
    let mut y = 1;
    loop { // while true {...}
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break; }
    }
}

// For Loops
fn for_loop() {
    println!("----- 3.3. For loops -----");

    let mut y = 1;
    for x in 1..11 { // exclusive on right
        y *= 2;
        println!("x = {}", x); // can break and continue like in while & loop
    }
    println!("y = {}", y);

    for (i, x) in (30..40).enumerate() {
        println!("({}, {})", i, x);
    }
}

// Match statements
fn match_stmt(country_code:i32) {
    println!("----- 3.4. Match statements -----");

    println!("The country is with code {} is {}",
        country_code,
        match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1 => "USA",
        1..=999 => "unknown", // matches if in given inclusive range
        _ => "invalid"
        });
}

pub fn main() {
    println!("----- 3. Control Flow -----");
    if_statement();
    while_and_loop();
    for_loop();
    match_stmt(666);
}