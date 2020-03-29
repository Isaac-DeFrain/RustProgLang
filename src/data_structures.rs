// Data Structures

#![allow(illegal_floating_point_literal_pattern)]
#![allow(unused_variables)]

// Structs
use std::fmt;

fn structs() {
    println!("----- 4.1. Structs -----");

    // struct = record
    #[derive(Clone, Copy)] // allows reuse of Point variables
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

    let _myline = Line {start: p1, end: p2}; // prefix with _ if not used elsewhere
    let myline = Line {start: p2, end: p1};
    println!("My line is {}", myline);
}

// Enumerations
fn enums() {
    println!("----- 4.2. Enumerations -----");

    //struct CMYK{cyan:u8, magenta:u8, yellow:u8, black:u8}
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u8, u8, u8), // tuple-style
        CMYK{cyan:u8, magenta:u8, yellow:u8, black:u8} // struct-style
    }

    let color = Color::RGB(0,0,0);
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        // equivalent to \/
        Color::RGB(0,0,0) | Color::CMYK{cyan:_, magenta:_, yellow:_, black:255}
          => println!("black"),
        Color::RGB(r,g,b) => println!("RGB({}, {}, {})", r, g, b),
        Color::CMYK{cyan:c, magenta:m, yellow:y, black:k}
          => println!("CMYK[cyan: {}, magenta: {}, yellow: {}, black: {}]", c, m, y, k),
    }
}

// Unions
fn unions() {
    println!("----- 4.3. Unions -----");

    // Included for compatibility with C, C++, etc.

    // 32 bits
    union IntOrFloat {
        int: i32,
        flt: f32
    }

    let i = IntOrFloat{int: 666};
    let f = IntOrFloat{flt: 6.66};
    let iof1 = unsafe {i.flt};
    let iof2 = unsafe {f.int};
    println!("666 -> flt = {}", iof1);
    println!("6.66 -> int = {}", iof2);

    fn matching_unions(iof: IntOrFloat) {
        unsafe {
            match iof {
                IntOrFloat{ int: 42 } => println!("int 42 matched"),
                IntOrFloat{ flt: 2.3 } => println!("float 2.3 matched"),
                IntOrFloat{ flt } => println!("{} float", flt),
            }
        }
    }

    matching_unions(IntOrFloat{int: 42});
    matching_unions(IntOrFloat{int: 23});
    matching_unions(IntOrFloat{flt: 2.3});
}

// Option<T> and if let/while let
fn optionT() {
    println!("----- 4.4. Option<T> -----");

    fn safe_div(x:f32, y:f32) -> Option<f32> {
        if y == 0.0 {None}
        else {Some(x/ y)}
    }

    fn print_option(op: Option<f32>) {
        match op {
            None => println!("None"),
            Some(f) => println!("Some({})", f)
        }
    }

    // debug output
    // equivalent to print_option(safe_div(...))
    println!("{:?}", safe_div(3.14, 1.01));

    print_option(safe_div(2.01, 0.0));
    print_option(safe_div(2.01, 0.1));
    print_option(safe_div(2.01, 1.0));

    // if-let
    // if match is successful, the first branch is executed with appropriate binding
    // else the second branch is executed
    if let Some(res) = safe_div(2.01, 10.0) { println!("result = {}", res) }
    else { println!("cannot divide by 0") }

    // while-let
    fn count_down_to_none(int:u8) -> Option<u8> {
        if int == 0 { None }
        else { Some(int - 1) }
    }

    let mut x:Option<u8> = Some(10);
    // true as long as match is successful
    while let Some(i) = x {
        println!("{}", i);
        x = count_down_to_none(i);
    }
    println!("done!")
}

// Arrays -- all elements have the same type
use std::mem;

fn arrays() {
    println!("----- 4.5. Arrays -----");

    // the size of an array is fixed
    let mut a:[i32;5] = [1,2,3,4,5]; // length 5 array of i32's
    println!("{:?} has length {}, first element is {}", a, a.len(), a[0]);
    a[0] = 42;

    // cannot compare arrays of different sizes,
    // i.e. a == b does not compile
    let b = [1u8; 10]; // 10 entries, each of size 1 byte = 10 bytes
    for i in 0..b.len() {
        println!("{}", b[i])
    }

    println!("b takes up {} bytes in memory", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = [[1.0, 0.0, 0.0], [0.0, 1.0, 3.14]];
    println!("{:?}", mtx);
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j { println!("mtx[{}][{}] = {}", i, j, mtx[i][j]) }
        }
    }

    // Arrays have a fixed size and can't be changed.
}

// Vectors == stack (LIFO) -- all elements have the same type
fn vectors() {
    println!("----- 4.6. Vectors -----");

    // the size of a vector can change dynamically, not fixed like an array
    let mut v = Vec::new(); // creates vector
    v.push(1); // adds 1 to last position
    v.push(2); // adds 2 to last position
    v.push(3); // adds 3 to last position
    let elem = v.pop(); // removes last entry, binds as an option (since vector can be empty)
    println!("v = {:?}", v); // can also print &v
    println!("popped element = {:?}", elem);

    // access vector entries like an array's
    println!("v[0] = {}", v[0]);

    // index has type usize i.e. u64 since this I'm on a 64-bit OS
    let idx:usize = 0;
    println!("v[0] = {}", v[idx]);

    // directly accessing vector elements can lead to runtime errors since the size can change
    // safe access = get, returns option
    fn get_vector_elem(idx:usize, vec:&Vec<i32>) {
        match vec.get(idx) {
            Some(x) => println!("The vector's {}-th element is {}", idx, x),
            None => println!("Error: out-of-bounds")
        }
    }

    // using reference because std::vec::Vec<i32> does not implement the Copy trait
    get_vector_elem(12, &v); // None, length = 2

    // let Some(last_elem) = v.pop(); // refutable pattern error -- None case is not covered

    // iterate over vector, from first to last
    for x in &v {
        println!("{}", x)
    }

    while let Some(x) = v.pop() {
        println!("{}", x) // prints last to first
    }
}

