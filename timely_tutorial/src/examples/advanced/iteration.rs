extern crate timely;

use timely::dataflow::operators::*;

pub fn run() {
    println!("Simple loop example");
    timely::example(|scope| {
        let (handle, stream) = scope.feedback(1);

        (1 .. 5)
            .to_stream(scope)
            .concat(&stream)
            .map(|x| if x % 2 == 0 {x / 2} else {3 * x + 1})
            .inspect(|x| print!("{:?} ", x))
            .filter(|x| *x != 1)
            .connect_loop(handle);
    });
    println!();

    // note we can also use scopes if necessary:
    //   see http://timelydataflow.github.io/timely-dataflow/chapter_4/chapter_4_1.html
    println!("Two loops");
    timely::example(|scope| {
        let (handle0, stream0) = scope.feedback(1);
        let (handle1, stream1) = scope.feedback(1);

        let results0 = stream0.map(|x| x / 2).filter(|x| *x != 1);
        let results1 = stream1.map(|x| {3 * x + 1}).filter(|x| *x != 1);

        // note what happens here is that stream created by `to_stream` gets empty after being
        // concatenated and no more data is passed to it; this means that only the stream0 and
        // stream1 loops will be fed when we do connect_loop
        let parts =
            (1 .. 5)
                .to_stream(scope)
                .concat(&results0)
                .concat(&results1)
                .inspect(|x| print!("{:?} ", x))
                .partition(2, |x| (x % 2, x));

        parts[0].connect_loop(handle0);
        parts[1].connect_loop(handle1);
    });
    println!();

    // exercise
    println!("Two loops exercise");
    timely::example(|scope| {
        let (handle0, stream0) = scope.feedback(1);
        let (handle1, stream1) = scope.feedback(1);

        let results0 = stream0.map(|x: (u64, u64)| (x.0, x.1 / 2)).filter(|x| x.1 != 1);
        let results1 = stream1.map(|x: (u64, u64)| (x.0, (3 * x.1 + 1))).filter(|x| x.1 != 1);

        // note what happens here is that stream created by `to_stream` gets empty after being
        // concatenated and no more data is passed to it; this means that only the stream0 and
        // stream1 loops will be fed when we do connect_loop
        let parts =
            (1u64 .. 5)
                .to_stream(scope)
                .map(|x: u64| (x, x))
                .concat(&results0)
                .concat(&results1)
                .inspect(|x| {
                    let (started, now) = x;
                    print!("({},{}) ", started, now)
                })
                .partition(2, |x| (x.1 % 2, x));

        parts[0].connect_loop(handle0);
        parts[1].connect_loop(handle1);
    });
    println!();
}
