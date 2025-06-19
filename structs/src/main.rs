#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("abir@gmail.com"),
        username: String::from("abir"),
        active: true,
        sign_in_count: 1,
    };
    let name = user1.username;
    user1.username = String::from("wallace123");

    println!("name {name}");

    let user2 = build_user(String::from("ulala"), String::from("abir"));
    let user3 = User {
        email: String::from("abir@zumla"),
        ..user2
    };

    // nameless struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    println!("{:#?}", user3);
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}
