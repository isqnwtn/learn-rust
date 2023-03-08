pub mod basic; // we can refer to this like this because mod.rs and basic.rs are inside the same file

// calling the sub module from here
// here we still have to use the entire path crate::basic wont work as crate is reffering to from the root
use crate::macro_mod::basic::macro_basic;

// exporting only this from crate::macro_mod because the rest are just auxiliary functions
pub fn macro_all_fun(){
    // using the functions form submodule here
    macro_basic();
}
