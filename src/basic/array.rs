use std::io;
use std::io::Write;

pub fn array_fn(){
    println!("Stuff related to arrays and loops in Rust");
    // defining constant sized arrays
    let arr_1:[i32;6] = [1,2,3,4,5,6];
    println!("arr_1: {:?}",arr_1);
    let mut idx: usize = 0;
    println!("printing all the odd values using while loop");
    while idx < arr_1.len(){
        if arr_1[idx] % 2 == 1{
            print!("{},",arr_1[idx]);
        }
        idx += 1;
    }
    io::stdout().flush().unwrap();
    println!("printing all the even values using for loop");
    for val in arr_1.iter(){
       if val % 2 == 0{
           print!("{},",val);
       }
    }
    println!("Vectors are arrays that can vary in size");
    let mut vec_1: Vec<i32> = Vec::new();
    for &val in arr_1.iter(){
        vec_1.push(val);
    }
    println!("It's rather safe to access vector elements using get for example vec_1[7]={:?}",vec_1.get(7));
}
