#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn toggle_activity(&mut self) {
        self.active = !self.active
    }
    fn have_conflict(&self, other: &User) -> bool {
        return self.username == other.username && self.email > other.email;
    }
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
    let mut user3 = User {
        email: String::from("abir@zumla"),
        ..user2
    };

    // nameless struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let x = Color(1, 2, 3);
    println!("{} value", x.1);
    user3.toggle_activity();
    println!("{:#?}", user3);
    let response = user3.have_conflict(&user1);
    println!("{response}")
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}
