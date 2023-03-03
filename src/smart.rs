/* Box is an example for a smart pointer, its used when we dont know
 * the exact size of a type during compile time. for example linkedllist
 * or trees.
 */

use rand::Rng;
use std::fmt;

enum Tree<T>{
    Node(T,Box<Tree<T>>,Box<Tree<T>>),
    Nill,
}

impl<T:fmt::Display> fmt::Display for Tree<T>{
   fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
       match &self{
           Self::Node(i,lnode ,rnode )=>write!(f,"{i}=>({lnode},{rnode})"),
           Self::Nill =>write!(f,""),
       }
   }
}

fn create_int_tree(lv: usize)->Tree<u32>{
   if lv == 0{
       Tree::Nill
   }
    else{
        let mut rng = rand::thread_rng();
        let i:u32 = rng.gen_range(0..10);
        let lnode = Box::new(create_int_tree(lv-1));
        let rnode = Box::new(create_int_tree(lv-1));
        Tree::Node(i,lnode,rnode)
    }
}

pub fn smart_ptr(){
    println!("Everything related to smart pointers..");
    println!("Printing a random tree of depth 3");
    println!("{}",create_int_tree(3));
}
