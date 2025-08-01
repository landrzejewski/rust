1. Write a generator of consecutive elements of the Fibonacci sequence (loops, recursion)
   https://en.wikipedia.org/wiki/Fibonacci_sequence

2. Create a type that represents money (monetary amount)
* Money can come in different currencies
* Money can be exchanged/converted to another currency at the indicated exchange rate
* Money can be added and subtracted with each other

3. Write a game of tic-tac-toe
* The board is 3 x 3 fields
* Players take turns to occupy the vacant fields, placing their sign (circle or cross) on them
* The game ends when all fields are occupied or one player occupies the winning sequence (column, row or diagonal)
* The game interface should be based on the command line / terminal

4. Write an application to record receipts/expenditures for the household budget. The application should record the amount,
   type of operation and its description (given as command line arguments) and generate a report/table in terminal.
   Report should contain all the operations and a summary/final balance. The application should save the data entered by the user in a plain text file

5. Implement the following system commands in Rust:
   * echo - prints the text given as an argument to the standard output
   * cat - prints the contents of the indicated files on the standard output, allows optional line numbering, line numbering can be disabled for blank lines
   * wc - prints the number of bytes, characters, words and lines for the indicated files
   * find - searches and prints the paths of files and/or directories whose names match the indicated patterns (use walkdir and regex libs, use iterators)
   * grep - finds and prints lines containing the indicated text/pattern from the indicated files/paths

6. Stworzenie mini systemu do rejestracji i analizy czasu pracy pracowników
   Napisz aplikację CLI, która:
   * Wczytuje dane z pliku `work_log.csv`, zawierającego wpisy czasu pracy w formacie:
   ```
   employee_id,date,start_time,end_time
   E001,2025-07-29,09:00,17:30
   E002,2025-07-29,08:45,17:15
   ...
   ```
   * Oblicza:
   - całkowity przepracowany czas na pracownika
   - średnią długość dnia pracy,
   - dni z nadgodzinami (np. > 8h).
   * Generuje raport (`report.txt`) z wynikami
   
----

https://doc.rust-lang.org/book
https://doc.rust-lang.org/rust-by-example/index.html
https://rustlings.rust-lang.org

Memory management: https://www.youtube.com/watch?v=7_o-YRxf_cc

Books:
   The Rust Programming Language, 2nd Edition
   Effective Rust
   Idiomatic Rust

   Rust Atomics and Locks
   Async Rust
   Write Powerful Rust Macros