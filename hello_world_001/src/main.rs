mod print; // No need to give print.rs file name during compilation
mod vars; // No need to give print.rs file name during compilation
mod types; // No need to give print.rs file name during compilation

fn main() {
    print::run();
    vars::run();
    types::run();
}