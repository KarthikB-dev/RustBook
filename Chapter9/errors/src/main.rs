use std::io::{self, Read, ErrorKind};
use std::error::Error;
use std::fs::File;

enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn last_char_of_first_line(text: &str) {
    text.lines().next()?.chars().last();
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Returns an error if this fails
    let mut username = String::new();
    let mut username_file = File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username);
}

// Box is a trait object -> means any kind of error
fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn...")
    let v = vec![1, 2, 3];
    // let oob = v[2000];
    // println!("{oob}");
    // let greeting_file_result = File::open("Hello.txt");


    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    let greeting_file = File::open("hello.txt")?;

    let biography = File::open("bio.txt").expect("bio.txt should be included in this project");
}
