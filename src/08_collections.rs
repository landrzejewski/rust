use std::collections::HashMap;

fn main() {
    // Vector

    /*let mut numbers:Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);*/

    let mut numbers = vec![1, 2, 3];
    let selected_number = numbers[0];
    // numbers[4]; // panic - index out of range

    if let Some(number) = numbers.get(2) {
        println!("Value with index 2: {number}");
    }

    for number in &mut numbers {
        println!("Number: {number}");
    }

    let mut itearator = numbers.iter();
    // let number = itearator.next();

    let result: Vec<i32> = itearator
        .map(|number| number + 1)
        .filter(|number| number % 2 == 0)
        .collect();

    // String

    let text = "Łukasz".to_string(); // String::from("Łukasz");
    for letter in text.chars() {
        println!("Letter: {letter}");
    }
    let first_letter = text.chars().nth(0); // bezpieczny dostęp
    let text_length = text.len();

    let text_slice = &text[0..2];
    println!("Text slice: {text_slice}");

    // HashMap

    let mut ratings: HashMap<&str, i32> = HashMap::new();
    ratings.insert("a", 10);
    ratings.insert("b", 11);
    let rate = ratings.entry("a").or_insert(12);

    if let Some(rate) = ratings.get("a") {
        println!("Rate: {rate}");
    }

    for (key, rate) in &ratings {
        println!("Rate: {}:{rate}", *key);
    }
}
