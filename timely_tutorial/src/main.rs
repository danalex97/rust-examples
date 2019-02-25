use std::env;

mod examples;
use examples::intro::{simple_example, hello_world};
use examples::core::{dataflow};

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
            // intro
            "hello-world" => hello_world::run(),
            "hello_world" => hello_world::run(),
            "simple-example" => simple_example::run(),
            "simple_example" => simple_example::run(),

            // core concepts
            "dataflow" => dataflow::run(),

            _ => ()
        }
    }
}
