use crate::parser::command::Action;
use crate::parser::command::Target;
use crate::parser::command::TargetForm;
use crate::parser::error::Error;

/// # Parse
/// The main function used by the program. It maps the actions requested by the input
/// to a respective helper function
pub fn parse(input: &str) -> Result<Action, Error> {
    let lowercase: String = input.to_lowercase();
    let lexicon: Vec<&str> = lowercase.split_whitespace().collect();

    let Some(&action) = lexicon.get(0) else {
        return Err(Error::NoInput)
    };

    match action {
        "list" => parse_list_action(lexicon.get(1)),
        "create" => parse_create_action(lexicon.get(1), lexicon.get(2)),
        "drop" => parse_delete_action(lexicon.get(1), lexicon.get(2)),
        "use" => parse_use_action(lexicon.get(1), lexicon.get(2)),
        "exit" => Ok(Action::Exit),
        _ => Err(Error::UnknownCommand(action.to_string())),
    }
}

/// # Parse List Action
/// lists the names of all databases or tables within the current database.
fn parse_list_action(target_keyword: Option<&&str>) -> Result<Action, Error> {
    let target = check_target(target_keyword, TargetForm::Plural)?;

    // return an Ok result with the List action
    Ok(Action::List(target))
}

/// # Parse Create Action
/// A helper function used to parse the user input using the create commands
fn parse_create_action(target_keyword: Option<&&str>, name: Option<&&str>) -> Result<Action, Error> {
    // check target
    let target = check_target(target_keyword, TargetForm::Singular)?;

    // check if name argument exists
    let Some(&name) = name else {
        return Err(Error::MissingTargetName(target));
    };

    // return an Ok result with the Create action
    Ok(Action::Create(target, name.to_string()))
}

fn parse_delete_action(target_keyword: Option<&&str>, name: Option<&&str>) -> Result<Action, Error> {
    let target = check_target(target_keyword, TargetForm::Singular)?;

    // check if name argument exists
    let Some(&name) = name else {
        return Err(Error::MissingTargetName(target));
    };

    // return an Ok result with the Delete action
    Ok(Action::Delete(target, name.to_string()))
}

fn parse_use_action(target_keyword: Option<&&str>, name: Option<&&str>) -> Result<Action, Error> {
    let target = check_target(target_keyword, TargetForm::Singular)?;

    // check if name argument exists
    let Some(&name) = name else {
        return Err(Error::MissingTargetName(target));
    };

    // return an Ok result with the Use action
    Ok(Action::Use(target, name.to_string()))
}

/// # Check Target
/// Helper function to return the correct target from the inputs
fn check_target(target_keyword: Option<&&str>, target_form: TargetForm) -> Result<Target, Error> {
    let Some(&target_keyword) = target_keyword else {
        return Err(Error::MissingTarget(target_form))
    };

    match target_form {
        TargetForm::Singular => {
            match target_keyword {
                "database" => Ok(Target::Database),
                "table" => Ok(Target::Table),
                _ => Err(Error::UnknownCommand(target_keyword.to_string())),
            }
        }
        TargetForm::Plural => {
            match target_keyword {
                "databases" => Ok(Target::Database),
                "tables" => Ok(Target::Table),
                _ => Err(Error::UnknownCommand(target_keyword.to_string())),
            }
        }
    }
}