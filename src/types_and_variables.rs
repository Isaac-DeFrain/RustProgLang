// Types and Variables

use std::mem;
use crate::MUTABLE_VAR; // imports from main.rs

// Fundamental data types
fn fundamental_data_types() {
    println!("----- 2.1. Fundamental data types -----");

    // Primitive types
    let a:u8 = 123; // 8-bit unsigned Int
    let b:i8 = -123; // 8-bit signed Int
    println!("a = {}", a); // macro
    println!("b = {}", b);
    // a = 42; // not allowed since a is immutable
    // let a:u8 = 42; // shadows the variable a
    // println!("a = {}", a); // returns a = 42

    // Mutable variables
    let mut c:u8 = 42; // mutable
    println!("c = {}", c);
    c = 23;
    println!("c = {}", c);

    // Compiler assigned types
    let mut d = 123456789; // compiler assigns signed 32-bit Int
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
    d = -42;
    println!("d = {} after modification", d);

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 666; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, size = {} bytes, {}-bit OS",
             z, size_of_z, 8 * size_of_z);

    // char - 4 bytes (Unicode)
    let e = 'e';
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // floating point - f64 by default
    let f = 3.14; // double-precision
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    // bool - 1 byte
    let g = true;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let h = 4 < 0;
    println!("h = {}", h);
}

// Operators
fn operators() {
    println!("----- 2.2. Operators -----");

    // arithmetic
    let mut a = 2+3*4;
    println!("a = {}", a);
    a += 1; // equivalent to a = a + 1; also -=, *=, /=, %=
    println!("remainder of {} / {} = {}", a, 2, a % 2);

    let a_cubed = i32::pow(a, 3);
    println!("a^3 = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // usual integral powers, i.e. repeated multiplication
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // uses taylor series
    println!("b^3 = {}", b_cubed);
    println!("b^PI = {}", b_to_pi);

    // bitwise - only available for integers
    let c = 42 | 23; // | OR, & AND, ^ XOR, ! NOR
    println!("42 | 23 = {}", c);
    let d = 1 << 10; // >>
    println!("2^10 = {}", d);
    let e = d >> 10;
    println!("(2^10)^-10 = {}", e);

    // logical: ||, &&, !
    println!("{}", true || false);
    println!("{}", true && false);
    println!("{}", !false);
    // comparison: <, >, <=, >=, ==
}

// Scope and Shadowing
fn scope_and_shadowing() {
    println!("----- 2.3. Scope and shadowing -----");

    let a = 42;
    println!("outside, a = {}", a); // 42
    {
        let a = 666;
        println!("inside, a = {}", a); // 666
    }
    println!("outside, a = {}", a); // 42
    unsafe
        {
            println!("{}", MUTABLE_VAR);
        }
}

// Stack and Heap
fn stack_and_heap() {
    println!("----- 2.4. Stack and heap -----");

    // Stack
    /*
      First in, last out structure
      Used to store values of variables in `let` bindings, `fn`, etc.
      - fast
      - limited space
    */

    let x = 5;
    println!("x = {}", x);

    // Heap
    /*
      - stores pointer to data in stack
      - long term storage
    */
    let y = Box::new(42); // stores 5 on the heap, address to value on stack
    println!("y = {}", *y); // * = dereference operator; returns value of y from heap

    struct Point { // record type
        x:f64,
        y:f64,
        z:f64
    }

    fn origin() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }

    let p1 = origin(); // stack allocated value
    let p2 = Box::new(origin()); // pointer to heap allocated value
    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 24 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 bytes (64-bit OS)

    let p3 = *p2;
    println!("p2 x-coordinate = {}", p3.x);
}

pub fn main() {
    println!("----- 2. Types and Variables -----");
    fundamental_data_types();
    operators();
    scope_and_shadowing();
    stack_and_heap();
}