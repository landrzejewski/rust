use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::{DirEntry, WalkDir};
use crate::exercises::utils::{assert, get_args, min_length};

fn show_help() {
    println!("Usage:");
    println!("grep text path1, path2 ...");
    println!("Args:");
    println!("  text - text to find");
}

fn find_file_paths(path: &String) -> Vec<String> {
    let is_file = |entry: &DirEntry| entry.path().is_file();
    let to_file_path =  |entry: DirEntry| entry.path().display().to_string();
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(is_file)
        .map(to_file_path)
        .collect()
}

fn get_matching_lines(text: &String, file_path: &String) -> Vec<(usize, String)> {
    let Ok(file) = File::open(file_path) else {
        eprintln!("Unable to open file: {file_path}");
        return Vec::new();
    };
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(index, line)| (index + 1, line))
        .filter(|(_, line)| line.contains(text))
        .collect()
}

fn print_matching_lines(matching_lines: &Vec<(usize, String)>) {
    matching_lines.iter().for_each(|(line_number, line)| {
        println!("[{:6}]: {}", line_number, line);
    })
}

fn grep(text: &String, paths: &Vec<String>) {
    paths.iter()
        .flat_map(find_file_paths)
        .map(|file_path| (file_path.clone(), get_matching_lines(text, &file_path)))
        .for_each(|(file, matching_lines)| {
            println!("File: {file}");
            print_matching_lines(&matching_lines);
        });
}

pub fn run() {
    let args = get_args();
    assert(&args, min_length(2), show_help);
    let text = &args[0];
    let paths = crate::exercises::utils::drop(args.clone(), 1);
    grep(&text, &paths);
}
