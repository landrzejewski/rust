use std::env;
use std::fs::*;
use std::io::{BufRead, BufReader, Write};
use rust_training::mult;

mod app {
    pub mod utils;
}

// cargo test -- --test-threads=1 --show-output

#[cfg(test)]
mod test {
    use crate::app::utils::Calculator;

    #[test]
    fn given_two_values_when_add_then_returns_their_sum() {
        let calculator = Calculator {};
        assert_eq!(1.0 + 2.0, calculator.add(1.0, 2.0));
    }
}

fn main() {
    let args: Vec<String> = env::args()
        .collect();
    println!("{:?}", args);

    if let Ok(path) = env::var("PATH") {
        println!("Path value: {}", path);
    }

    if let Ok(content) = read_to_string("notes.md") {
        for line in content.lines().map(|line| line.to_uppercase()) {
            println!("{line}");
        }
    }

    let file = File::open("notes.md")
        .unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(current_line) = line {
            println!("{}: {}", index + 1, current_line)
        }
    }

    // let test_file = File::create("test.txt");

    let test_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("test.txt");

    if let Ok(mut output_file ) = test_file {
        writeln!(output_file, "Test value");
    }
}
