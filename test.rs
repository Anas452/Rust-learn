// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     let mut i =0;
//     while i<list.len() {
//         if list[i] > largest {
//             largest = list[i];
//         };
//         i= i+1;
//     };

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }
use std::io;
#[derive(Debug)]
fn main() {
    

struct User {
    Username: String,
    Email: String,
    Address: String,
    Password: String,
};
// let mut Username = String::new();
// io::stdin().read_line(&mut Username);
  let mut username = String::new();
  io::stdin().read_line(&mut username);

  let mut email = String::new();
  io::stdin().read_line(&mut email);

  let mut address = String::new();
  io::stdin().read_line(&mut address);

  let mut password = String::new();
  io::stdin().read_line(&mut password);

  let user1= User{
    Username:username,
    Email:email,
    Address:address,
    Password:password,

  };
  println!("{:?}",user1);

fn user_input(username:String,email:String,address:String,password:String)->User{
  User{
    Username:username,
    Email:email,
    Address:address,
    Password:password,
  }
}
  

}







