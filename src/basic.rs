use std::io;
use std::io::Write;

pub fn basic_fn(){
    println!("Hello and welcome to Rust basic stuff..");
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();

    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name)
               .expect("Failed to read from line");
    user_name.pop(); // removing the newline from the end

    print!("Hello {} please enter your age:",user_name);
    io::stdout().flush().unwrap();

    let mut user_age_str = String::new();
    io::stdin().read_line(&mut user_age_str)
        .expect("Failed to read age");
    let user_age: i32 = user_age_str.trim().parse()
        .expect("Entered age is not a number");
    println!("Hello {} you are {} years old",user_name,user_age);

    let voting_status = match user_age{
        1..=17 => "you cant vote lmao",
        18     => "you've just qualified for voting, nice",
        19..   => "you're responsible for this country, but you cant do much, it's ok",
        _      => "what or earth?? how? why? I know you've given negative age lol",
    };
    println!("{}",voting_status);
}

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

pub fn string_fn(){
    println!("things related to strings");
}
