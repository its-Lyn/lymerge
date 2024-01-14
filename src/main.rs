// Lyfetch
// Small and light wrapper over gentoo's emerge

// I am bored.

use utils::system_commands::check_os;

mod utils;
mod logger;
mod cli;

fn main() {
   check_os();

   // What a barren main function
   cli::parser::parse();
}
