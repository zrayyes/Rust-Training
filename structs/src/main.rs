struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.username);

    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    user2.active = false;

    println!("{}", user2.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
