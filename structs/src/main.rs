fn main() {
    let user1 = User {
        email: String::from("kevin@gmail.com"),
        username: String::from("kevin"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: String::from("kevin@gmail.com"),
        username: String::from("kevin"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("kevin@gmail.com");

    build_user(String::from("Kevin"), String::from("k@gmail.com"));

    // using update syntax
    let user3 = User {
        email: String::from("k@gmail.com"),
        ..user1 // must come last // values are moved for strings and copied for those that can be copied.
                // Therefore user1 is no longer valid as username, a string, was moved
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    // username: &str, // won't work
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        // username: username,
        // email: email,
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs // unit is () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.
struct AlwaysEqual;
