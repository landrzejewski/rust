/*
https://github.com/landrzejewski/rust-training

Installation/environment setup
- rustup tool from https://rustup.rs
- Visual Code + Rust extension, alternatively RustRover
- git

Important commands:
rustup --version                       # check rustup and rustc version
rustc main.rs                          # compile a file
rustfmt main.rs                        # format source a file
cargo new training_project             # create new project with cargo tool
cargo build                            # build an application in debug mode
cargo run                              # build and run an application in debug mode
cargo build --release                  # build an application in release mode
cargo check                            # check/build code without generating executables
cargo fmt                              # format source files in the project
cargo clippy                           # lint project
cargo clean                            # clean project
*/
use crate::exercises::employees::run;
use crate::exercises::tic_tac_toe;

mod exercises;
mod memory_management;
mod language_basics;
mod collections_generics_traits;
mod threads;

macro_rules! say_hello {
    () => {
        println!("Say hello!");
    };
}

macro_rules! greet {
    ($name:expr) => {
        println!("Hello {}!", $name);
    };
}

macro_rules! calc {
    ($a:literal, +, $b:literal) => {
        $a + $b
    };
}

macro_rules! vector {
    ($($x:expr), *) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
}

fn main() {
    run().unwrap();
}

