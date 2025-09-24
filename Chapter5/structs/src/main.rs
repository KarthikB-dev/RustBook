struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
}

struct always_equal;

// Normal syntax
fn build_user(username : String, email : String) -> User {
    User {
        active: false,
        sign_in_count : 0,
        email : email,
        username : username,
    }
}

// Field init syntax
fn build_user_init(username : String, email : String) -> User {
    User {
        active: false,
        sign_in_count : 0,
        email,
        username,
    }
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        active: false,
        sign_in_count : 0,
        email : String::from("example@gmail.com"),
        username : String::from("example_user"),
    };
    // User becomes active
    user1.active = true;
    // Syntax sugar for copying most fields
    let user2 = User {
        email : String::from("example2@gmail.com"),
        username : String::from("example_user2"),
        ..user1
    };
    let subject = always_equal;
}
