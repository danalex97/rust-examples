extern crate timely;

use std::collections::HashMap;
use timely::dataflow::operators::{ToStream, FrontierNotificator};
use timely::dataflow::operators::generic::operator::Operator;
use timely::dataflow::channels::pact::Pipeline;

use timely::dataflow::operators::generic::operator::source;

pub fn run() {
    timely::example(|scope| {
        (0u64..10)
            .to_stream(scope)
            // unary operator: ready-to-assemble operator w/ 1 input and 1 output
            //   - input distribution: Pipeline - "don't move anything"
            //   - the name
            //   - the logic
            .unary(Pipeline, "increment", |_capability, _info| { // denote `s1`
                // _capability is the default capability of the operator
                let mut vector = Vec::new();

                // return a closure that binds input and output handles
                //  - input.next() => Some((timestamp, data)) (not at None we should return the
                //   control since Timely uses cooperative multitasking)
                // - output.session(timestamp) => starts an output session at the
                //   indicated timestamp
                //    - we can give data to output by `give`, `give_iterator`, `give_content`
                //    - internally it buffers output and flushes at dropping
                move |input, output| {
                    // note that since we used `move`, we can use stateful operations, keeping
                    // the state inside the scope above (`s1`); note that in principle we should
                    // also take into consideration the times of the data when keeping state
                    while let Some((time, data)) = input.next() {
                        data.swap(&mut vector);
                        // note time is actually a capability; look below for explaination
                        let mut session = output.session(&time);
                        for datum in vector.drain(..) {
                            session.give(datum + 1);
                        }
                    }
                }
            });
            // we also have `binary` for binary operators and `operators::source` that can be
            // called with a closure |output| for fetching data from sources
    });

    // operators hold `capabilities` for sending data at any timestamp
    //  - Capability<Time> is a capability that output will request before creating a session
    //  - capability argument(see above) exists so that we can construct operators with the
    //    ability to **send data before they receive** any data
    timely::example(|scope| {
        source(scope, "Source", |capability| {
            // capability is actually an Option<Capability<Time>>
            let mut cap = Some(capability);

            // we use the move operator such that the automatic capture `cap`'s ownership is move
            // in the closure below
            move |output| {
                let mut done = false;
                if let Some(cap) = cap.as_mut() {
                    let time = cap.time().clone();

                    // get some data and send it
                    output
                        .session(&cap)
                        .give(*cap.time());

                    // downgrade capability to be one step in the future; the capability downgrade
                    // will be communicated to others in the downstream
                    cap.downgrade(&(time + 1));
                    done = time > 20;
                }
                if done {
                    // stop transmitting
                    cap = None;
                }
            }
        });
    });

    // capabilities are reported through `frontiers`; each input has an associated `frontier`,
    // which is a description of timestamps that might arrive to that input in the future
    //  - operators may what to check that their output is correct by looking at the times
    //    provided by `input.frontier`(we say time since we may have "partialy ordered" time)
    // ==> we use Notificator - a helper that says when is safe to send data(via frontiers)

    // `concat` example
    timely::example(|scope| {
        let in1 = (0..10).to_stream(scope);
        let in2 = (0..10).to_stream(scope);

        in1.binary_frontier(&in2, Pipeline, Pipeline, "concat_buffer", |_capability, _info| {
            let mut notificator = FrontierNotificator::new();

            // HashMap<Time, Vec<Data>> used to buffer data that is not ready to send
            let mut stash = HashMap::new();

            move |input1, input2, output| {
                // note we can't put this in a vector due to time misatch; we could call
                // a function though
                while let Some((time, data)) = input1.next() {
                    stash.entry(time.time().clone())
                        .or_insert(Vec::new())
                        .push(data.replace(Vec::new()));
                    notificator.notify_at(time.retain());
                }
                while let Some((time, data)) = input2.next() {
                    stash.entry(time.time().clone())
                        .or_insert(Vec::new())
                        .push(data.replace(Vec::new()));
                    notificator.notify_at(time.retain());
                }

                notificator.for_each(&[input1.frontier(), input2.frontier()], |time, _notificator| {
                    let mut session = output.session(&time);
                    if let Some(list) = stash.remove(time.time()) {
                        for mut vector in list.into_iter() {
                            session.give_vec(&mut vector);
                        }
                    }
                });
            }
        });
    });
}
