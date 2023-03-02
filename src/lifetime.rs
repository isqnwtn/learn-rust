pub fn lifetime_fn(){
    println!("Stuff related to reference and lifetime");
    let str1: String = String::from("small");
    let str2: String = String::from("very long");

    let result: &str = longest_str(str1.as_str(), str2.as_str());
    println!("The longest string is {}",result);
}
/* this actually means that the result will have lifetime (a) which will
 * be the smallest of the lifetimes (a) in the arguments
 */
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

/* the following code will fail because of the reason explained above
 * pub fn lifetime_fn(){
    println!("Stuff related to reference and lifetime");
    let str1: String = String::from("small");
    let result: &str;
    {
    let str2: String = String::from("very long");
    let result: &str = longest_str(str1.as_str(), str2.as_str());
    }
    println!("The longest string is {}",result);
}
 */
