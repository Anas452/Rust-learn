
// #![allow(unused_variables)]
// fn main() {
// let v = vec![1, 2, 3, 4, 5];

// let third: &i32 = &v[4];
// println!("The third element is {}", third);

// match v.get(6) {
//     Some(third) => println!("The third element is {}", third),
//     None => println!("There is no third element."),
// }
// }
fn main(){
  let mut v = vec![1, 2, 3, 4, 5];

  let first = &v[0];
  {
    v.push(6);
  }
  println!("The first element is: {}", first);
}