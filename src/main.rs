pub mod console;
use console::get_data;
pub mod structures;
use structures::{Commands, User};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    loop {
        let mut user_login = String::new();
        let mut command_args = String::new();
        println!("Enter your login:");
        if get_data(&mut user_login).is_err() {
            break;
        }
        println!("Enter your command:");
        if get_data(&mut command_args).is_err() {
            break;
        }
        let command_args: Vec<_> = command_args.split(" ").collect();
        let command_type: &str = command_args[0];
        let mut command: Option<Commands> = None;
        let mut argument_option: Option<String> = None;
        let amount_of_args: usize = command_args.len();
        if (amount_of_args == 1 && command_type != "deleteAdmin") || amount_of_args > 2 {
            continue;
        }
        if amount_of_args == 2 {
            argument_option.replace(command_args[1].to_string());
        }
        let argument: String = argument_option.unwrap();
        match command_type {
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
        print!("{}",serde_json::to_string(&user).unwrap());
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
