use crate::cli::format::{BOLD_FACE, CYAN, YELLOW, RESET};

pub fn show_help() -> String {
    format!(
        "{BOLD_FACE}PebbleDB Help Page{RESET}\n\
        \n\
        {BOLD_FACE}{CYAN}Database Commands:{RESET}\n\
        \x20 CREATE DATABASE {YELLOW}<name>{RESET}       Create a new database\n\
        \x20 DROP DATABASE {YELLOW}<name>{RESET}         Drop an existing database\n\
        \x20 LIST DATABASES               List all databases\n\
        \x20 USE {YELLOW}<name>{RESET}                   Select a database\n\
        \x20 USE NONE                     Deselect current database\n\
        \n\
        {BOLD_FACE}{CYAN}Table Commands:{RESET}\n\
        \x20 CREATE TABLE {YELLOW}<name>{RESET}          Create a new table\n\
        \x20 DROP TABLE {YELLOW}<name>{RESET}            Drop an existing table\n\
        \x20 LIST TABLES                  List tables in the current database\n\
        \x20 DESCRIBE TABLE {YELLOW}<name>{RESET}        Show table information\n\
        \n\
        {BOLD_FACE}{CYAN}Record Commands:{RESET}\n\
        \x20 ADD TO {YELLOW}<table>{RESET}               Add a new record\n\
        \x20 SELECT {YELLOW}<table> <id>{RESET}          Select records from a table\n\
        \x20 UPDATE {YELLOW}<table> <id>{RESET}          Update a record\n\
        \x20 DELETE {YELLOW}<table> <id>{RESET}          Delete a record\n\
        \n\
        {BOLD_FACE}{CYAN}Server Commands:{RESET}\n\
        \x20 SERVER START                 Start the HTTP server\n\
        \x20 SERVER STOP                  Stop the HTTP server\n\
        \x20 SERVER STATUS                Show server status\n\
        \n\
        {BOLD_FACE}{CYAN}System Commands:{RESET}\n\
        \x20 VERSION                      Show current version\n\
        \x20 HELP                         Show this help page\n\
        \x20 EXIT                         Exit PebbleDB\n\
        "
    )
}