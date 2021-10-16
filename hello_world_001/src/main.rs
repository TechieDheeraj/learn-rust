mod print; // No need to give print.rs file name during compilation
mod vars; // No need to give print.rs file name during compilation
mod types; // No need to give print.rs file name during compilation
mod strings; // No need to give print.rs file name during compilation
mod tuples; // No need to give print.rs file name during compilation
mod arrays; // No need to give print.rs file name during compilation
mod vectors; // No need to give print.rs file name during compilation
mod conditionals; // No need to give print.rs file name during compilation
mod loops; // No need to give print.rs file name during compilation
mod functions; // No need to give print.rs file name during compilation
mod pointer_ref; // No need to give print.rs file name during compilation
mod structs; // No need to give print.rs file name during compilation

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
}
