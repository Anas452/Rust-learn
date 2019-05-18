fn main() {
    
    struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
    };

    let mut user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
    };
user1.email = String::from("another@example.com");
println!("{}",user1.email)
}

fn main() {
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
}
