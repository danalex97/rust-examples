use std::fmt::Debug;

// a generic tuple struct
struct SGen<T>(T);

// a generic function
fn gen_fn<T>(_s : SGen<T>){}

struct Val<T> {
    val : T,
}

// impl of Val -- similar to a usual impl(e.g. see functions.rs)
impl <T> Val<T> {
    fn value(&self) -> &T { &self.val }
}

struct Empty;
struct Null;

// Generic traits example
trait DoubleDrop<T> {
    fn double_drop(self, _ : T);
}

impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

// Bounds
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn print_debug_area<T: Debug + HasArea>(t: &T) {
    println!("{:?}", t.area());
}

// where
trait PrintInOption {
    fn print_in_option(self);
}

// a generic implementation; without where we can't express `Option<T>: Debug`
impl<T> PrintInOption for T where
    Option<T>: Debug {

    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

// Associated types
// `A` and `B` are defined in the trait via the `type` keyword.
trait Contains {
    type A;
    type B;

    // Syntax to refer to these new types generically.
    fn contains(&self, &Self::A, &Self::B) -> bool;
}

struct Container(i32, i32);

impl Contains for Container {
    type A = i32;
    type B = i32;

    // can use &Self::A or &int32
    fn contains(&self, number_1: &Self::A, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
}

fn print_contains<C: Contains>(container: &C, x : &C::A, y : &C::B) {
    println!("Containes: {}", container.contains(&x, &y));
}

fn main() {
    gen_fn::<char>(SGen('a'));

    let v = Val { val : 2u32 };
    println!("{}", v.value());

    let empty = Empty;
    let null  = Null;
    empty.double_drop(null);
    // null is not accessible here due to move semantics

    // bounds
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    println!("{:?}", area(&rectangle));
    print_debug(&rectangle);
    print_debug_area(&rectangle);

    let vec = vec![1, 2, 3];
    vec.print_in_option();

    print_contains(&Container(1i32, 2i32), &1i32, &2i32);
}
