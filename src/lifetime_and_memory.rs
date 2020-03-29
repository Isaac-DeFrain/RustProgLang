// Lifetime and Memory
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

// Ownership
fn ownership() {
    println!("----- 7.1. Ownership -----");

    // Memory safety is the main value add of Rust.
    // Memory safety is achieved through the notion of *ownership*

    let v = vec![1,2,3,4,5]; // v owns the memory stored in the vector
    // the variable is on the stack, but the data is on the heap
    let w = v; // effectively creates a pointer to the data in the heap represented by v
    // only one variable can point to any piece of data on the heap => memory safety, no data races
    //println!("{:?}", v); <--- does not compile because 'v' was moved

    // primitive types are stored on the stack so they implement the Copy trait
    // functions also take ownership of non-primitive variables
    let foo = |_:Vec<i32>| ();
    foo(w);
    //println!("w = {:?} is still here!", w); <--- value borrowed here after move

    let a = 0;
    let bar = |_:i32| ();
    bar(a);
    println!("a = {} is still here!", a);

    // inelegant solution
    let z = vec![1,2,3];
    let print_return = |x: Vec<i32>| -> Vec<i32> {
        println!("vector = {:?}", x);
        x
    };
    let zz = print_return(z);
    let zzz = print_return(zz); // works but it kinda sucks

    // primitive types can be forced to be stored on the heap with Box
    let b = Box::new(0);
    let baz = |_:Box<i32>| ();
    baz(b);
    //println!("b = {} is still here!", b); <--- value borrowed here after move

    // likewise non-primitive types can be forced to be stored on the stack by implementing the Copy trait

    // reminiscent of channels and unforgeable names in Rholang!
}

// Borrowing
fn borrowing() {
    println!("----- 7.2. Borrowing -----");

    // elegant solution -- borrowing instead of moving
    // immutable references
    let v = vec![1,2,3];
    let print_vector_ref = |x: &Vec<i32>| {
        println!("vector = {:?}", x);
        //x.push(4); <-- cannot borrow immutable local variable 'x' as mutable
    };
    print_vector_ref(&v);
    println!("the vector {:?} is still here!", v);

    // mutable references
    let mut a = 40;
    let x = &a;
    let y = &a;     // there can be arbitrarily many immutable borrows, but no sandwiching
    let b = &mut a; // only one nontrivial mutable borrow is allowed to prevent data races
    *b += 2;
    //drop(b);
    println!("a = {}", a);

    let mut z = vec![1,2,3,4,5];
    for v in &z { // immutable borrow
        println!("{}", v);
        //z.push(6); <-- mutable borrow
        // pushing values to the vector as we iterate over it would cause undefined behavior
        // so Rust prevents it
    }
}

// Lifetime
fn lifetime() {
    println!("----- 7.3. Lifetime -----");

    let hey: &'static str = "hey hey hey"; // static = lives as long as the program is running

    // Example of the necessity of lifetimes
    struct Person {
        name: String
    }

    struct Company<'t> {
        name: String,
        boss: &'t Person // must explicitly specify lifetime of this reference
        // Rust prevents a situation where the Person goes out of scope before the Company
    }

    let elon = Person{ name: String::from("Elon Musk") };
    let tesla = Company{ name: String::from("Tesla"), boss: &elon };

    // lifetime's in methods are handled by the compiler
    impl Person {
        // compiler auto-magically rewrites this to:
        // fn get_ref_name<'a>(&'a self) -> &'a String {...} -- function elision
        fn get_ref_name(&self) -> &String {
            &self.name
        }
    }

    let mut n: &String;
    {
        let p = Person{ name: String::from("PERSON") };
        n = p.get_ref_name();
    }
}

// Lifetime in structure implementation
fn lifetime_in_structure_impl() {
    println!("----- 7.4. Lifetime in structure implementation -----");

    struct Person<'a> {
        name: &'a str // no lifetime elision, must explicitly specify lifetime
    }

    impl<'a> Person<'a> {
        fn talk(&self) {
            println!("Hi, name is {}!", self.name)
        }
    }
    let isaac = Person{name: "Isaac"};
    isaac.talk();
}

// Reference counted variables
fn rc_variables() {
    println!("----- 7.4. Reference counted variables -----");

    // alternative to borrowing and ownership -- reference counted variables
    use std::rc::Rc;

    struct Person {
        name: Rc<String> // reference counted String
    }

    impl Person {
        fn new(name: Rc<String>) -> Person {
            Person{name}
        }

        fn greet(&self) {
            println!("Hello, I'm {}", self.name)
        }
    }

    let name = Rc::new(String::from("Isaac"));
    println!("name = {} -- has {} strong pointer(s)", name, Rc::strong_count(&name)); // 1
    {
        let isaac = Person::new(name.clone());
        isaac.greet();
        println!("name = {} -- has {} strong pointer(s)", name, Rc::strong_count(&name)); // 2
    }
    println!("name = {}", name); // compiles now; nice!
    println!("name = {} -- has {} strong pointer(s)", name, Rc::strong_count(&name)); // 1
}

// Atomic reference counted variables
fn atomic_rc_variables() {
    println!("----- 7.6. Atomic reference counted variables -----");

    // rc variables are limited to a single thread
    use std::thread;
    use std::rc::Rc;
    use std::sync::Arc;

    struct Person {
        name: Arc<String> // reference-counted String
    }

    impl Person {
        fn new(name: Arc<String>) -> Person {
            Person{name}
        }

        fn greet(&self) {
            println!("Hello, I'm {}", self.name)
        }
    }

    let name = Arc::new(String::from("Isaac"));
    let isaac = Person::new(name.clone());
    let t = thread::spawn(move || {
        isaac.greet()
    }); // `std::rc::Rc<std::string::String>` cannot be sent between threads safely
    // Rc class does NOT implement the Send trait, but Arc DOES!

    println!("name = {}", name);
    t.join().unwrap();

    // Arc allows for a thread safe way of counting references -- guarded
}

// Using a Mutex for thread-safe mutability
fn mutex() {
    println!("----- 7.7. Using a Mutex for thread-safe mutability -----");

    use std::thread;
    use std::rc::Rc;
    use std::sync::{Arc,Mutex};

    struct Person {
        name: Arc<String>, // reference-counted String
        mood: Arc<Mutex<String>>
    }

    impl Person {
        fn new(n: Arc<String>, m: Arc<Mutex<String>>) -> Person {
            Person{name: n, mood: m}
        }

        fn greet(&self) {
            println!("Hello, I'm {} and I'm currently feeling {:?}!",
                     self.name, self.mood.lock().unwrap().as_str());
            let mut mood = self.mood.lock().unwrap();
            mood.clear();
            mood.push_str("excited");
            println!("But now that I've met you, I'm feeling {}!", mood);
        }
    }

    let name = Arc::new(String::from("Isaac"));
    let mood = Arc::new(Mutex::new(String::from("happy")));
    let isaac = Person::new(name.clone(), mood.clone());
    let t = thread::spawn(move || {
        isaac.greet()
    });

    t.join().unwrap();
}

pub fn main() {
    println!("----- 7. Lifetime and Memory -----");
    ownership();
    borrowing();
    lifetime();
    lifetime_in_structure_impl();
    rc_variables();
    atomic_rc_variables();
    mutex();
}