use crate::types::command::RecordCommand;

/// # Execute
pub fn execute(command: RecordCommand) {
    match command {
        RecordCommand::Select => select_record(),
        RecordCommand::Insert => insert_record(),
        RecordCommand::Update => update_record(),
        RecordCommand:: Delete => delete_record(),
    }
}

/// # Select Record
fn select_record() {
    println!("Showing records...");
}

/// # Add Record
fn insert_record() {
    println!("Record inserted...");
}

/// # Update Record
fn update_record() {
    println!("Record updated...");
}

/// # Delete Record
fn delete_record() {
    println!("Record deleted...");
}

