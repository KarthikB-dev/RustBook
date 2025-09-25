#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn width(&self) -> bool {
        return self.width > 0;
    }
    fn can_hold(&self, other : &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}

fn main() {
    println!("Hello, world!");
    let h = 2;
    let w = 3;
    let mut rect_area = area(h, w);
    println!("{rect_area}");
    let hw_tuple = (2, 3);
    rect_area = tuple_area(hw_tuple);
    println!("{rect_area}");
    let rectangle1 = Rectangle{width: 2, height: 3};
    rect_area = struct_area(&rectangle1);
    println!("{rect_area}");
    // Won't work if a string representation hasn't been defined
    // println!("{rectangle1}")

    // Won't work because ownership is taken
    // dbg!(rectangle1);
    dbg!(&rectangle1);

    // Works because of line 1 allowing for debug output
    println!("{rectangle1:?}");

    rect_area = rectangle1.area();
    println!("{rect_area}");

    let rectangle2 = Rectangle{width: 4, height : 5};
    let rect2_bigger = rectangle2.can_hold(&rectangle1);
    println!("{rect2_bigger}");
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn struct_area(dimensions: &Rectangle) -> u32 {
    return dimensions.width * dimensions.height;
}