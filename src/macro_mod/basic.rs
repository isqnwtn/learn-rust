// macro_export means that this macro should be used
// whenever the crate is called
#[macro_export]
macro_rules! multiply_add {
   ($a:expr, $b:expr, $c:expr) => {
     $a *( $b + $c )
   };
}

// this is an example with macro that can take n number
// of arguments
#[macro_export]
macro_rules! print_list {
    ( $( $x:expr ),* ) => {
        {
           let mut i:usize = 0;
            $(
               i = i+1;
               println!("var {} = {}",i,$x);
            )*
        }
    };
}

pub fn macro_basic(){
   println!("macro related functions");
   println!("Usage for multiply add {}", multiply_add!(2,1,3));
   print_list!(1,2,5,7,8);
}
