# Timely Dataflow Tutorial

#### Running an example

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
