#![allow(unused_variables)]
extern crate timely;

use timely::dataflow::InputHandle;
use timely::dataflow::operators::{Input, Exchange, Inspect, Probe};

pub fn run() {
    timely::execute_from_args(std::env::args(), |worker| {
        let index = worker.index();
        let mut input = InputHandle::new();

        // putting a probe after after inspect tells us whether
        // we should expect to see the **method associated with inspect fire again**
        // for a given timestamp
        let probe = worker.dataflow(|scope|
            scope.input_from(&mut input)
                .exchange(|x| *x)
                .inspect(move |x| println!("worker {}:\thello {}", index, x))
                .probe()
        );

        for round in 0..10 {
            if worker.index() == 0 {
                input.send(round);
            }

            // `input` - how we provide data to dataflow computation and it has timestamp associated with it
            //   - we can add data with that timestamp or greater timestamp
            //   - we can advance timestamp with advance_to: the timestamp used are restricted to the argument(here `round + 1`)
            //   -- the time in `advance_to` is announced to all other workers
            input.advance_to(round + 1);

            // `probe` - how we learn about timestamp data at some point in dataflow graph
            //   -  we can consult a probe with `less_than` to ask wheather there is a time less
            //      that argument at that point in dataflow graph

            // progress info is passive --> we see what is state of system
            //  - that means the implementation can choose to synchronize using the data probes
            //    shared in the system
            worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}
