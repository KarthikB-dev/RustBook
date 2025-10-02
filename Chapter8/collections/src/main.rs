fn main() {
    let mut v: Vec<i32> = Vec::new();
    v = vec![1, 2, 3];
    v = vec![];
    v.push(5);
    v.push(6);
    v.push(7);
    let third: &i32 = &v[2];
    // panics because it tries to access an out of bounds
    // array index
    // let dne_panic = v[100];

    // optional type, returns None if it can't be found
    let dne_no_panic: Option<&i32> = v.get(100);
    println!("Out of bounds value: {dne_no_panic:?}");

    // Disallowed, we can't have this mutable borrow
    // before the print statement on line 21, and after
    // the immutable borrow on line 8. This is
    // because vectors are copied to new space when
    // capacity is exceeded.
    // v.push(8);

    println!("Third: {third}");

    for curr in &mut v {
        *curr += 10;
    }

    println!("Prior to removal:");
    for curr in &v {
        println!("{curr}");
    }

    // Vectors can't have types mixed like this...
    // let mixed_types = vec!['a', 1, String::new("abc")];

    // But enum types can be mixed!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let mixed_enum_types = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(12.4),
        SpreadsheetCell::Text("Hello".to_string()),
    ];

    v.pop();
    println!("After removal:");
    for curr in &v {
        println!("{curr}");
    }

    // Doesn't work because vectors don't use the display trait 
    // and consequently can't have .to_string() called
    // println!("{v}");

    // Strings are UTF-8 encoded, so Telugu works!
    let mut curry_leaf = String::from("కరివేపాకు");
    println!("{curry_leaf}");
    // We can combine two strings using push_str
    let chutney = String::from(" పచ్చడి");
    // We need a reference here, because taking ownership
    // wouldn't let us print it on line 68
    let mut curry_leaf_chutney = String::from(curry_leaf.clone());
    curry_leaf_chutney.push_str(&chutney);
    println!("{curry_leaf_chutney}");

    let mut plus_chutney = curry_leaf.clone() + &chutney;
    println!("{plus_chutney}");

    let s = format!("{curry_leaf}-{curry_leaf_chutney}-{plus_chutney}");
    println!("{s}");

    let ccl_letters = curry_leaf_chutney.chars();
    for letter in ccl_letters {
        print!("{letter}");
    }
    println!("");
}
