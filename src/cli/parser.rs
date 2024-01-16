use crate::utils::system_commands::{run_command, ensure_root};
use crate::logger::logger::log_stdout;
use crate::logger::log_level::LogLevel;

use super::arguments::create_arguments;

pub fn parse() {
    let command_matches = create_arguments().get_matches();

    if command_matches.get_flag("version") {
        println!("lymerge 0.6.2");
        run_command("emerge", vec!["--version"]);

        println!(r"
                 ♡ ∩ ∩
                 („•֊•„)♡             ♡
                |￣U U￣￣￣￣￣￣￣￣|
                |  Have a good day..  |
                |                     |
                 ￣￣￣￣￣￣￣￣￣￣♡
        ");

        std::process::exit(0);
    }

    match command_matches.subcommand() {
        Some(("install", sub_matches)) => {
            ensure_root();

            let mut emerge_arguments = vec![
                "--ask", 
                "--verbose", 
                "--keep-going",
                "--getbinpkg"
            ];

            let packages =  sub_matches.get_many::<String>("PACKAGES");
            match packages {
                Some(packages) => {
                    for package in packages {
                        emerge_arguments.push(package);
                    }
                },

                None => {
                    log_stdout(LogLevel::Error, "Please make sure to provide the package(s).", true);
                    std::process::exit(1);
                }
            }


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

        Some(("search", ser_matches)) => {
            let mut search_arguments = vec![
                "--search"
            ];

            let packages = ser_matches.get_many::<String>("SEARCH");
            match packages {
                Some(packages) => {
                    for package in packages {
                        search_arguments.push(package);
                    }
                },

                None => {
                    log_stdout(LogLevel::Error, "Please make sure to provide the package(s).", true);
                    std::process::exit(1);
                }
            }

            run_command(
                "emerge",
                search_arguments
            )
        },

        Some(("uninstall", uns_matches)) => {
            ensure_root();

            let mut uninstall_arguments = vec![
                "--ask",
                "--deselect"
            ];

            let packages = uns_matches.get_many::<String>("PACKAGES");
            match packages {
                Some(packages) => {
                    for package in packages {
                        uninstall_arguments.push(package);
                    }
                },
                
                None => {
                    log_stdout(LogLevel::Error, "Please make sure to provide the package(s).", true);
                    std::process::exit(1);
                }
            }  

            // Deselect, then depclean, makes it safer.
            run_command("emerge", uninstall_arguments);
            run_command("emerge", vec!["--ask", "--depclean"]);
        },

        Some(("info", _)) => {
            run_command("emerge", vec!["--info"])
        },

        _ => unreachable!(),
    }
}
