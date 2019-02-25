use std::env;

mod examples;
use examples::simple_example;
use examples::hello_world;

fn extract(args: Vec<String>) -> Option<String> {
    for arg in args.iter() {
        if let Some(pos) = arg.find("name=") {
            let example = String::from(&arg[(pos + 5)..]);
            return Some(example)
        }
    }
    return None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(example) = extract(args) {
        match example.as_ref() {
            "hello-world" => hello_world::run(),
            "hello_world" => hello_world::run(),
            "simple-example" => simple_example::run(),
            "simple_example" => simple_example::run(),
            _ => ()
        }
    }
}
