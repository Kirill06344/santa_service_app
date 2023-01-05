use std::{io};

pub fn get_data(data: &mut String) -> Result<usize, &str> {
    let current_length = io::stdin().read_line(data).expect("Error: Input is invalid");
    if current_length == 0 {
        return Err("Programm was shutting down.");
    }
    Ok(current_length)
}