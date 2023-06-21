/* Ownership

 - zapewnia bezpieczeństwo (brak nieprzewidywalnych zachowań) dla programów pisanych w Rust https://doc.rust-lang.org/reference/behavior-considered-undefined.html
 - Rust nie pozwala na manualne zarządzanie pamięcią sterty - dzieje się to automatycznie (poziom kompilacji, brak garbage collectora)
 - Zasady:
   - wszystkie dane na stercie mogą mieć tylko jednego właściciela
   - kiedy właściciel przestaje być dostępny (zakończenie zasięgu w którym był zdefiniowany), Rust zwalnia powiązaną z nim pamięć na stercie
   - własność pamięci może zostać przeniesiona w momencie przypisania do innej zmiennej lub wywołania funkcji
   - dostęp do pamięci na stercie może się odbywać wyłącznie za pośrednictwem aktualnego właściciela
 */

fn main() {
    let x = 5; // 1
    let result = add_one(x); // 3
    println!("Result: {result}");

    /* Zmienne proste/prymitywne istnieją w ramach ramek ułożonych na stosie (powiązanych z daną funkcją),
     a w momencie przekazania do funkcji lub przypisania do innej zmiennej ich wartości są kopiowane

      - dla ramki main i pozycji 1 istnieje zmienna x = 5
      - dla ramki add_one i pozycji 2 istnieje zmienna value = 5
      - dla ramki main i pozycji 3 istnieją zmienne x = 5 i result = 6

     W przypadku typów złożonych, alokowanych na stercie, posługujemy się wskaźnikiem (trzymanym na stosie),
     dzięki temu możliwe jest współdzielenie wartości zmiennej bez konieczności jej kopiowania
     Dealokacja pamięci odbywa się automatycznie w momencie niszczenia ramki właściciela
    */

    let numbers = Box::new([1, 2, 3]);
    let my_numbers = numbers; // W pamięci znajduje się jedna tablica i dwa wskaźniki,

    // println!("My numbers: {:?}", numbers); // błąd kompilacji własność zostaje przeniesiona na poziom my_numbers
    println!("My numbers: {:?}", my_numbers);

    let first_name = String::from("Jan");
    // say_hello(first_name);
    // let first_name = other_say_hello(first_name);

    // Alternatywą jest wykorzystanie referencji (specjalny typ wskaźnika) i użyczanie/pożyczanie własności
    better_say_hello(&first_name);
    println!("First name: {:?}", first_name);

    /* Dereferencja
       - może się odbywać manualnie z użyciem *
       - może się odbywać automatycznie - użycie operatora . oraz makra
    */
    let mut x = Box::new(1);
    let a = *x; // *x odczytuje wartość ze sterty czyli 1
    *x += 1;

    /*
    Istnieje stosunkowo dużo zagrożeń wynikających z użycia wielu referencji np.
    (dealokacja lub mutacja pamięci z poziomu innej referencji co może być dla nas zaskoczeniem, próba jednoczesnej mutacji z poziomu wielu referencji)
    Rust narzuca pewne reguły, które mają zapobiegać sytuacjom potencjalnie niebezpiecznym.
    https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html#references-change-permissions-on-paths
    */

    let mut values: Vec<i32> = vec![1, 2, 3];
    let value: &i32 = &values[2];
    println!("Value: {}", value);
    values.push(4);
}

fn add_one(value: i32) -> i32 {
    value + 1 // 2
}

fn say_hello(name: String) {
    println!("Hello {name}");
}

fn other_say_hello(name: String) -> String {
    println!("Hello {name}");
    name
}

fn better_say_hello(name: &String) {
    println!("Hello {name}"); // następuje niejawna dereferencja
}
