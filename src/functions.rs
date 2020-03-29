// Functions
#![allow(unused_variables)]

// Functions and Function Arguments
fn functions_and_args() {
    println!("----- 5.1. Functions and arguments -----");

    fn print_value(x:i32) {
        println!("value = {}", x)
    }
    print_value(42);

    fn increase(x:&mut i32) { // input: mutable reference
        *x += 1; // dereference and increase value by 1
    }
    let mut y = 1;
    increase(&mut y);
    println!("y = {}", y);
    println!("&y = {}", &y);
}

// Methods
use std::fmt;

fn methods() {
    println!("----- 5.2. Methods -----");

    // method = function attached to a specific data type

    struct Point {
        x: f64,
        y: f64,
        z: f64
    }

    // implementing display trait for Point struct
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }

    let p1 = Point {x: 3.14, y: 2.71, z: 1.41};
    println!("p1 has coordinates {}", p1);

    let p2 = Point {x: -3.14, y: 27.1, z: -14.3};
    println!("p2 has coordinates {}", p2);

    struct Line {
        start: Point,
        end: Point
    }

    // implementing display trait for Line struct
    impl fmt::Display for Line {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}, {}]", self.start, self.end)
        }
    }

    impl Line {
        fn len(&self) -> f64 {
            let dx = self.start.x - self.end.x;
            let dy = self.start.y - self.end.y;
            let dz = self.start.z - self.end.z;
            (dx*dx + dy*dy + dz*dz).sqrt()
        }
    }

    let my_line = Line{start:p1, end:p2};
    println!("My line has length {}", my_line.len());
}

// Closures
fn closures() {
    println!("----- 5.3. Closures -----");

    let a = 2;
    // {
        let plus_a = |x:i32| {
            let mut z = x;
            z += a;
            z
        };
        println!("a = {}", a);
        println!("{} + a = {}", 3, plus_a(3));
    // }
    let b = &a;
}

// Higher-order functions
fn higher_order_fns() {
    println!("----- 5.4. Higher-order functions -----");

    // greater_than -> i32 -> bool
    fn greater_than(limit:i32) -> impl Fn(i32) -> bool {
        move |x| x >= limit
    }

    // loop
    let limit = 500;
    let mut sum = 0;
    let is_even = |x:i32| x % 2 == 0;
    for i in 0.. {
        let isq = i*i;
        let above_limit = greater_than(limit);
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq
        }
    }
    println!("loop sum = {}", sum);

    // hof
    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|x| *x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |acc, x| acc + x);
    println!("hof sum = {}", sum2);
}

pub fn main() {
    println!("----- 5. Functions -----");
    functions_and_args();
    methods();
    closures();
    higher_order_fns();
}