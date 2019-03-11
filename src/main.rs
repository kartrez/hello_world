extern crate library;

use library::mymod::{*};

fn main() {
    println!("----------------START------------------\n");
    cortaje::main();
    println!("---------------------------------------\n");
    format::main();
    println!("---------------------------------------\n");
    arrays::main();
    println!("---------------------------------------\n");
    structure::main();
    println!("---------------------------------------\n");
    list::main();
    println!("---------------------------------------\n");
    aliases::main();
    println!("---------------------------------------\n");
    enums::main();
    println!("---------------------------------------\n");
    destructure::cortaje::main();
    println!("---------------END---------------------\n");
}