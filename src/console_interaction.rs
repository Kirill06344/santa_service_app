use std::{io};

pub fn get_data(data: &mut String) -> Result<usize, &str> {
let current_length = io::stdin().read_line(data).expect("Error: Input is invalid");
if current_length == 0 {
return Err("Programm was shutting down.");
}
Ok(current_length)
}

pub fn check_command(command: &str, length_of_command: &usize) -> bool {
*length_of_command < 2 || *length_of_command > 3 || (*length_of_command == 2 && command == "assign") || (*length_of_command == 3 && command != "assign")
}
