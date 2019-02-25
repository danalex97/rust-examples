#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct - useful for generics
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn rect_area(self) -> f32 {
        let Rectangle { p1, p2 } = self;

        let Point { x: x0, y: y0 } = p1;
        let Point { x: x1, y: y1 } = p2;

        (x1 - x0) * (y1 - y0)
    }
}

fn square(p : Point, l : f32) -> Rectangle {
    let Point { x, y } = p;
    Rectangle {
        p1 : Point { x, y },
        p2 : Point { x: x + l, y: y + l},
    }
}

// each valid struct type is a valid enum
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad   => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),

        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),

        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

// C-like enums
enum Nbr {
    Zero,
    One,
}

#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

#[allow(dead_code)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

#[allow(dead_code)]
impl List {
    fn new() -> List {
        List::Nil
    }

    // All values in Rust are stack allocated by default.
    // Values can be boxed (allocated in the heap) by creating a Box<T>.
    // A box is a smart pointer to a heap allocated value of type T.
    // When a box goes out of scope, its destructor is called, the inner object is destroyed,
    //   and the memory in the heap is freed.
    fn add(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // Boxed values can be dereferenced using the * operator;
        //   this removes one layer of indirection.

        // `self` has type `&List`, so `*self` has type `List`
        // matching over concrete type is preferred
        match *self {
            // can't take ownership of tail since `self` is borrowed: &
            // instead take a reference to the tail
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }
}

// mulable global variable - need to be typed
static _LANGUAGE: &str = "Rust";

// unmutable global variable - need to be typed
const _THRESHOLD: i32 = 10;

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // update the fields in point with x: 0.1 (like JS Object.assign)
    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // "Destructure" the point -- note it needs all fields; can discard by _
    let Point { x: my_x, y: my_y } = point;
    println!("{}", my_x);

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x }, // can instantiate as an expression as well
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity 1 & 2 - structs
    let rect = Rectangle {
        p1 : Point { x: 0f32, y: 0f32},
        p2 : Point { x: 2f32, y: 2f32},
    };
    println!("{}", rect.rect_area());
    println!("{:#?}", square(Point{ x: 0.0, y: 0.0}, 2.0));

    // enums
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // avoid manual scoping via use
    use WebEvent::{PageLoad, KeyPress};
    inspect(KeyPress('x'));
    inspect(PageLoad);

    // can be used as C-like enum
    println!("zero is {}", Nbr::Zero as i32);
    println!("one  is {}", Nbr::One as i32);
}
