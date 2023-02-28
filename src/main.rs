/*
 * Author : Vismay
 * description : README.md
 */
use clap::Parser;
use clap::ArgEnum;

// A collection of resources while learning rust
#[derive(Default,Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    // package name
    #[clap(arg_enum,short,long)]
    example_type: ExampleKind,

    // max depth
    #[clap(short, long, default_value_t=true)]
    interactive: bool, // depth
}

#[derive(ArgEnum, Clone, Debug, Default)]
enum ExampleKind {
    // basic examples in rust
    #[default]
    BasicExamples,

    // functions in rust
    Functions,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}
