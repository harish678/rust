// struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_contract: u64,
}

// function returns struct.. can skip reassign if the field name and param name same
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_contract: 1,
    }
}

// Tuple Structs
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("user1@email.com"),
        username: String::from("user1"),
        active: true,
        sign_in_contract: 1,
    };

    user1.email = String::from("another@email.com");

    let _user2 = build_user(String::from("user2@email.com"), String::from("user2"));

    // create new instance from another instance
    let _user3 = User {
        email: String::from("user3@email.com"),
        ..user1
    };

    // tuple structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}