// Slices
fn slices() {
    println!("----- 4.7. Slices -----");

    // part of an array, but unlike an array, size is not known at compile time
    let mut data = [1,2,3,4,5];
    fn print_slice(slice: &mut [i32]) {
        println!("{:?}", slice);
        slice[0] = 666
    }
    print_slice(&mut data[1..4]); // [2, 3, 4]
    println!("{:?}", data); // [1, 666, 3, 4, 5]
}

// Strings
fn strings() {
    println!("----- 4.8. Strings -----");

    // utf-8
    let s:&'static str = "hello"; // type: &'static str = string slice
    // static => when we reference s, we are referring to a location in the program
    // immutable, fixed size like array
    // str created at runtime just has memory allocated and we operate on that memory

    // iterating over &str
    for c in s.chars().rev() {
        print!("{}", c);
    }
    print!("\n");

    // accessing particular indices
    if let Some(char) = s.chars().nth(0) {
        println!("The first element of {} is {}", s, char)
    }

    // String -- heap allocated vector type (support vector operations)
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(", ");
        a += 1
    }
    println!("{}", letters);

    // both &str and String are commonly used in APIs

    // &str <> String
    let x:&str = &letters; // deref conversion: String -> &str
    let y = x.to_string(); // &str -> String
    println!("{}", y == String::from(x)); // true
    // x.to_owned() -> String

    // concatenation
    // String + &str
    let cat = y + &letters;
    println!("{}", cat)
}

// Tuples -- can hold different types
// struct with fields 0, 1, 2, ...
fn tuples() {
    println!("----- 4.9. Tuples ------");

    fn sum_prod(x:i32, y:i32) -> (i32, i32) {
        (x + y, x * y)
    }

    let sp = sum_prod(3, 4);
    // destructuring
    let (s, p) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", 3, 4, s, p);

    let sp1 = sum_prod(19,23);
    let combined = (sp, sp1);
    println!("{:?}", combined);

    // single element tuple
    let meaning = (42,);
    println!("{:?}", meaning);
}

// Hashmap
use std::collections::HashMap;

fn hashmap() {
    println!("----- 4.10. Hashmap ------");

    let mut polygons = HashMap::new();
    polygons.insert(String::from("triangle"), 3);
    polygons.insert(String::from("rectangle"), 4);
    polygons.insert(String::from("pentagon"), 5);
    println!("a {} has {} sides", "rectangle", polygons["rectangle".into()]); // unsafe like array access

    for (key, val) in &polygons {
        println!("{} : {}", key, val)
    }

    println!("{:?}", polygons);

    // polygons.insert("rectangle".into(), 5); // overwrites this value
    polygons.entry("circle".into()).or_insert(1);
    println!("{:?}", polygons);
    {
        let circ = polygons.entry("circle".into()).or_insert(2);
        *circ = 0
    }
    println!("{:?}", polygons);
}

// Pattern matching
fn pattern_matching() {
    println!("----- 4.11. Pattern matching -----");

    fn how_many(x:u32) -> &'static str {
        match x {
            0 => "no",
            1 | 2 => "one or two",
            z@3..=6 => "a few",
            12 => "a dozen",
            _ if (x % 2 == 0) => "an even number of",
            _ => "several"
        }
    }

    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x))
    }

    enum Color {
        Red,
        Blue,
        Green,
        RGB(u8, u8, u8), // tuple-style
        CMYK{cyan:u8, magenta:u8, yellow:u8, black:u8} // struct-style
    }

    let color = Color::CMYK{cyan:0,magenta:0,yellow:0,black:123};
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        // equivalent to \/
        Color::RGB(0,0,0) | Color::CMYK{cyan:_, magenta:_, yellow:_, black:255}
        => println!("black"),
        Color::RGB(r,g,b) => println!("RGB({}, {}, {})", r, g, b),
        Color::CMYK{black:k, cyan:c,..} // equivalent to _ for all other fields
        => println!("CMYK[cyan: {}, black: {}]", c, k),
    }

    // Pattern matching is arbitrarily deep
}

// Generics
fn generics() {
    println!("----- 4.12. Generics -----");

    // parametric polymorphism

    struct Point<A,B> {
        x: A,
        y: B
    }

    struct Line<A,B,C,D> {
        start: Point<A,B>,
        end: Point<C,D>
    }

    let a:Point<u16,f32> = Point {x:0, y:0.01};
    let b:Point<f32,f64> = Point {x:3.14, y:0.00001};
    let my_line = Line{start:a, end:b};
}

pub fn main() {
    println!("----- 4. Data structures -----");
    structs();
    enums();
    unions();
    optionT();
    arrays();
    vectors();
    slices();
    strings();
    tuples();
    hashmap();
    pattern_matching();
    generics();
}