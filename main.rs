fn main(){
        let s= String::from("hello");
        let r1 =s;
        println!("{}",r1);
        let r2  =&s;
        println!("{}",r2);
}