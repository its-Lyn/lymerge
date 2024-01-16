use clap::{
    Command, 
    arg, Arg, ArgAction
};

pub fn create_arguments() -> Command {
    Command::new("lymerge")
        .about("A small and easy wrapper around emerge.")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Download and install packages.")
                .arg(
                    arg!(<PACKAGE> "The package to install.").num_args(0..) 
                )
                .arg(
                    Arg::new("nobin")
                        .short('n')
                        .long("nobin")
                        .help("Tell emerge not to pull from your binhost.")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("upgrade")
                .about("Upgrade your system.")
                .arg(
                    Arg::new("nosync")
                        .short('s')
                        .long("nosync")
                        .help("Tell emerge not to sync your repositories before updating.")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("nobin")
                        .short('n')
                        .long("nobin")
                        .help("Tell emerge not to pull from your binhost.")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("search")
                .about("Search for packages")
                .arg(
                    arg!(<SEARCH> "The packages to search for.").num_args(0..)
                )
        )
        .subcommand(
            Command::new("info")
                .about("Show portage info.")
        )
        .subcommand(
            Command::new("uninstall")
                .about("Remove packages from your system.")
                .arg(
                    arg!(<PACKAGES> "The packages to uninstall.").num_args(0..)
                )
        )
}
