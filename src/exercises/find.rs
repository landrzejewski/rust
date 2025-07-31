use crate::exercises::utils::{assert, get_args, is_not_empty, min_length};
use regex::Regex;
use std::io::{Stdout, Write, stdout};
use walkdir::{DirEntry, WalkDir};

const SEPARATOR: &str = ",";

enum ElementType {
    Dir,
    File,
    Link,
}

impl From<&str> for ElementType {
    fn from(value: &str) -> Self {
        match value {
            "dir" => ElementType::Dir,
            "file" => ElementType::File,
            "link" => ElementType::Link,
            _ => ElementType::File,
        }
    }
}

fn show_help() {
    println!("Usage:");
    println!("find regexp t1,t2,t3 path1 path2 ...");
    println!("options:");
    println!("  regexp - match/regular expression");
    println!("  types - one or many types separated by comma. Types: dir,file,link");
}

fn is_type_of(entry: &DirEntry, element_type: &ElementType) -> bool {
    let file_type = entry.file_type();
    match element_type {
        ElementType::Dir => file_type.is_dir(),
        ElementType::File => file_type.is_file(),
        ElementType::Link => file_type.is_symlink(),
    }
}

fn find(regex: &Regex, types: &Vec<ElementType>, paths: &Vec<&String>) -> Vec<String> {
    let by_type = |entry: &DirEntry| {
        types
            .iter()
            .any(|element_type| is_type_of(entry, element_type))
    };

    let by_name = |entry: &DirEntry| regex.is_match(entry.file_name().to_str().unwrap_or_default());

    let entry_to_string = |entry: DirEntry| entry.path().display().to_string();

    let find_on_path = |path: &String| {
        WalkDir::new(path)
            .into_iter()
            .flatten()
            .filter(by_type)
            .filter(by_name)
            .map(entry_to_string)
    };

    let mut stdout = stdout();
    paths.iter().enumerate().fold(Vec::new(), |mut acc, entry| {
        let (index, path) = entry;
        print_progress(&mut stdout, index + 1, paths.len());
        acc.extend(find_on_path(path));
        acc
    })
}

pub fn run() {
    let args = get_args();
    assert(&args, min_length(3), show_help);
    let regex = Regex::new(&args[0]).expect("Invalid regex expression");
    let types: Vec<ElementType> = args[1]
        .split(SEPARATOR)
        .map(|element| element.trim())
        .map(ElementType::from)
        .collect();
    assert(&types, is_not_empty, show_help);
    let paths = args.iter().skip(2).collect::<Vec<_>>();
    assert(&paths, is_not_empty, show_help);
    find(&regex, &types, &paths)
        .iter()
        .for_each(|path| println!("{path}"))
}

fn print_progress(stdout: &mut Stdout, step: usize, total_steps: usize) {
    // stdout.write(format!("\rProcessing {}%", (step * 100) / total).as_bytes());
    print!("\rSearching... {}%", (step * 100) / total_steps);
    _ = stdout.flush();
    if step == total_steps {
        println!();
    }
}
