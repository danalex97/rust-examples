#![allow(unused_variables)]
extern crate timely;

use timely::dataflow::InputHandle;
use timely::dataflow::operators::{Input, Exchange, Inspect, Probe};

pub fn run() {
    timely::execute_from_args(std::env::args(), |worker| {
        let index = worker.index();
        let mut input = InputHandle::new();

        // Create a new input, exchange data, and inspect its output:
        //   - there are 2 dataflow operators: exchange and inspect
        //   - the `exchange` operator gets datum and hands it downstream
        //   - the `inspect` oprator takes action for each datum
        let probe = worker.dataflow(|scope|
            scope.input_from(&mut input)
                .exchange(|x| *x)
                .inspect(|x| {
                    let limit = (*x as f64).sqrt() as u64;
                    if *x > 1 && (2 .. limit + 1).all(|i| x % i > 0) {
                        println!("prime number found: {}", x)
                    }
                })
                .probe()
        );

        // introduce new data
        for round in 0..100_000 {
            if worker.index() == 0 {
                input.send(round);
            }
            input.advance_to(round + 1);
        }
    }).unwrap();
}
