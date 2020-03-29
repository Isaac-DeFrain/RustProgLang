// Traits
#![allow(unused_variables)]
#![allow(unused_mut)]

// Traits
fn traits() {
    println!("----- 6.1. Traits -----");

    trait Animal {
        fn new(name: &'static str) -> Self; // static method
        fn name(&self) -> &'static str;
        fn talk(&self) {
            println!("{} cannot talk", self.name())
        }
    }

    struct Human {
        name: &'static str
    }

    struct Cat {
        name: &'static str
    }

    impl Animal for Human {
        fn new(name: &'static str) -> Human {
            Human{name}
        }
        fn name(&self) -> &'static str { self.name }
        fn talk(&self) {
            println!("{} says hello", self.name)
        }
    }

    impl Animal for Cat {
        fn new(name: &'static str) -> Cat {
            Cat{name}
        }
        fn name(&self) -> &'static str { self.name }
        fn talk(&self) {
            println!("{} says meow", self.name)
        }
    }

    let isaac = Human::new("Isaac");
    isaac.talk();

    let fluffy: Cat = Animal::new("Fluffy");
    fluffy.talk();

    // defining behaviors for types we don't own
    // define a sum() method for vectors
    trait Summable<T> {
        fn sum(&self) -> T;
    }

    // implement sum() as the sum of elements
    impl Summable<i32> for Vec<i32> {
        fn sum(&self) -> i32 {
            let mut acc = 0;
            for x in self {
                acc += *x
            }
            return acc
        }
    }

    let a = vec![1,2,3,4,5];
    println!("vector sum = {}", a.sum());
}

// Trait parameters
use std::fmt::Debug;

fn trait_params() {
    println!("----- 6.2. Trait parameters -----");

    trait Shape<T> {
        fn kind(&self) -> &'static str;
        fn number_of_sides(&self) -> Option<u8>;
        fn area(&self) -> T;
        fn center(&self) -> (T, T);
    }

    #[derive(Debug)] // equivalent to deriving Show in Haskell
    struct Circle {
        center: (f32, f32),
        radius: f32
    }

    #[derive(Debug)]
    struct Rectangle {
        lower_left: (f32, f32),
        upper_right: (f32, f32)
    }

    impl Shape<f32> for Circle {
        fn kind(&self) -> &'static str { "circle" }
        fn number_of_sides(&self) -> Option<u8> { None }
        fn area(&self) -> f32 {
            let pi = std::f32::consts::PI;
            let rsq = (self.radius * self.radius) as f32;
            pi * rsq
        }
        fn center(&self) -> (f32, f32) { self.center }
    }

    impl Shape<f32> for Rectangle {
        fn kind(&self) -> &'static str { "rectangle" }
        fn number_of_sides(&self) -> Option<u8> { Some(4) }
        fn area(&self) -> f32 {
            let width = self.upper_right.0 - self.lower_left.0;
            let length = self.upper_right.1 - self.lower_left.1;
            length * width
        }
        fn center(&self) -> (f32, f32) {
            let horizontal = (self.upper_right.0 + self.lower_left.0) / 2.0;
            let vertical = (self.upper_right.1 + self.lower_left.1) / 2.0;
            (horizontal, vertical)
        }
    }

    let circ = Circle{center: (0.0, 1.32), radius: std::f32::consts::FRAC_1_SQRT_2};
    let rect = Rectangle{lower_left: (0.0, 1.32), upper_right: (3.14159, 2.7182)};

    // on-the-fly constraints
    // trait-bound syntax: fn shape_details<T: Shape + Debug>(shape: T, shape2: T,...) {...}
    // fn shape_details<T>(shape: T, shape2: T,...) where T: Shape + Debug {...}
    fn shape_details(shape: impl Shape<f32> + Debug) {
        println!("Details of {:?}", shape);
        println!("Number of sides: {:?}", shape.number_of_sides());
        println!("Center: {:?}", shape.center());
        println!("Area: {}", shape.area());
    }

    shape_details(circ);
    shape_details(rect);
}

// Into
fn into() {
    println!("----- 6.3. Into -----");

    #[derive(Debug)]
    struct Person {
        name: String
    }

    impl Person {
        // fn new(name: &str) -> Person {
        //     Person{name: name.to_string()}
        // }
        //fn new<S: Into<String>>(name: S) -> Person {
        fn new<S: Into<String>>(name: S) -> Person {
            Person { name: name.into() }
        }
    }

    let isaac = Person::new("Isaac");
    println!("{:?}", isaac);
}

// Drop
fn drop_() {
    println!("----- 6.4. Drop -----");

    struct Creature {
        name: String
    }

    impl Creature {
        fn new(name: &str) -> Creature {
            println!("{} enters the game", name);
            Creature{ name: name.into() }
        }

    }

    impl Drop for Creature {
        fn drop(&mut self) {
            println!("{} is dead now", self.name)
        }
    }

    let goblin = Creature::new("Jeff");
    println!("Super exciting game play ensues");
    drop(goblin); // force drop here
    println!("blah blah blah");
    //let n = goblin.name; // doesn't compile because goblin was moved and std::string::String does not implement the Copy trait
}

// Operator overloading
use std::ops::{Add, Mul, Sub, AddAssign};
use std::fmt;
use std::process::Output;

fn operator_overloading() {
    println!("------ 6.5. Operator overloading -----");

    // operator overloading in Rust is accomplished through traits

    #[derive(Debug, Copy, Clone)]
    struct Complex<T> {
        re: T,
        im: T
    }

    impl<T> Complex<T> {
        fn new(r: T, i: T) -> Complex<T> {
            Complex::<T>{re:r, im:i}
        }
    }

    // implementing display trait for Complex<T: std::fmt::Display>
    impl<T: std::fmt::Display> fmt::Display for Complex<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }

    // nice default display
    let mut a = Complex::new(1,2);
    let mut b = Complex::new(4,5);
    println!("a = {}", a);
    println!("b = {}", b);

    // + overloading for Complex<T: Add<Output = T>>
    impl<T: Add<Output = T>> Add for Complex<T> {
        type Output = Complex<T>;
        // self is a reference; Self is the type of the rhs operand, i.e. Complex<T>
        fn add(self, rhs: Self) -> Self::Output {
            Complex{
                re: self.re + rhs.re,
                im: self.im + rhs.im
            }
        }
    }
    // - overloading for Complex<T: Sub<Output = T>>
    impl<T: Sub<Output = T>> Sub for Complex<T> {
        type Output = Complex<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Complex{
                re: self.re - rhs.re,
                im: self.im - rhs.im
            }
        }
    }
    // * overloading for Complex<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy>
    // Copy is necessary because we use real and imaginary parts multiple times in calculation
    impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Mul for Complex<T> {
        type Output = Complex<T>;
        fn mul(self, rhs: Self) -> Self::Output {
            Complex{
                re: self.re * rhs.re - self.im * rhs.im,
                im: self.re * rhs.im + self.im * rhs.re
            }
        }
    }
    impl<T: AddAssign<T>> AddAssign for Complex<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.re += rhs.re;
            self.im += rhs.im
        }
    }

    println!("a + b = {}", a + b);
    println!("a * b = {}", a * b);
    a += b;
    println!("a = {}", a);

    // for comparing complex numbers, we need to talk about equality
    // two types of equality:
    // partial equality
    // full equality: x = x (cannot be supported for x = NAN)
    impl<T: PartialEq> PartialEq for Complex<T> {
        fn eq(&self, other: &Self) -> bool {
            self.re == other.re && self.im == other.im
        }
    }

    impl<T: Eq> Eq for Complex<T> {}
    // these traits can also be derived automatically with #[derive(PartialEq, Eq)]

    println!("a == b = {}", a == b);
    println!("a != b = {}", a != b);
}

