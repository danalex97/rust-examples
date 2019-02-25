// note that operations(e.g. a = b + 2, generates new binding for a) basically generate
// a new binding, just like in Python

fn main() {
    let mut _mutable = 12;
    _mutable = 1_000_023;

    // note that operations(e.g. a = b + 2, generates new binding for a) basically generate
    // a new binding, just like in Python
    _mutable += 2;

    // shadowing => like Python
    let _mutable = 50;

    // shadowing in new scope
    {
        let _mutable = 100;
        println!("{}", _mutable);
    }
    // due to shadowing, the binding is restored here
    println!("{}", _mutable);

    // we can do this, but is not recommended
    let _x;
    _x = 5;
}
