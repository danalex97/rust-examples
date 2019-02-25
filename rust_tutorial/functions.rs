struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // &self is sugar for self: &Self
    // i.e. we borrow self
    fn area(&self) -> f64 {
        let Point {x, y} = self.p1;
        let Point { x: x1, y: y1 } = self.p2;
        ((x1 - x) * (y1 - y)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x, y} = self.p1;
        let Point { x: x1, y: y1 } = self.p2;
        2.0 * ((x1 - x).abs() + (y1 - y).abs())
    }

    // &mut self is sugar to `self: &mut Self`
    fn translate(&mut self, x: f64) {
        // pass each value as a mutable reference
        for c in vec![&mut self.p1.x, &mut self.p2.x, &mut self.p1.y, &mut self.p2.y].iter_mut() {
            // dereference the iterator and the mutable reference
            **c += x;
        }
    }
}

// pair of 2 heap allocate integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    fn destroy(self) {
        let Pair(x, y) = self;
        println!("Destroying Pair({:?}, {:?})", x, y);
    }
}

fn main() {
    let mut rectangle = Rectangle {
        // Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    rectangle.translate(1.0);
    let Rectangle { p1: _, p2: Point {x, y: _} } = rectangle;
    println!("{:?}", x);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // closures
    let cl = |i| i + 1;
    println!("{:?}", cl(1));

    // TODO: why inc is mut?
    let mut count = 0;
    let mut inc = || {
        count += 1;
    };
    inc();
    inc();

    // we can't do this since a variable can be borrowed as mutable only once
    //println!("{:?}", count);

    // Using move before vertical pipes forces closure to take ownership of captured variables:
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    // TODO: why & here?
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // this does not work since haystack does not own reference any more; it would work if
    // closure does not use `move`
    // println!("There're {} elements in vec", haystack.len());

    // TODO after generics and traits:
    //  - https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html later
    //  - https://doc.rust-lang.org/rust-by-example/fn/closures/anonymity.html
    //  - https://doc.rust-lang.org/rust-by-example/fn/closures/input_functions.html
    //  - https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples.html
    //  - https://doc.rust-lang.org/rust-by-example/fn/hof.html
}
