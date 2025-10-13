// Non generic version - what if this could be generalized
// to other types?
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            return item;
        }
    }
    largest
}

// Generic version!
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            return item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.x.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is: {result}");

    let int_point = Point { x: 4, y: 5 };
    println!("int_point.x = {}", int_point.x());
    let mixed_point = MixedPoint { x: 4, y: 5.45 };

    let f32_point = Point { x: 1.414, y: 1.732 };
    println!(
        "f32_point origin distance = {}",
        f32_point.distance_from_origin()
    );

    let integer = Some(32);
    let float = Some(32.5);
}
