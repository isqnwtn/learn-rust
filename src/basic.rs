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
}
