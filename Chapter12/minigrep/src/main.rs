use std::env;
use std::fs;

fn main() {
    // We don't import args directly because
    // that variable might be used elsewhere.
    // Note that you should use args_os if 
    // you want to read in unicode characters
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("Searching in {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text {contents}");

    dbg!(args);
}
