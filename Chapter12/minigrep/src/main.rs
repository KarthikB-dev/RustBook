use std::env;

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

    dbg!(args);
}
