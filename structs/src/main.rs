struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.username);

    let mut user2 = User {
        username: String::from("otheruser"),
        email: String::from("noone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user2.active = false;

    println!("{}", user1.active);

}
