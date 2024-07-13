use std::{error::Error, process::Command};

use sudo::RunningAs;

pub fn ensure_root() -> Result<(), Box<dyn Error>> {
    if sudo::check() == RunningAs::User {
        sudo::escalate_if_needed()?;
    }
    
    Ok(())
}

pub fn run_command(name: &str, args: Vec<&str>) { 
    let cmd = Command::new(name)
        .args(args)
        .status();

    if let Err(e) = cmd {
        eprintln!("Command {name} failed to execute: {e}. Aborting");
        std::process::exit(1);
    }
}