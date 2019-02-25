extern crate timely;
use timely::dataflow::operators::{ToStream, Inspect};

pub fn run() {
    timely::example(|scope| {
        (0..10)
            .to_stream(scope)
            .inspect(|x| {
                println!("seen: {:?}", x)
            });
    });
}
