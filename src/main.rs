// Lyfetch
// Small and light wrapper over gentoo's emerge

// I am bored.

use utils::system_commands::{check_os, run_command};

mod utils;
mod logger;

fn main() {
   check_os();

   run_command("emerge", vec!["--pretend", "net-im/discord"]);
}
