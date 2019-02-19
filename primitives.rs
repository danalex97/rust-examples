use std::fmt;
use std::mem;

fn _reverse(pair: (i32, bool)) -> (bool, i32) {
    let (x, y) = pair;
    (y, x)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {}, {} )\n", self.0, self.1)?;
        write!(f, "( {}, {} )", self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    let (a, b, c, d) = (m.0, m.1, m.2, m.3);
    Matrix(a, c, b, d)
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // annotations
    let _a_float: f64 = 1.0;
    let _an_integer   = 5i32;

    // mutable
    let mut _mutable = 12;
    _mutable = 1_000_023;

    // error => The type of a variable can't be changed.
    // mutable = true;

    // shadowing => like Python
    let _mutable = true;

    // error => the new declared variable is not mutable
    // mutable = false;

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("m:\n{}", matrix);
    println!("t:\n{}", transpose(matrix));

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let _ys: [i32; 9] = [0; 9]; // 9 zeros

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("length: {}", xs.len());

    // stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices; a slice can be used to borrow a section
    // of an array
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1..4]); // slice is [2, 3, 4], i.e. the right end is non-inclusive

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
}
