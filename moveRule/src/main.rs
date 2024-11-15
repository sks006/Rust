fn main() {
//    let a1: String = String :: from("Hello");
   
//    {
//      let a2 = a1;
//      println!("{}", a1); // This will result in a compilation error: borrow of moved value
//    }
//    println!("{}", a2); // This will work as expected


    let a1 = String::from("Hello World My Name Is SHIHAB");
    let a2 =&a1;
    println!("THIS IS REFERENCE {}", a1); 




}
