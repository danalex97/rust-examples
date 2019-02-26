# [Timely Dataflow Tutorial](http://timelydataflow.github.io/timely-dataflow/introduction.html)

### Introduction

#### Running an example

- Examples: run examples by replacing `<example-name>` in the commands below
  - intro: `hello-hello`, `simple-example`
  - core: `dataflow`, `timestamps`, `progress`
  - building: `inputs`, `outputs`, `operators`, `creating-operators`


- Build and run:
```
cargo build && cargo run -- -name=<example-name>
```

- Use multiple workers:
```
cargo build && cargo run -- -name=<example-name> -w2
```

- Use multiple processes:
```
cargo build && cargo run -- -name=<example-name> -n2 -p0
cargo build && cargo run -- -name=<example-name> -n2 -p1 # in second terminal
```

#### When to use/not use Timely

Advantages of timely: data parallelism, streaming data, expressivity(can express loops).
Use cases: when data does not fit on one computer.

When not to use:
  - when you don't want to move data around: e.g. sorting
  - when data needs to be processed in some particular order: e.g. DFS

### Core Concepts

#### Dataflow

A dataflow is a set of **independent components** which operate in response to input data and connections between **components**. Rather than insisting on a specific sequence of instructions, you provide flexibility to the engine.

#### Timestamps

Data is tagged with **timestamps** to indicate(roughly) when the computation would have happened in a sequential execution.

#### Progress

If the data moves along the dataflow with only increasing timestamps, we can reason about **progress** of our computation. Timestamps that are not *possible* are considered "passed", and components react as they see fit.

### Building Timely Dataflows

- Creating inputs: `inputs`
- Creating outputs: `outputs`
- Adding operators: `operators`
- Creating operators: `creating-operators`
- Word count example: `word-count`

### Running Timely Dataflows

We create the dataflow graph when we define the workflow:
```rust
worker.dataflow::<(),_,_>(|scope| {
    let streams = (0 .. 5)
        .to_stream(scope)
        .flat_map(|x| 0 .. x)
        .filter(|x| *x < 3)
        ...
})
```

However, we start feeding the data to the graph only we advance the inputs and let the worker to run:
```rust
for round in 0..10 {
    input.send(round);
    input.advance_to(round + 1);
    while probe.less_than(input.time()) {
        worker.step();
    }
}
```

The `worker.step()` part is where the actual timely dataflow computation happens. Until this, all data is just building up in queues.

#### Providing input

When `input.send` is called, it moves data from scope to a **queue** shared with the **input dataflow operator**. As the queue starts to fill, it moves data along to the next recipients. This probably means the input queue of next operator, but it may mean network transmission.

When `input.advance_to` is called with a time, the input handler announces it will no longer send data timestamped with anything lower than its argument. This means that any **operator** that was waiting on a(possible) timestamp **can get to work**.

Note that progress tracking is done proportional with the number of timestamps. This means, the smaller the number of timestamps the better in terms of performance.

#### Monitoring probes

**Probes** are ways for the user to monitor progress. A probe handle monitors information that timely provides about the availability of timestamps. We use this information via methods such as `less_than(time)`(check if a time less than `time` is possible).

The most common thing to do with a probe handle is to check whether we are "caught up" to the input times: `probe.less_than(input.time())`. We need to keep in mind the trade-off between overloading and underloading the system.

#### Operator execution

The statement `worker.step()` tells the worker that now is a good time to schedule each of the operators. The system will go through each dataflow operator and call its closure once.

#### Extending dataflows

A worker can spawn an arbitrary number of dataflows and will clean after each one after its complete.

### Advanced Dataflows

- Iterations: `iteration`
- Capture are replay: `capture-replay`
   - `capture_into` is an unary operator that produces no output, basically reacting when frontier changes or when presented with new computations: both cases feeding data via some implementer of EventPusher
   - `replay_into` takes a sequence of events and reproduces a stream as it was recorded

### Internals
