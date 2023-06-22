use std::fmt::{Display, Formatter};

fn main() {
    println!("Value as string: {}", to_string(true));
    let point = Point {
        x: 30,
        y: 20
    };
    println!("{}", point);
    point.show();
    point.show_info();
}

fn i32_to_string(value: i32) -> String {
    format!("{value}")
}

fn f64_to_string(value: f64) -> String {
    format!("{value}")
}

fn to_string<T: Display>(value: T) -> String {
    format!("{value}")
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T: Display + Clone> Display for Point<T> {
// impl<T> Display for Point<T> where T: Display + Clone {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }

}

impl<T: Display + Clone> Show for Point<T> {

    fn show(&self) {
        println!("{}", &self);
    }

}

trait Show {

    fn show(&self);

    fn show_info(&self) {
        println!("Info");
    }

}

trait Number {
}

fn show_instance<T: Show>(instance: &T) {
// fn show_instance(instance: &impl Show) {
    instance.show();
}

fn show_instance_factory() -> impl Show {
    Point {
        x: 0,
        y: 1
    }
}

trait Iterator {

    type I;

    fn next(&mut self) -> Option<Self::I>;

}

impl Iterator for Point<T> {

    type I = i32;

    fn next(&mut self) -> Option<Self::I> {
        self.x
    }

}

