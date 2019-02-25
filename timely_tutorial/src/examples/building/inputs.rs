extern crate timely;

use timely::dataflow::InputHandle;
use timely::dataflow::operators::ToStream;

pub fn run() {
    timely::execute_from_args(std::env::args(), |worker| {
        let mut input = InputHandle::<(), String>::new();

        // `to_stream` - takes a scope as an argument and produces a stream in that scope.
        worker.dataflow(|scope| {
            input.to_stream(scope);
            (0 .. 9).to_stream(scope);
        });
    }).unwrap();
}
