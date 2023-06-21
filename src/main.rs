/* Structs

 - grupują elementy dowolnego typy, ale w przeciwieństwie do krotek, pozwalają na ich nazwanie
 - pozwalają na stworzenie wielu instancji mających takie same właściwości (odpowiednik obiektów z innych języków)
 - dostęp do elementów struktury odbywa się za pomocą operatora .
 - jeśli instancja jest mutowalna można modyfikować jej pola
 */

fn main() {
    let mut account = Account {
        email: String::from("jan@training.pl"), // dzięki użyciu String::from struktura posiada indywidualną kopię tekstu
        password: String::from("123"),
        active: true
    };
    println!("{:?}", account);
    account.active = false;
    println!("{:?}", account);

    let other_account = Account {
        email: String::from("marek@training.pl"),
        ..account // skopiowanie pozostałych pól (odbywa się z przeniesieniem własności)
    };
    println!("Other account: {:?}", other_account);

    // println!("{:?}", account.password); // błąd - po skopiowaniu elementów do other_account straciliśmy częściowo własność (typy referencyjne)
    // println!("{:?}", account); // błąd - po skopiowaniu elementów do other_account straciliśmy częściowo własność (typy referencyjne)

    // Borrow checker śledzi własność i uprawnienia zarówno na poziomie samej struktury jak i jej pól https://rust-book.cs.brown.edu/ch05-01-defining-structs.html#borrowing-fields-of-a-struct
    // Structs i ownership https://rust-book.cs.brown.edu/ch05-03-method-syntax.html#methods-and-ownership

    // Struktury podobnie jak Traits i Enums pozwalają na definiowanie metod. W takim przypadku ich pierwszym parametrem jest zawsze &self (instancja struktury)

    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", rectangle.area()); // == Rectangle::area(&rectangle);

    let square = Rectangle::square(60);

    // Rust pozwala na zdefiniowanie tzw. tuple structs (nazwanych krotek) oraz pustych struktur (pod kątem Traits)

    /*let origin = Point(0, 0);
    let x = origin.0;*/

    // Destrukcja struktury

    let point = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = point;
    let Point { x, y } = point;
    println!("a, b: {}, {}", a, b);
    println!("x, y: {}, {}", a, b);

    match point {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    match point {
        Point { x, .. } => println!("x is {}", x),  // ignorowanie pozostałych elementów struktury
    }
}


#[derive(Debug)]
struct Account {
    active: bool,
    email: String,
    password: String
}

// struct Point(i32, i32);

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {  // &self to skrót self: &Self lub self: &Rectangle
        self.width * self.height
    }

    fn width(&self) -> u32 {  // można rozdzielić metody na wiele bloków impl
        self.width
    }

    fn square(size: u32) -> Self { // Associated function, często używana do tworzenia nowych instancji, jak String::new, String::from
        Self {  // W tym przypadku Self to alias do Rectangle
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
