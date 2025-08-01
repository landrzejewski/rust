use crate::exercises::utils::{assert, get_args, min_length};

fn show_help() {
    println!("Usage:");
    println!("grep text path1, path2 ...");
    println!("Args:");
    println!("  text - text to find");
}

fn grep(text: &String, paths: &Vec<String>) {

}

pub fn run() {
    let args = get_args();
    assert(&args, min_length(2), show_help);
    let text = &args[0];
    let paths = crate::exercises::utils::drop(args.clone(), 1);
    grep(&text, &paths);
}
