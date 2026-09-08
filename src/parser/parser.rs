use crate::parser::grammar::{Target, TargetForm};
use crate::shared::command::{Command, DatabaseCommand, RecordCommand, SystemCommand, TableCommand};
use crate::shared::error::Error;

/// # parse
/// Main function used by the program. It maps the actions requested by the input
/// to a respective helper function
pub fn parse(
    input: &str
) -> Result<Command, Error> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let Some(&command) = tokens.get(0) else {
        return Err(Error::NoInput)
    };

    match command.to_lowercase().as_str() {
        // common database and table command
        "list" => list_command(&tokens),
        "create" => create_command(&tokens),
        "drop" => drop_command(&tokens),

        // database-specific command
        "use" => use_command(&tokens),

        // table-specific command
        "describe" => describe_command(&tokens),

        // record-specific command TODO: work on this (7/14/2026)
        "add" => add_command(&tokens),
        "read" => select_command(&tokens),
        "update" => update_command(&tokens),
        // "delete" => delete_command(&tokens),

        // system specific command
        "exit" => exit_command(&tokens),
        "help" => help_command(&tokens),

        _ => Err(Error::UnknownCommand(command.to_string())),
    }
}

// =================================================================================================
// Common Database and Table Commands
// =================================================================================================

/// # List Command
fn list_command(
    tokens: &[&str]
) -> Result<Command, Error> {
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
fn create_command(
    tokens: &[&str]
) -> Result<Command, Error> {
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
fn drop_command(
    tokens: &[&str]
) -> Result<Command, Error> {
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

// =================================================================================================
// Database Specific Commands
// =================================================================================================

/// # Use Command
fn use_command(
    tokens: &[&str]
) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 2)?;

    // check if name token exists
    let Some(&name) = tokens.get(1) else {
        return Err(Error::NoInput)
    };

    // return an Ok statement of Database target with the Use command
    Ok(Command::Database(DatabaseCommand::Use(name.to_string())))
}

// =================================================================================================
// Table Specific Commands
// =================================================================================================

/// # Describe Command
fn describe_command(
    tokens: &[&str]
) -> Result<Command, Error> {
    // check if the number of tokens are correct
    validate_token_count(tokens, 2)?;

    // check if name token exists
    let Some(&name) = tokens.get(1) else {
        return Err(Error::NoInput)
    };

    // return an Ok statement of Table target with the Describe command
    Ok(Command::Table(TableCommand::Describe(name.to_string())))
}

// =================================================================================================
// Record Specific Commands
// =================================================================================================

/// # Add Command
fn add_command(
    tokens: &[&str]
) -> Result<Command, Error> {
    // check if the number of tokens is correct
    validate_token_count(tokens, 3)?;

    // check if keyword token exists
    let Some(&keyword) = tokens.get(1) else {
        return Err(Error::NoInput)
    };

    let Some(&table_name) = tokens.get(2) else {
        return Err(Error::NoInput)
    };

    Ok(Command::Record(RecordCommand::Insert(table_name.to_string())))
}

/// # Select Command
fn select_command(
    tokens: &[&str]
) -> Result<Command, Error> {
    // check if the number of tokens is correct
    validate_token_count(tokens, 3)?;

    // check if keyword token exists
    let Some(&table_name) = tokens.get(1) else {
        return Err(Error::NoInput)
    };

    let Some(&id) = tokens.get(2) else {
        return Err(Error::NoInput)
    };

    Ok(Command::Record(RecordCommand::Select(table_name.to_string(), id.parse::<u32>()?)))
}

/// # Update Command
fn update_command(
    tokens: &[&str]
) -> Result<Command, Error> {
    // check if the number of tokens is correct
    validate_token_count(tokens, 3)?;

    // check if keyword token exists
    let Some(&table_name) = tokens.get(1) else {
        return Err(Error::NoInput)
    };

    let Some(&id) = tokens.get(2) else {
        return Err(Error::NoInput)
    };

    Ok(Command::Record(RecordCommand::Update(table_name.to_string(), id.parse::<u32>()?)))
}

/// # Delete Command
fn delete_command() {
    // TODO: work on this (7/14/2026)
}

// =================================================================================================
// System Specific Commands
// =================================================================================================

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

// =================================================================================================
// Helper Functions
// =================================================================================================

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

    match (target_form, target_keyword.to_lowercase().as_str()) {
        (TargetForm::Singular, "database") => Ok(Target::Database),
        (TargetForm::Singular, "table") => Ok(Target::Table),
        (TargetForm::Plural, "databases") => Ok(Target::Database),
        (TargetForm::Plural, "tables") => Ok(Target::Table),

        _ => Err(Error::UnknownCommand(target_keyword.to_string())),
    }
}