// Static dispatch
fn static_dispatch() {
    println!("----- 6.6. Static dispatch -----");

    // dispatch = how the compiler knows what to call
    // Rust supports static & dynamic dispatch

    trait Printable {
        fn format(&self) -> String;
    }

    impl Printable for i32 {
        fn format(&self) -> String {
            format!("i32: {}", *self)
        }
    }

    impl Printable for String {
        fn format(&self) -> String {
            format!("String: {}", *self)
        }
    }

    fn print_it<T: Printable>(x: T) {
        println!("{}", x.format())
    } // monomorphization

    // static dispatch
    let a = 666;
    let b = String::from("hello");
    print_it(a); // compiler determines correct implementation for print_it
    print_it(b);

}

// Dynamic dispatch
fn dynamic_dispatch() {
    println!("----- 6.7. Dynamic dispatch -----");

    trait Printable {
        fn format(&self) -> String;
    }

    impl Printable for i32 {
        fn format(&self) -> String {
            format!("i32: {}", *self)
        }
    }

    impl Printable for String {
        fn format(&self) -> String {
            format!("String: {}", *self)
        }
    }

    // dynamic dispatch -- format implementation is chosen at runtime!
    // print_it has to look at the type of x when the fn is called
    fn print_it(x: &dyn Printable) {
        println!("{}", x.format())
    }

    let a = 666;
    let b = String::from("hello");
    print_it(&a);
    print_it(&b);
}

// Why dynamic dispatch
fn why_dynamic_dispatch() {
    println!("----- 6.8. Why dynamic dispatch -----");

    // dynamic dispatch is needed if we have a function accepting arguments of a type with subtypes having different implementations
    // e.g. area(x: &Shape) and both Circle and Square are Shape
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle{radius:f64}
    struct Square{side:f64}

    impl Shape for Circle {
        fn area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    let shapes: [&dyn Shape;3] = [
        &Circle{radius: 5.0},
        &Circle{radius: 2.0},
        &Square{side: 5.0}
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());
    }
}

// Vectors of different objects
fn vectors_of_different_objects() {
    println!("----- 6.9. Vectors of different objects ------");

    trait Animal {
        fn new(name: &'static str) -> Self; // static method
        fn name(&self) -> &'static str;
        fn talk(&self) {
            println!("{} cannot talk", self.name())
        }
    }

    struct Human {
        name: &'static str
    }

    struct Cat {
        name: &'static str
    }

    impl Animal for Human {
        fn new(name: &'static str) -> Human {
            Human{name}
        }
        fn name(&self) -> &'static str { self.name }
        fn talk(&self) {
            println!("{} says hello", self.name)
        }
    }

    impl Animal for Cat {
        fn new(name: &'static str) -> Cat {
            Cat{name}
        }
        fn name(&self) -> &'static str { self.name }
        fn talk(&self) {
            println!("{} says meow", self.name)
        }
    }

    let isaac = Human{name: "Isaac"};
    isaac.talk();
    let garfield = Cat{name: "Garfield"};
    garfield.talk();

    //let animals: Vec<Animal> = vec![isaac, garfield]; <--- does not compile
    // define enum { Human(Human), Cat(Cat) } which uses the Human and Cat structs
    // or define a vector of Boxes
    //let mut animals: Vec<Box<dyn Animal>> = Vec::new(); <--- does not compile either...
    //animals.push(Box::new(isaac));
    //animals.push(Box::new(garfield));
}

pub fn main() {
    println!("----- 6. Traits -----");
    traits();
    trait_params();
    into();
    drop_();
    operator_overloading();
    static_dispatch();
    dynamic_dispatch();
    why_dynamic_dispatch();
}