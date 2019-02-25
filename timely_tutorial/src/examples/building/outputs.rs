extern crate timely;

use timely::dataflow::operators::{ToStream, Inspect, Capture};
use timely::dataflow::operators::capture::Extract;

pub fn run() {
    timely::execute_from_args(std::env::args(), |worker| {
        // inspecting each worker
        worker.dataflow::<(),_,_>(|scope| {
            (0 .. 5)
                .to_stream(scope) // creates a stream in this scope
                .inspect(|x| println!("hello: {}", x)); // and we inspect that stream
        });

        // inspect_batch if you want process the outputs more efficiently
        worker.dataflow::<(),_,_>(|scope| {
            (0 .. 9)
                .to_stream(scope)
                .inspect_batch(|t, xs| println!("{:?} @ {:?}", xs, t));
        });
    }).unwrap();

    // capturing streams - the Capture trait provides a mechanism for exfiltrating a stream
    // from a dataflow, into information that can be replayed in other dataflows
    let (data1, data2) = timely::example(|scope| {
        // data1 is the receive side of Rust's threadsafe channel
        //   - data1 contains events
        let data1 = (0 .. 3).to_stream(scope).capture();
        let data2 = vec![0,1,2].to_stream(scope).capture();
        (data1, data2)
    });
    assert_eq!(data1.extract(), data2.extract());
}
