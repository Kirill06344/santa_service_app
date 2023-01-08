use std::{io};
use simple_tables::Table;
use simple_tables::macros::{table_row, table};

#[table_row]
pub struct CommandsRow {
    pub id: u32,
    pub command_name: String,
    pub parameter_1: String,
    pub parameter_2: String,
    pub description: String,
}

#[table(rows = CommandsRow)]
pub struct CommandsTable {}

pub fn get_data(data: &mut String) -> Result<usize, &str> {
    let current_length = io::stdin().read_line(data).expect("Error: Input is invalid");
    if current_length == 0 {
        return Err("Programm was shutting down.");
    }
    Ok(current_length)
}

pub fn check_command(command: &str, length_of_command: &usize) -> bool {
        *length_of_command > 3 
        ||  (*length_of_command == 2 && command == "assign") 
        || (*length_of_command == 3 && command != "assign")
        || (*length_of_command == 1 && (command != "clear" && command != "help"))
        || (*length_of_command != 1 && (command == "clear" || command == "help"))
}

pub fn print_help() {
    let rows: Vec<CommandsRow> = vec![
        CommandsRow{ id: 1, command_name: "create".to_string(), parameter_1: "<group name>".to_string(), parameter_2: "      -     ".to_string(), description: "Every user can create a team and become superuser".to_string()},
        CommandsRow{ id: 2, command_name: "delete".to_string(), parameter_1: "<group name>".to_string(), parameter_2: "      -     ".to_string(), description: "Superuser can delete group".to_string()},
        CommandsRow{ id: 3, command_name: "join".to_string(), parameter_1: "<group name>".to_string(), parameter_2: "      -     ".to_string(), description: "Every user can join a group".to_string()},
        CommandsRow{ id: 4, command_name: "leave".to_string(), parameter_1: "<group name>".to_string(), parameter_2: "      -     ".to_string(), description: "User can leave the group".to_string()},
        CommandsRow{ id: 5, command_name: "assign".to_string(), parameter_1: "<group name>".to_string(), parameter_2: "<admin name>".to_string(), description: "Superuser can appoint another user as superuser".to_string()},
        CommandsRow{ id: 6, command_name: "deleteAdmin".to_string(), parameter_1: "<group name>".to_string(), parameter_2: "      -     ".to_string(), description: "Superuser can leave the group if amount of superusers is one or more".to_string()},
        CommandsRow{ id: 7, command_name: "generateSantas".to_string(), parameter_1: "<group name>".to_string(), parameter_2: "      -     ".to_string(), description: "Superuser can generate Santas for each person in the group".to_string()},
        CommandsRow{ id: 8, command_name: "getUsers".to_string(), parameter_1: "     -     ".to_string(), parameter_2: "      -     ".to_string(), description: "Superuser can generate Santas for each person in the group".to_string()},
        CommandsRow{ id: 9, command_name: "getGroups".to_string(), parameter_1: "     -     ".to_string(), parameter_2: "      -     ".to_string(), description: "Superuser can generate Santas for each person in the group".to_string()},
        CommandsRow{ id: 10, command_name: "clear".to_string(), parameter_1: "     -     ".to_string(), parameter_2: "      -     ".to_string(), description: "Clear the terminal".to_string()},
        CommandsRow{ id: 11, command_name: "help".to_string(), parameter_1: "     -     ".to_string(), parameter_2: "      -     ".to_string(), description: "Print table with commands".to_string()},
    ];
    let table = CommandsTable::from_vec(&rows);
    println!("{}", table.to_string());
}

pub fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn do_extra_console_command(command: &str) -> bool {
    if command == "help" {
        print_help();
        return true;
    } else if command == "clear" {
        clear_terminal();
        return true;
    }
    false
}
