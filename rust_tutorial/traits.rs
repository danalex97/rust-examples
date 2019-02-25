use std::ops;
// Trait - collection of methods for unknown type `Self`; they can access methods in the same
// trait

// &'static is a refences that lives for entire duration of program
struct Sheep {
    naked : bool,
    name : &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

// impl of sheep has sheep's own methods
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name());
            self.naked = true;
        }
    }
}

// implementing the Animal trait for Sheep
impl Animal for Sheep {
    // note here return type is Sheep whereas in trait is Self
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked : false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaa?"
        } else {
            "baaaaah!"
        }
    }

    fn talk(&self) {
        println!("{}: {}", self.name, self.noise());
    }
}

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        // we need & since self is a reference
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// overloading example
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

impl ops::Add<Bar> for Foo {
    // we need Output to implement this trait as well
    type Output = FooBar;

    fn add(self, _rhs : Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    // note we need this signature
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

// Drop - implement for when objects get out of scope, call via `drop`
// Clone - derive for cloning objects 

fn main() {
    // we need mut since func `shear` borrows mutably
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    // example for Derive
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter.", cmp);

    // example for oprator overloading
    println!("{:?}", Foo + Bar);

    // fibonacci iterator
    println!();
    for i in fibonacci().take(4) {
        // note the number is out of the optinal here
        println!("> {}", i);
    }

    println!();
    let array = [1u32, 3, 3, 7];
    // The `iter` method produces an `Iterator` over an array/slice.
    for i in array {
        println!("> {}", i);
    }
    println!();
}
