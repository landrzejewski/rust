#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // Zmienne

    let x = 10; // deklaracja zmiennej niemutowalnej, typ jest wnioskowany automatycznie
    println!("The value of x is: {x}");
    // x = 6; błąd kompilacji - zmienna nie może zmienić swojej wartości

    let y; // nie ma konieczności deklaracji z jednoczesną inicjalizacją, ale musi ona nastąpić przed pierwszym użyciem
    y = 10;
    println!("The value of y is: {y}");

    let mut z = 10; // deklaracja zmiennej mutowalnej
    z = 30;
    println!("The value of z is: {z}");

    let _o = 10; // zmienna nie będzie powodować warning, nawet jeśli nigdy nie będzie użyta

    // Shadowing

    let x = 20; // nadpisanie zmiennej, umożliwia zmianę typu oraz tego czy zmienna jest mutowalna
    {
        let x: f32 = x as f32 * 3.0;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");

    /* Stałe

     - muszą mieć określony typ - nie jest on wnioskowany automatycznie
     - ich wartość musi być znana w czasie kompilacji
     - nie mogą zmieniać swojej wartości (użycie mut nie jest dozwolone)
     - mogą mieć dowolny zasięg, także globalny
    */
    const MONTH_OF_THE_YEAR: i8 = 4;
    const TIMEOUT: i64 = 3600 * 10;

    /* Typy danych

    - muszą być znane/określone w czasie kompilacji - Rust jest statycznie typowany
    - w większości przypadków mogą być wywnioskowane automatycznie przez kompilator
    - typy skalarne/proste - integers, floating-point numbers, booleans and characters
    - typy złożone - tuples, arrays
    */

    /* Integers

     Length	   Signed type    Unsigned type
     8-bit	   i8	          u8
     16-bit	   i16	          u16
     32-bit	   i32 // default u32
     64-bit	   i64	          u64
     128-bit   i128	          u128

     arch	isize	usize

     Number literals	   Example
     Decimal	           98_222_000
     Hex	               0xff
     Octal	               0o77
     Binary	               0b1111_0000
     Byte (u8 only)	       b'A'

     - w trybie debug kompilator dodaje weryfikację wystąpienia integer overflow (asercja) i przerywa wykonanie programu
       w przypadku jego wystąpienia
     */

    // let a: u8 = 300; // integer overflow

    /* Floating-point numbers

     - zgodne ze standardem IEEE-754

     Length	   Type
     32-bit	   f32
     64-bit	   f64 // default
     */

    let value = 14.3;
    let small_value: f32 = 0.1;

    // Wspierane są standardowe operatory matematyczne https://rust-book.cs.brown.edu/appendix-02-operators.html
    // Uwaga: w przeciwieństwie do innych języków należy zadbać o jawną konwersję typów

    let sum = 5 + 10;
    let difference = 11.5 - 4 as f32;
    let product = 4.0 * 12.5;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // obcięcie wyniku do 0
    let remainder = 54 % 5;

    /* Boolean

     - dopuszczalne wartości to true i false
     */

    let positive_result = true;
    let negative_result: bool = false;

    /* char

    - reprezentuje Unicode Scalar Value (może przechowywać złożone znaki)
    - ma rozmiar 4 bajtów
    */

    let c = 'a';
    let z: char = 'ℤ';
    let cat = '😻';

    /* Tuple

     - grupuje elementy dowolnego typu
     - ma z góry określoną i niezmienną długość
     - dostęp do poszczególnych elementów odbywa się przez indeksy/pozycję lub destrukcję
     */

    let tuple = (11, 11.0, 11);
    let first_value = tuple.0; // dostęp do elementu z użyciem indeksu/pozycji
    let (a, b, c) = tuple; // destrukcja krotki

    let unit = (); // pusta krotka, unit

    /* Array

    - grupuje elementy tego samego typu
    - ma z góry określoną i niezmienną długość
    - dostęp do elementów odbywa się przez indeksy, przekroczenie poprawnego zakresu kończy się przerwaniem programu (panic)
    - alokacja pamięci następuje na stosie (stack) jak dla typów prymitywnych
    */

    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("First number: {}", numbers[0]);
    let zeros = [0; 7]; // stworzenie tablicy o danym rozmiarze, wypełnionej
    println!("Array of zeros: {:?}", zeros);

    // Funkcje i wyrażenia

    println!("Addition result: {}", add(1, 2));
    println!("Validation result: {}", validate(10, is_even));
    println!("Validation result: {}", validate(10, |value| value % 2 == 0));

    let score = {
        let x = 3;
        x * 3
    };

    // Control flow

    /* Wyrażenia warunkowe

     - pozwalają na wykonywanie sekcji kodu po spełnieniu/niespełnieniu określonego warunku
     - wynik wyrażenia warunkowego musi być typu bool (nie ma niejawnej konwersji ja w niektórych językach)
     - zwracają rezultat, który może zostać przypisany do zmiennej lub wykorzystany w inny sposób
       (zwracane wyniki muszą być tego samego typu, trzeba zadbać o wszystkie możliwe scenariusze)
    */

    let number = 3;
    if number < 5 {
        println!("Number is lower than 5");
    } else if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is equal 5");
    }

    let some_condition = true;
    let option = if some_condition { 'a' } else { 'b' };

    // Pattern matching

    let dice_roll = 3;
    let roll_result = match dice_roll {
        6 => {
            println!("You won!");
            "Win"
        },
        1 => {
            println!("You lost!");
            "Loose"
        },
        value=> {  // jeśli wartość nas nie interesuje można użyć _
            println!("You hit {value}, try again");
            "Try again"
        }
    };

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        _ => ()
    }

    // Pętle

    // loop

    let mut counter = 0;
    let result = 'myLoop: loop {
        counter += 1;
        if counter == 20 {
            break 'myLoop counter * 2;
        }
    };
    println!("The loop result is: {result}");

    // while

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // for in

    let elements = [1, 2, 3, 4, 5];

    for index in 0..4 {
        println!("Current element: {}", elements[index]);
    }

    for element in elements {
        println!("Current element: {}", element);
    }

    for index in (0..elements.len() -1).step_by(2) {
        println!("Current element: {}", elements[index]);
    }

}

fn add(value: i32, other_value: i32) -> i32 {
    let result = value + other_value;
    //return result;
    //result
    value + other_value // dodanie średnika spowoduje błąd - funkcja będzie wtedy zwracać ()
}

fn validate<T>(value: T, predicate: fn(T) -> bool) -> bool {
    predicate(value)
}

fn is_even(value: i32) -> bool {
    value % 2 == 0
}
