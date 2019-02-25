extern crate rary;

fn main() {
    rary::public_function();
    rary::indirect_access();
}

// rustc executable.rs --extern rary=library.rlib && ./executable
