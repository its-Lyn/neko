use std::error::Error;

use crate::utils::system;

use super::arguments::make_arguments;

pub fn parse_arguments() -> Result<(), Box<dyn Error>> {
    let arguments = make_arguments().get_matches();

    if arguments.get_flag("version") {
        println!("(    /\\/\\              neko v1.0.0");
        println!(" )  ( ' ')             Copyright (c) 2024 Lyn");
        println!(" ( / -- )");
        println!("  (    /|              neko is a program distributed under the MIT license\n");
        system::run_command("pacman", vec!["--version"]);

        std::process::exit(0);
    }

    match arguments.subcommand() {
        Some(("grab", grab_matches)) => {
            system::ensure_root()?;

            let mut pacman_arguments = vec![
                "--sync"
            ];

            let packages = grab_matches.get_many::<String>("tuna_cans").ok_or("Failed to unpack packages.")?;
            pacman_arguments.extend(packages.map(String::as_str));

            system::run_command("pacman", pacman_arguments);
        },

        Some(("eat", eat_matches)) => {
            system::ensure_root()?;

            let mut pacman_arguments = vec![
                "--remove",
                "--nosave",
                "--recursive"
            ];

            let packages = eat_matches.get_many::<String>("tuna_cans").ok_or("Failed to unpack packages.")?;
            pacman_arguments.extend(packages.map(String::as_str));

            system::run_command("pacman", pacman_arguments);
        },

        Some(("explore", explore_matches)) => {
            let mut pacman_arguments = vec![
                "--sync",
                "--search"
            ];
 
            let packages = explore_matches.get_many::<String>("tuna_cans").ok_or("Failed to unpack packages.")?;
            pacman_arguments.extend(packages.map(String::as_str));

            system::run_command("pacman", pacman_arguments);
        },

        Some(("nap", nap_matches)) => {
            system::ensure_root()?;

            let mut pacman_arguments = vec![
                "--sync",
                "--refresh",
                "--refresh",
                "--sysupgrade"
            ];

            if !nap_matches.get_flag("force") {
                pacman_arguments.remove(1);
            }

            system::run_command("pacman", pacman_arguments);
        },

        Some(("sniff", sniff_matches)) => {
            let mut pacman_arguments = vec![
                "--query",
                "--info"
            ];

            if sniff_matches.get_flag("remote") {
                pacman_arguments[0] = "--sync";
            }

            let packages = sniff_matches.get_many::<String>("tuna_cans").ok_or("Failed to unpack packages.")?;
            pacman_arguments.extend(packages.map(String::as_str));

            system::run_command("pacman", pacman_arguments);
        }

        _ => unreachable!()
    }

    Ok(())
}