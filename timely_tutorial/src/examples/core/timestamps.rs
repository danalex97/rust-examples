#![allow(unused_variables)]
extern crate timely;

use timely::dataflow::InputHandle;
use timely::dataflow::operators::{Input, Exchange, Inspect, Probe};

pub fn run() {
    timely::execute_from_args(std::env::args(), |worker| {
        let index = worker.index();
        let mut input = InputHandle::new();

        // Timestamps are used to correlate inputs and outputs.
        //  When we introduce records with some logical timestamp, unless our dataflow computation changes the timestamps,
        //  we expect to see corresponding outputs with that same timestamp.
        //   - inspect_batch gets access to batches of records with the *same timestamp*
        let probe = worker.dataflow(|scope|
            scope.input_from(&mut input)
                .exchange(|x| *x)
                .inspect_batch(move |t, xs| {
                    for x in xs.iter() {
                        println!("worker {}:\thello {} @ {:?}", index, x, t)
                    }
                })
                .probe()
        );

        // introduce new data
        for round in 0..10 {
            if worker.index() == 0 {
                input.send(round);
            }
            input.advance_to(round + 1);
        }
    }).unwrap();
}
