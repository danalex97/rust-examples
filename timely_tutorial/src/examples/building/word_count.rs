extern crate timely;

use std::collections::HashMap;

use timely::dataflow::{InputHandle, ProbeHandle};
use timely::dataflow::operators::{Inspect, Probe, Map, Operator};
use timely::dataflow::channels::pact::Exchange;

pub fn run() {
    timely::execute_from_args(std::env::args(), |worker| {
        let index = worker.index();

        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        // define a distribution function for strings(here by length of the text)
        let exchange = Exchange::new(|x: &(String, i64)| (x.0).len() as u64);

        worker.dataflow::<usize,_,_>(|scope| {
            input
                .to_stream(scope)
                .flat_map(|(text, diff): (String, i64)| {
                    text.split_whitespace()
                        .map(move |word| (word.to_owned(), diff))
                        .collect::<Vec<_>>()
                })
                // note that exchange does a `shuffle`
                .unary_frontier(exchange, "WordCount", |_capability, _info| {
                    let mut queues = HashMap::new();
                    let mut counts = HashMap::new();

                    move |input, output| {
                        // read inputs and put them in the queue specific for `time`
                        // we do this since **we could get data out of order**
                        while let Some((time, data)) = input.next() {
                            queues
                                .entry(time.retain())
                                .or_insert(Vec::new())
                                .push(data.replace(Vec::new()));
                        }

                        // we look at each queue; note each key is a Capability<Time>
                        for (key, queue) in queues.iter_mut() {
                            // input frontier says if we can expect more times or not; i.e.
                            // if its bigger than key.time(), is clear we can process the queued
                            // data
                            if !input.frontier().less_equal(key.time()) {
                                // start an output session
                                let mut session = output.session(key);

                                // each queue contains batches of data associated with that
                                // timestamp
                                for mut batch in queue.drain(..) {
                                    // we go through each word
                                    for (word, diff) in batch.drain(..) {
                                        // update the counts
                                        let count = counts.entry(word.clone()).or_insert(0i64);
                                        *count += diff;

                                        // give the pair to output session; that is, the next
                                        // operator will see a stream of (word, count) pairs
                                        session.give((word, *count));
                                    }
                                }
                            }
                        }

                        // If we don't do this we will hang in this operator; that is the session
                        // will always get some data each time the current closure is called; this
                        // means that the downstream will wait for us to send more messages since
                        // we sent messages in this session
                        queues.retain(|_key, val| val.len() > 0)
                    }
                })
                .inspect(move |x| println!("worker {} sees {:?}", index, x))
                .probe_with(&mut probe);
        });

        for round in 0..10 {
            input.send(("roun one".to_owned(), 1));

            // we `advance_to` to tell timely dataflow that we have ceased sending data for
            // round and anything before it
            input.advance_to(round + 1);

            while probe.less_than(input.time()) {
                worker.step();
            }
        }
    }).unwrap();
}
