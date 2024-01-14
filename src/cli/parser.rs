use crate::utils::system_commands::{run_command, ensure_root};
use crate::logger::logger::log_string;
use crate::logger::log_level::LogLevel;

use super::arguments::create_arguments;

pub fn parse() {
    let command_matches = create_arguments().get_matches();

    match command_matches.subcommand() {
        Some(("install", sub_matches)) => {
            ensure_root();

            let mut emerge_arguments = vec![
                "--ask", 
                "--verbose", 
                "--keep-going",
                "--getbinpkg",
                sub_matches.get_one::<String>("PACKAGE").expect(log_string(LogLevel::Error, "Please make sure to provide the package.").as_str())
            ];

            if sub_matches.get_flag("nobin") {
                emerge_arguments.remove(3);
            }

            run_command(
                "emerge", 
                emerge_arguments
            );
        },

        Some(("upgrade", upg_matches)) => {
            ensure_root();

            if !upg_matches.get_flag("nosync") {
                run_command("emerge", vec!["--sync"]);
            }

            let mut upgrade_arguments =  vec!["--ask", "--verbose", "--update", "--deep", "--changed-use", "--getbinpkg", "--keep-going", "@world"];
            if upg_matches.get_flag("nobin") {
                upgrade_arguments.remove(5);
            } 

            run_command(
                "emerge", 
                upgrade_arguments
            )
        },

        _ => unreachable!(),
    }
}
