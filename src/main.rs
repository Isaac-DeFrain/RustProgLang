#![allow(dead_code)] // disables warnings about unused variables, etc.
#![allow(unused_imports)] // disables warnings about unused imports
#![allow(non_snake_case)] // disables snake case warning

// imports required modules
mod ownership;
mod types_and_variables;
mod control_flow;
mod data_structures;
mod functions;
mod traits;
mod lifetime_and_memory;

const MEANING_OF_LIFE:u8 = 42; // no fixed address
static CONST_VAR:u16 = 666; // fixed address, immutable
static mut MUTABLE_VAR:u8 = 23; // fixed address, mutable (unsafe)

// main function executes when program is run
fn main() {
    //ownership::main();
    //types_and_variables::main();
    //control_flow::main();
    //data_structures::main();
    //functions::main();
    //traits::main();
    lifetime_and_memory::main();
}