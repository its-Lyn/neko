use clap::{arg, Arg, ArgAction, Command};

pub fn make_arguments() -> Command {
    Command::new("neko")
        .about("Simple pacman wrapper.")
        .version("0.1.0")
        .arg_required_else_help(true)
        .disable_version_flag(true)
        // Arguments
        // Eg. neko --version
        .arg(
            Arg::new("version")
                .long("version")
                .short('v')
                .help("Print neko's version.")
                .action(ArgAction::SetTrue)
        ) 
        // Subcommands
        // Eg. neko install <pkg>
        .subcommand(
            Command::new("grab")
                .aliases(vec!["install", "in"])
                .about("Download packages from pacman repos.")
                .arg(
                    arg!(<tuna_cans> "The packages to download.").num_args(0..)
                ) 
        )
        
        .subcommand(
            Command::new("eat")
                .aliases(vec!["uninstall", "rm"])
                .about("Completely remove packages from your computer.")
                .arg(
                    arg!(<tuna_cans> "The packages to uninstall").num_args(0..)
                )
        )

        .subcommand(
            Command::new("explore")
                .aliases(vec!["search, se"])
                .about("Search the repos for packages.")
                .arg(
                    arg!(<tuna_cans> "The packages to search for.").num_args(0..)
                )
        )

        .subcommand(
            Command::new("nap")
                .aliases(vec!["upgrade", "up"])
                .about("Upgrade your system.")
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .help("Force refresh the database, even if it looks up-to-date.")
                        .action(ArgAction::SetTrue)
                )
        )

        .subcommand(
            Command::new("sniff")
                .alias("info")
                .about("Show package information.")
                .arg(
                    arg!(<tuna_cans> "The packages to show information for.").num_args(0..)
                )
                .arg(
                    Arg::new("remote")
                        .long("remote")
                        .short('r')
                        .help("Show information for packages not installed on your system.")
                        .action(ArgAction::SetTrue)
                )
        )
}