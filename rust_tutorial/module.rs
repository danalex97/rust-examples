mod my_mod {
    // Items in modules default to private visibility.
    #[allow(dead_code)]
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // pub(crate) makes functions visible only within the current crate
    #[allow(dead_code)]
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // Modules can also be nested
    #[allow(dead_code)]
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        // pub(super) fn public_function_in_super_mod() {
        pub(in my_mod) fn public_function_in_my_mod() {
            // node we can use self:: or super:: to disambiguate function calls
        }
    }
}

#[allow(dead_code)]
mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}


// extern crate deeply; // normally, this would exist and not be commented out!
// use deeply::nested::{
//     my_first_function as batman,
//     my_second_function,
//     AndATraitType
// };

mod my_module;

fn function() {
    println!("called `function()`");
}

fn main() {
    // Modules allow disambiguation between items that have the same name.
    my_mod::function();

    function();
    my_module::function();
    my_module::indirect_access();
    my_module::nested::function();
}
