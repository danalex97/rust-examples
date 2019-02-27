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

### Progress Tracking

Data bear a **logical timestamp**, indicating some moment in the computation at which they should be executed. This is not necessarily a physical timestamp, but rather something similar to an *epoch*.

In order to send a timestamped message, an operator must hold a **capability for that timestamp**. Progress tracking can be viewed as:
- workers collectively maintaining a view of outstanding(see active in Naiad) timestamp capabilities at each location in dataflow graph
- workers independently determine and communicate the change in their view of capabilities

#### Dataflow structure

Each operator is **identified by an index** and has associated **multiple input and output ports**. We define `Source` and `Target` as an `(index, port)` pair.

#### Maintaining capabilities

Capabilities can exist in two places in timely dataflow:
- an operator can hold capabilities to send timestamped messages to an output
- a timestamped message bears a capability for its timestamp

Operators can:
- consume input messages, acquiring a capability
- may clone, downgrade or drop any capability
- send output messages for which they have a capability

We track **how many** capabilities for **time t** are there at **location l**. As messages get consumed the number of capabilities at a Target will decrease and the number of the capabilities of the corresponding Source will increase. Cloning, downgrading and dropping changes the counts as well. This can be represented as a stream as well.

Each worker **broadcasts** the stream of progress change batches to all workers in the system along point-to-point FIFO channels. This means that an arbitrary point in time, each worker has seen an **arbitrary prefix** of the sequence of progress made in the system by each worker. Intuitively, this *helps* guarantee safety properties since each worker has a view of a system with harder constraint on the current capabilities.

#### Path Summaries

Each worker maintains an "accumulation" of progress update batches, which explains where capabilities may exist in the dataflow graph. However, the paths that the information travels in the graph is relevant to **guarantee** safety properties.

A **path summary** is informally meant to summarize what must happen to a timestamp as it travels along a path in a timely dataflow.
```rust
pub trait PathSummary<T> : PartialOrder {
    fn results_in(&self, src: &T) -> Option<T>;
    fn followed_by(&self, other: &Self) -> Option<Self>;
}
```

The types implementing PathSummary must be partially ordered, and implement two methods:
- `results_in` explains what must happen to a timestamp moving along a path; note `results_in` must advance(>=) the timestamp
- `followed_by` explains how 2 path summaries combine

Two path summaries are ordered if for all timestamps the two results of the path summaries applied to the timestamp are also ordered.(notice we use both fields in structure) Since we can go along many paths, we talk about collections of Path summaries.

#### Operator Summaries

An **operator summary** needs to provide a collection of path summaries for each of its internal paths, **from each of its inputs to each of its outputs**. This means the operation must describe what timestamps could result in any outcome. The most common example is "timestamped data at each of my inputs could result in equivalently timestamped data at each of my outputs".

The **Feedback operator** is where most of our interesting path summaries start: it is the operator found in feedback loops that ensures that a specific coordinate of the timestamp(see Naiad) is incremented. All cycles in a timely dataflow graph must pass through such an operator.

**From the operator summaries we build path summaries, and from the path summaries we determine, for every pair of either Source or Target a collection of path summaries between the two.**

#### Safety

Each operator maintains a **collection of counts of timestamp capabilities** at each location (Source or Target) in the dataflow graph. At the same time, there is a statically defined **set of path summaries** from each location to any other location in the dataflow graph.

**Property:** for any collection of counts resulting from the accumulation of arbitrary prefixes of progress update batches(*in the progress stream*) from participating workers, ∀ location-timestamp pair (l1, t1) if ∃ (l2, t2) with **accumulated count > 0** s.t. ∃ path summary p from l2 to l1 with p(t2) <= t1, then no message will ever arrive at location l1 bearing timestamp t1.

For a simple explanation of this property, check [this link](http://timelydataflow.github.io/timely-dataflow/chapter_5/chapter_5_2.html#a-safety-property).
