use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "s({})", self.0)
    }
}

impl fmt::Display for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d({})", self.0)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (i, x) in vec.iter().enumerate() {
            if i != 0 {
                // write!(f, ", "); => this `Result` may be an `Err` variant, which should be handled
                // we add ? to handle `Err`
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", i,   x)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num =
            self.red as u32 * 16 * 16 +
            self.green as u32 * 16 +
            self.blue as u32;
        write!(f, "RGB ({}, {}, {}) 0x{:06X}", self.red, self.green, self.blue, num)
        // {:06X} - complete with 0s for 6 digits and use hexa format with capitals
    }
}

fn main() {
    println!("{} {} {obj}", 1, 2, obj="valoare");

    println!("{}", Deep(Structure(7))); // use my impl
    println!("{:?}", Deep(Structure(7))); // use debug -> need #[derive(Debug)]

    let name = "Peter";
    let age = 22;
    let peter = Person { name, age };
    println!("{:#?}", peter); // for pretty

    let v = List(vec![1, 2, 3]);
    println!("{}", v); // stuff with ! are macros

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
       Color { red: 128, green: 255, blue: 90 },
       Color { red: 0, green: 3, blue: 254 },
       Color { red: 0, green: 0, blue: 0 },
   ].iter() {
       println!("{}", *color);
   }
}
