// fn main(){

//   struct Data{
//     first_name: String,
//     last_name:String,
//     Status:String,
//     Age: i64,
//     active:bool,

//   };
//   fn input(first_name: String,last_name:String, Status:String) -> Data {
//       Data{
//         first_name:first_name,
//         last_name:last_name,
//         Status: Status,
//         Age: 20,
//         active:true,

//       }
// };
// input(String::from("Muhammad"),String::from("Anas"),String::from("Student"));
// // println!("{}",last_name)
// }




// let mut user1 = data{
  //   first_name: String::from("Muhammad"),
  //   last_name: String::from("Anas"),
  //   Status: String::from("student"),
  //   Age: 20,
  //   active:true,
  // };

  // println!("the user name is {} {}",user1.first_name,user1.last_name);
  // user1.Status = String::from("Worker");
  // println!("the usern is {}",user1.Status);'

fn main(){

  struct Data{
    first_name: String,
    last_name:String,
    Status:String,
    Age: i64,
    active:bool,

  };

  let user1 = Data{
    first_name: String::from("Muhammad"),
    last_name: String::from("Anas"),
    Status: String::from("student"),
    Age: 20,
    active:true,
  };

  let user2 = Data{
    first_name: String::from("ali"),
    last_name: String::from("ansari"),
    Status: String::from("worker"),
    Age: user1.Age,
    active:user1.active,
  };

  let user3 =Data{
    ..user2
  };

  println!("{}",user2.Age)

}