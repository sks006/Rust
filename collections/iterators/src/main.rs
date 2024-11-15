
// fn main() {
//     let v1= vec![1, 2, 3,5,7,8];
//     let v2 =v1.iter().filter(|x| *x%2==1);
//     let v3 = v2.map(|i| i * 2);
//     println!("{:?}", v3);



// }

 //----------use iterators to filter and map in one step; -------//


// fn main() {
//     let v1 = vec![1, 2, 3, 5, 7, 8];
    
//     let v3: Vec<i32> = v1.iter()
//         .filter(|&&x| x % 2 == 1)  // Filter odd numbers
//         .map(|&i| i * 2)           // Double each value
//         .collect();

//     println!("{:?}", v3);
// }

fn main() {
    let v1 = vec![1, 2, 3, 5, 7, 8];
    let v2: Vec<i32> = v1.iter().filter(|&x| x % 2 == 1).map(|x| x * 2).collect();
    println!("{:?}", v2);
}