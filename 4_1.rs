// fn main() {
//     let s ="hello";
//     {
//       let s = "hi";
//     }
//     println!("{}",s);
// }


// fn main() {
//   let mut s = String::from("hello");
//   s.push_str(",world!");

//   println!("{}",s)
// }


// fn main() {
// {
//     let s = String::from("hello"); // s is valid from this point forward

//     // do stuff with s
// }                                  // this scope is now over, and s is no
//                                    // longer valid
// }



// fn main() {
//   let x = 5;
//   let y = x;
//   println!("{}",y);
// }

// this two example says that when s2 is defined to s1 
// s1 is out of the scope

// fn main() {
// let s1 = String::from("hello");
// let s2 = s1;
// }


// fn main(){
// let s1 = String::from("hello");
// let s2 = s1;

// println!("{}, world!", s1);
// }

// the better way to use the both s1 and s2
// fn main() {
// let s1 = String::from("hello");
// let s2 = s1.clone();

// println!("s1 = {}, s2 = {}", s1, s2);
// }

//in this example we see a integer vlue 
// fn main() {
// let x = 5;
// let y = x;

// println!("x = {}, y = {}", x, y);
// }

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x); 
//     println!("{}",x)  ;               // x would move into the function,
//                                     // but i32 is Copy, so it’s okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    println!("{}",s2)                                    // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}




to get or receive something from someone with the intention of giving it back after a period of time: