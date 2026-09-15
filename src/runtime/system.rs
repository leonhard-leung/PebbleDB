use clap::Error;
use crate::constants::system::VERSION;
use crate::shared::command::SystemCommand;
use crate::shared::output::Output;

/// # Execute
pub fn execute_system(
    command: SystemCommand
) -> Output {
    match command {
        SystemCommand::Help => Output::HelpPage,
        SystemCommand::Exit => Output::ExitApplication,
        SystemCommand::Version => Output::Message(format!("v{}", VERSION)),
    }
}