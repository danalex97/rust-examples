extern crate timely;

use timely::dataflow::operators::{
    ToStream, Inspect, Map, Filter, Partition, Concatenate
};

pub fn run() {
    timely::execute_from_args(std::env::args(), |worker| {
        // - map takes owned data, so we can mutate it as we like
        // - map_in_place takes a closure which receives a mutable reference
        // - flat_map makes each element into an iterator
        // - filter receives a reference to the data
        worker.dataflow::<(),_,_>(|scope| {
            let streams = (0 .. 5)
                .to_stream(scope)
                .flat_map(|x| 0 .. x)
                .filter(|x| *x < 3)
                .map(|mut x| {
                    x += 1;
                    x
                })
                .map(|x| x.to_string())
                .map_in_place(|x| x.truncate(5))
                .map(|x| x.parse::<u64>().unwrap())
                .partition(3, |x| (x % 3, x));

            // partition only logically partitions the data
            // it does not move the data between workers
            for (i, stream) in streams.iter().enumerate() {
                stream.inspect(move |x| println!("seen {}: {:?}", i, x));
            }

            // concatenation does not exchange data, but works on 'logical' partitions
            scope.concatenate(streams).inspect(|x| println!("combined: {:?}", x));

            // `exchange` - a partition variant that routes records to a worker based on the
            // supplied closure; the exchange operator does not change contents of the stream, but
            // the distribution to the workers; other operations typically use `exchange`, so it
            // is not usually necessary
        });
    }).unwrap();
}
