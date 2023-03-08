/*
 * Author : Vismay
 * description : README.md
 */
use clap::Parser;
use clap::ArgEnum;

pub mod basic; // this is taken from src/basic.rs
pub mod lifetime;
pub mod smart;
pub mod macro_mod; // this refers to macro_mod/mod.rs as ther's no macro_mod.rs in src/

use crate::basic::basic_fn;
use crate::basic::array_fn;
use crate::basic::string_fn;

use crate::lifetime::lifetime_fn;

use crate::smart::smart_ptr;

use crate::macro_mod::macro_all_fun;



// A collection of resources while learning rust
#[derive(Default,Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    // package name
    #[clap(arg_enum,short,long)]
    example_type: ExampleKind,
    // max depth
    #[clap(short, long, default_value_t=false)]
    interactive: bool, // depth
}

#[derive(ArgEnum, Clone, Debug, Default)]
enum ExampleKind {
    #[default]
    BasicExamples,
    Array,String,Functions,
    Lifetime,
    SmartPtr,
}

fn main() {
    let args = Arguments::parse();
    //println!("{:?}", args);
    match args.example_type{
        ExampleKind::BasicExamples => basic_fn(),
        ExampleKind::Array => array_fn(),
        ExampleKind::String => string_fn(),
        ExampleKind::Functions => println!("not implemented yet"),
        ExampleKind::Lifetime => lifetime_fn(),
        ExampleKind::SmartPtr => smart_ptr(),
    };
}
