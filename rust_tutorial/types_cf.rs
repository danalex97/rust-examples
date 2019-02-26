use std::convert::From;
use std::string::ToString;

// type aliasing
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // this is a trait: look at that lesson
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        format!("Number({})", self.value)
    }
}

fn main() {
    let _decimal = 65.4321_f32;

    // Explicit conversion
    let _integer = _decimal as u8;
    let _character = _integer as char;

    // type inference
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();

    // Insert `elem` in the vector. Compiler infers type.
    // If this is commented, the compiler cannot infer type.
    vec.push(elem);

    println!("{:?}", vec);

    // use type aliasing
    let _inches: Inch = 2 as u64_t;

    // type conversion
    let num = Number::from(30);
    println!("My number is {}", num.to_string());

    // parsing ints
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap(); // 'turbofish' syntax

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};

    // expressions
    let x = 5u32;
    let _y = {
       let x_squared = x * x;
       let x_cube = x_squared * x;

       x_cube + x_squared + x
   }; // blocks are RHS expressions

   // breaking using labels
   'outer: loop {
       println!("Entered the outer loop");

       'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
   }

   // break can resturn stuff
   let mut counter = 0;
   let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // fors & ranges
    for x in 1..10 { print!("{} ", x); } println!("");
    for x in 1..=10 { print!("{} ", x); } println!("");

    // note vec! is a macro for declaring a vector
    for name in vec!["Bob", "Frank", "Ferris"].into_iter() {
        // iter leaves collection untouched by borrowing
        // into_iter consumes collection
        // iter_mut allows to modify collection via mutable borrowing
        println!("{}", name);
    }
    // can't do this with into_iter since collection was consumed
    // println!("{:?}", names);

    // match example
    for x in 1..5 {
        let y = x * 5;
        print!("{} ", y);
        match y  {
            2 | 3 | 5 | 7 | 11 => println!("Prime"),
            y @ 13...19 => println!("hmm: {}", y),
            _ => println!("Ain't special"),
        }
    }

    // match can destructure tuples and enums

    // match for destructuring pointers
    //  - dereferencing: *
    //  - destructuring: &, ref, ref mut

    // assign reference to number 4
    let reference = &4;
    match reference {
        &val => println!("{:?}", val),
    }
    match *reference {
        val => println!("{:?}", val),
    }

    // if val is not declared mutable, the match cannot borrow mutably
    let mut value = 3;
    match value {
        ref mut r => println!("{:?}", r),
    }

    // destructuring structures
    struct Foo { x: (u32, u32), y: u32 }
    let foo = Foo { x: (1, 2), y: 3 };
    // note we can combine this with destructuring tuples
    let Foo { x: (a, _), y } = foo;
    println!("{} {}", a, y);


    // guards
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    // if and while let can be used
    let number  = Some(7);
    let number2: Option<i32> = None;
    for x in vec![number, number2].iter() {
        if let Some(x) = x { // tries to destructure
            println!("Found {}.", x);
        } else {
            println!("Found none.");
        }
    }
}
