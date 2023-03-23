
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        Some(0)
    }
}

pub fn trait_fn(){
    // variable needs to be mutable here because of the function next
    // borrows it as a mutable reference, this does mean that whenver a call to
    // next is made the variable is passed a mutable reference and hence the compiler
    // will assume that its making changes even though we're just returning some random
    // value
    let mut c = Counter{};
    println!("example for advanced trait");
    let val = c.next().unwrap_or(0);
    println!("accessing next for counter {}",val);
}
