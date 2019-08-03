fn main(){
        let mut x= String::from("*"); 
        let mut y=1;
        while y != 7 {
                println!("{}",x);
                y=y+1;
                x.push_str("*");
        }
}
