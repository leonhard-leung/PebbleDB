use crate::parser::command::Action;
use crate::parser::command::Target;
use crate::parser::error::Error;

pub fn parse(input: &str) -> Result<Action, Error> {
    let lowercase: String = input.to_lowercase();
    let lexicon: Vec<&str> = lowercase.split_whitespace().collect();

    let Some(&action) = lexicon.get(0) else {
        return Err(Error::UnknownCommand("unknown command".to_string()));
    };

    match action {
        "create" => parse_create_action(lexicon.get(1), lexicon.get(2)),
        "exit" => Ok(Action::Exit),
        _ => Err(Error::UnknownCommand("unknown command".to_string())),
    }
}

fn parse_create_action(target_keyword: Option<&&str>, name: Option<&&str>) -> Result<Action, Error> {
    // check if target argument exists
    let Some(&target_keyword) = target_keyword else {
        return Err(Error::MissingTarget);
    };

    // check if the target name is correct
    let target = match target_keyword {
        "database" => Target::Database,
        "table" => Target::Table,
        _ => return Err(Error::UnknownCommand(target_keyword.to_string())),
    };

    // check if name argument exists
    let Some(&name) = name else {
        return Err(Error::MissingTargetName(target));
    };

    // return an Ok result with the Create action
    Ok(Action::Create(target, name.to_string()))
}