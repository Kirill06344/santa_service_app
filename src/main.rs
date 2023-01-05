use std::{io};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Commands {
    CreateGroup(String), // group name
    DeleteGroup(String), // group name
    JoinGroup(String), // group name
    LeaveGroup(String), // group name
    AssignAdmin(String), // user name
    DeleteAdmin,
    GenerateSantas(String), // group name
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    loop {
        let mut user_login = String::new();
        let mut command_args = String::new();
        println!("Enter your login:");
        let mut current_length = io::stdin().read_line(&mut user_login).expect("Error: Input is invalid");
        if current_length == 0 {
            break;
        }
        println!("Enter your command:");
        current_length = io::stdin().read_line(&mut command_args).expect("Error: Input is invalid");
        if current_length == 0 {
            break;
        }
        let command_args: Vec<_> = command_args.split(" ").collect();
        let command_name: &str = command_args[0];
        let mut command: Option<Commands> = None;
        let mut argument_option: Option<String> = None;
        if (command_args.len() == 1 && command_name != "deleteAdmin") || command_args.len() > 2 {
            continue;
        }
        if command_args.len() == 2 {
            argument_option.replace(command_args[1].to_string());
        }
        let argument: String = argument_option.unwrap();
        match command_name {
            "create" => command.replace(Commands::CreateGroup(argument)),
            "delete" => command.replace(Commands::DeleteGroup(argument)),
            "join" => command.replace(Commands::JoinGroup(argument)),
            "leave" => command.replace(Commands::LeaveGroup(argument)),
            "assign" => command.replace(Commands::AssignAdmin(argument)),
            "deleteAdmin" => command.replace(Commands::DeleteAdmin),
            "generateSantas" => command.replace(Commands::GenerateSantas(argument)),
            _ => continue,
        };
        let command: Commands = command.unwrap();
        let user = User {
            login: user_login,
            command,
        };
        let response = reqwest::Client::new()
            .post("127.0.0.1:8080")
            .json(&user)
            .send()
            .await?
            .json()
            .await?;
        println!("{:#?}", response); // response from server
    }
    Ok(())
}
