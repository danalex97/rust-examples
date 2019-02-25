# [Timely Dataflow Tutorial](http://timelydataflow.github.io/timely-dataflow/introduction.html)

### Introduction

#### Running an example

- Examples:
  - intro: `hello-hello`, `simple-example`
  - core: `dataflow`, `timestamps`, `progress`

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
