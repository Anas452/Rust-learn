// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// fn main() {
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
// let s1 = String::from("hello");

// let len = calculate_length(&s1);
// }


// fn main() {
// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, nothing happens.
// }

// //ERROR CODE
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// ERROR FREE CODE 
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

//TWO MUT REFRENCE ERROR
// fn main(){
//   let mut s = String::from("hello");

//   let r1 = &mut s;
//   let r2 = &mut s;

//   println!("{}, {}", r1, r2);

// }

//ERROR IS FIX BY CALLING IT IN A SCOPE
// fn main() {
// let mut s = String::from("hello");

// {
//     let r1 = &mut s;

// } // r1 goes out of scope here, so we can make a new reference with no problems.

// let r2 = &mut s;
// }

//IMMUTABLE AND MUTABLE REFRENCE CANNNOT BE CALLED AT A SAME TIME 
//THERE CAN TWO IMMUTABLE REFRENCES 
// fn main(){
//   let mut s = String::from("hello");

//   let r1 = &s; // no problem
//   let r2 = &s; // no problem
//   let r3 = &mut s; // BIG PROBLEM

//   println!("{}, {}, and {}", r1, r2, r3);

// }

//THIS WILL RUN BECAUSE IMMUTABLE REFRENCE END AT PRINT STATEMENT 
//MUTABLE REFRENCE IS AT OUT OF OF SCOPE

// fn main(){
//   let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // r1 and r2 are no longer used after this point
//   let r3 = &mut s; // no problem
//   println!("{}", r3);
// }

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
