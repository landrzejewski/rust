// Instalacja rustup + ide + plugin https://rust-book.cs.brown.edu/ch01-01-installation.html

// Sprawdzenie wersji Rust'a
// rustc --version

// Aktualizacja Rust'a
// rustup update

// Lokalna dokumentacja
// rustup doc

// Kompilacja pojedynczego pliku
// rustc main.rs

// Sprawdzenie wersji cargo
// cargo --version

// Utworzenie projektu z użyciem cargo
// cargo new project_name

// Utworzenie biblioteki
// cargo new --lib

// Omówienie Cargo.toml i formatu pliku - TOML (Tom’s Obvious, Minimal Language)

// Zbudowanie projektu z użyciem cargo
// cargo build [--release]

// Uruchomienie projektu z użyciem cargo
// cargo run

// Sprawdzenie projektu pod kątem poprawności kompilacji bez generowania plików wynikowych
// cargo check

// Zbudowanie i uruchomienie testów
// cargo test

// Wyczyszczenie projektu
// cargo clean

// Wygenerowanie dokumentacji
// cargo doc

// Debugowanie

// Zmiana istniejącego toolchain

// rustup toolchain install nightly
// cargo +stable/beta/nightly test
// rustup override set nightly // działa na poziomie projektu

// Inne narzędzia

// rust-clippy, an advanced linter and style tool
// rustup component add clippy
// cargo clippy
// Wyłączanie reguł #[allow(clippy::too_many_arguments)]
// Autofix cargo clippy --fix -Z unstable-options

// rustfmt, an opinionated code formatter
// rustup component add rustfmt
// cargo fmt

// sccache, a compiler cache for rustc
// cargo install sccache
// export RUSTC_WRAPPER=`which sccache`
// cargo build

// Dodawanie zależności do projektu https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
// cargo add rand

// Możliwe jest wskazanie ścieżki do katalogu lub repozytorium np. w celu dokonania tymczasowej poprawki
// [dependencies]
// num_cpus = { path = "num_cpus" }

// [dependencies]
// num_cpus = { git = "https://github.com/xyz", rev = "c463db0a698b035914ae1fd6b7ce5d2a4e727b46" }

// Publikowanie crates
// cargo publish

// Kompilacja na inne platformy
// rustup target list
// rustup target add <target>
// build --target <target>

// Budowanie ze statycznym dołączeniem C runtime
// RUSTFLAGS="-C target-feature=+crt-static" cargo build

// rustup target add x86_64-unknown-linux-musl
// RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-musl

fn main() {
    println!("Hello in Rust World")
}
