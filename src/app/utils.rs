pub fn echo(text: &str) {
    println!("{text}");
}

pub struct Calculator {
}

impl Calculator {

    pub fn add(&self, value: f64, other_value: f64) -> f64 {
        value + other_value
    }

}
