use std::fs::File;
use std::io::ErrorKind;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // panic!("crash and burn...")
    let v = vec![1, 2, 3];
    // let oob = v[2000];
    // println!("{oob}");
    let greeting_file_result = File::open("Hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
