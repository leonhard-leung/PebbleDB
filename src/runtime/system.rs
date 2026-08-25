use crate::shared::command::SystemCommand;

/// # Execute
pub fn execute(command: SystemCommand) {
    match command {
        SystemCommand::Help => show_help(),
        SystemCommand::Exit => exit_system(),
    }
}

/// # Show Help
fn show_help() {
    println!("Showing help page...")
}

/// # Exit System
fn exit_system() {
    std::process::exit(0);
}