use crate::parser::grammar::{Target, TargetForm};
use crate::types::command::{Command, DatabaseCommand, SystemCommand, TableCommand};
use crate::types::error::Error;

/// # Parse
/// The main function used by the program. It maps the actions requested by the input
/// to a respective helper function
pub fn parse(input: &str) -> Result<Command, Error> {
    let lowercase = input.to_lowercase();
    let tokens: Vec<&str> = lowercase.split_whitespace().collect();

    let Some(&command) = tokens.get(0) else {
        return Err(Error::NoInput)
    };

    match command {
        // common database and table command
        "list" => list_command(&tokens),
        "create" => create_command(&tokens),
        "drop" => drop_command(&tokens),

        // database specific command
        "use" => use_command(&tokens),

        // table specific command
        "describe" => describe_command(&tokens),

        // record specific command TODO: work on this (7/14/2026)
        // "add" => add_command(),
        // "select" => select_command(),
        // "update" => update_command(),
        // "delete" => delete_command(),

        // system specific command
        "exit" => exit_command(&tokens),
        "help" => help_command(&tokens),

        _ => Err(Error::UnknownCommand(command.to_string())),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # COMMON DATABASE AND TABLE SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # List Command
fn list_command(tokens: &[&str]) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 2)?;

    // check if target token exists
    // define the target afterward
    let target = validate_target(tokens.get(1), TargetForm::Plural)?;

    // return an Ok statement
    // either Database or Table target with the List command
    match target {
        Target::Database => Ok(Command::Database(DatabaseCommand::List)),
        Target::Table => Ok(Command::Table(TableCommand::List)),
    }
}

/// # Create Command
fn create_command(tokens: &[&str]) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 3)?;

    // check if target token exists
    // define the target afterward
    let target = validate_target(tokens.get(1), TargetForm::Singular)?;

    // check if name token exists
    let Some(&name) = tokens.get(2) else {
        return Err(Error::MissingTargetName(target));
    };

    // return an Ok statement
    // either Database or Table target with the Create command
    match target {
        Target::Database => Ok(Command::Database(DatabaseCommand::Create(name.to_string()))),
        Target::Table => Ok(Command::Table(TableCommand::Create(name.to_string()))),
    }
}

/// # Drop Command
fn drop_command(tokens: &[&str]) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 3)?;

    // check if target token exists
    // map the target type afterward
    let target: Target = validate_target(tokens.get(1), TargetForm::Singular)?;

    // check if name token exists
    let Some(&name) = tokens.get(2) else {
        return Err(Error::MissingTargetName(target));
    };

    // return an Ok statement
    // either Database or Table target with the Drop command
    match target {
        Target::Database => Ok(Command::Database(DatabaseCommand::Drop(name.to_string()))),
        Target::Table => Ok(Command::Table(TableCommand::Drop(name.to_string()))),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # DATABASE SPECIFIC SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # Use Command
fn use_command(tokens: &[&str]) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 2)?;

    // check if name token exists
    let Some(&name) = tokens.get(1) else {
        return Err(Error::NoInput)
    };

    // return an Ok statement of Database target with the Use command
    Ok(Command::Database(DatabaseCommand::Use(name.to_string())))
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # TABLE SPECIFIC SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # Describe Command
fn describe_command(tokens: &[&str]) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 2)?;

    // check if name token exists
    let Some(&name) = tokens.get(1) else {
        return Err(Error::NoInput)
    };

    // return an Ok statement of Table target with the Describe command
    Ok(Command::Table(TableCommand::Describe(name.to_string())))
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # RECORD SPECIFIC SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # Add Command
fn add_command() {
    // TODO: work on this (7/14/2026)
}

/// # Select Command
fn select_command() {
    // TODO: work on this (7/14/2026)
}

/// # Update Command
fn update_command() {
    // TODO: work on this (7/14/2026)
}

/// # Delete Command
fn delete_command() {
    // TODO: work on this (7/14/2026)
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # SYSTEM SPECIFIC SECTION
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # Help Command
fn help_command(tokens: &[&str]) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 1)?;

    Ok(Command::System(SystemCommand::Help))
}

/// # Exit Command
fn exit_command(tokens: &[&str]) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 1)?;

    Ok(Command::System(SystemCommand::Exit))
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// # HELPER FUNCTIONS
///////////////////////////////////////////////////////////////////////////////////////////////////

/// # Validate Token Count
fn validate_token_count(tokens: &[&str], expected_len: usize) -> Result<(), Error> {
    if tokens.len() > expected_len  {
        return Err(Error::TooManyArguments(
            format!("received {} instead of {}", tokens.len(), expected_len))
        );
    }
    Ok(())
}

/// # Validate Target
/// Helper function to return the correct target from the inputs
fn validate_target(target_key: Option<&&str>, target_form: TargetForm) -> Result<Target, Error> {
    let Some(&target_keyword) = target_key else {
        return Err(Error::MissingTarget(target_form))
    };

    match (target_form, target_keyword) {
        (TargetForm::Singular, "database") => Ok(Target::Database),
        (TargetForm::Singular, "table") => Ok(Target::Table),
        (TargetForm::Plural, "databases") => Ok(Target::Database),
        (TargetForm::Plural, "tables") => Ok(Target::Table),

        _ => Err(Error::UnknownCommand(target_keyword.to_string())),
    }
}