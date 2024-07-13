use which::which;

mod cli;
mod utils;

fn main() {
    if let Err(_) = which("pacman") {
        eprintln!("Could not find the pacman binary in your system! Are you sure you're on Arch Linux?");
        std::process::exit(1);
    }

    if let Err(e) = cli::parser::parse_arguments() {
        eprintln!("Failed to run neko: {e}.");
        std::process::exit(1);
    }
}
