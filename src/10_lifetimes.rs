fn main() {
    let x;
    {
        let y = 10;
        x = &y;
    }
    // println!("x: {x}"); // błąd - próba użycia zmienna, która już nie istnieje

    // Lifetime nie zmienia czasu życia referencji, ale opisuje relacje między czasami życia wielu referencji

    let result = get_longer("Hi", "Hello");

    let last_name = "Kowalski";

    let address = "test";

    let client = Person {
        first_name: "Jan",
        last_name,
        address
    };
}

/*
  - dla metod, które mają dokładnie jeden argument, kompilator przypisuje ten sam lifetime parameter do argumentu jak i rezultatu
  - dla metod z argumentami, kompilator przypisuje różne (kolejne) lifetime parameters do argumentów oraz rezultatu
  - dla metod z argumentami, które zawierają &self lub &mut self to wtedy lifetime przypisany do atrybutu  &self lub &mut self jest przypisany do rezultaty
 */

fn get_longer<'a>(text: &'a str, other_text: &'a str) -> &'a str { // w tym przypadku zwracana referencja musi być ważna tak długo jak referencje przekazywanych argumentów
    if text.len() >= other_text.len() { text } else { other_text }
}

struct Person<'a, T> {  // instancja Person nie może przetrwać dłużej niż referencje, które posiada / przechowuje
    first_name: &'a str,
    last_name: &'static str, // static oznacza czas życia całego programu
    address: T
}

