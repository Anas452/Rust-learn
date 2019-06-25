// fn main() {
    
//     struct User {
//       username: String,
//       email: String,
//       sign_in_count: u64,
//       active: bool,
//     };

//     let mut user1 = User {
//       email: String::from("someone@example.com"),
//       username: String::from("someusername123"),
//       active: true,
//       sign_in_count: 1,
//     };
// user1.email = String::from("another@example.com");
// println!("{}",user1.email)
// }


// #![allow(unused_variables)]
// fn main() {
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// };
// build_user(anas, inn);
// }


// #![allow(unused_variables)]
// fn main() {
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// let user1 = User {
//     email: String::from("someone@example.com"),
//     username: String::from("someusername123"),
//     active: true,
//     sign_in_count: 1,
// };

// let user2 = User {
//     email: String::from("another@example.com"),
//     username: String::from("anotherusername567"),
//     ..user1
// };
// }


// #![allow(unused_variables)]
// fn main() {
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// let black = Color(0, 0, 0);
// let origin = Point(0, 0, 0);

// println!("{}",black);
// }

// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: "someone@example.com",
//         username: "someusername123",
//         active: true,
//         sign_in_count: 1,
//     };
// }
