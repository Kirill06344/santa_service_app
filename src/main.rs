pub mod console_interaction;
use console_interaction::{get_data, check_command, print_help, do_extra_console_command};
pub mod urls_generator;
use urls_generator::get_urls_container;
use std::collections::HashMap;
use reqwest::StatusCode;
pub mod server_interaction;
use server_interaction::{get_command_result, User, Method};
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut instruction_should_be_shown = true;
    let urls_container:HashMap<String, String> = get_urls_container();
    loop {
        let mut user_login = String::new();
        println!("{}", format!("Enter your login:").yellow());
        if get_data(&mut user_login).is_err() {
            break;
        }
        let user_login = user_login.trim();
        let current_user_response = reqwest::Client::new()
            .post("http://127.0.0.1:8080/get_login_id")
            .json(&user_login)
            .send()
            .await?;
        let mut current_id: Option<i32> = None;
        if current_user_response.status() == StatusCode::OK {
            let current_user: i32 = current_user_response.json().await?;
            current_id.replace(current_user);
        } else {
            let error_message: String = current_user_response.json().await?;
            println!("{}", format!("{}", error_message).purple().italic());
            continue;
        }
        let current_id = current_id.unwrap();
        loop {
            if instruction_should_be_shown {
                print_help();
                instruction_should_be_shown = false;
            }
            let mut command_args = String::new();
            println!("{}", format!("Enter your command:").yellow());
            if get_data(&mut command_args).is_err() {
                break;
            }
            let command_args: Vec<_> = command_args.trim().split(" ").collect();
            let command: &str = command_args[0];
            if check_command(command, &command_args.len()) {
                println!("{}", format!("Wrong format of command!").purple().italic());
                continue;
            }
            if do_extra_console_command(command) {
                continue;
            }
            let mut admin_name: String = user_login.to_string();
            if command == "assign" {
                admin_name = command_args[2].to_string();
            }
            let mut group_name: String = "default".to_string();
            if command != "getUsers" && command != "getGroups" {
                group_name = command_args[1].to_string();
            }
            let current_command_data: User = User {
                group_name,
                user_id: (current_id),
                admin_name
            };

            match urls_container.get(command) {
                Some(url) => {
                    let needed_url  = url.clone();
                    let cmd = command.clone().to_string();
                    tokio::task::spawn_blocking(move || {
                        let mut method: Method = Method::POST;
                        if cmd == "getUsers" || cmd == "getGroups" {
                            method = Method::GET;
                        }
                        println!("{}", format!("{}", get_command_result(needed_url, current_command_data, method).unwrap()).purple().italic());
                    }).await.expect("Task panicked")
                },
                None => {
                    println!("{}", format!("Wrong format of command!").purple().italic());
                    continue;
                },
            }
        }
        println!("{}", format!("You have been log out of profile. Would you like to log in to your profile?[Y/N]").yellow());
        let mut change_profile: String = String::new();
        if get_data(&mut change_profile).is_err() || !(change_profile.trim() == "Y" || change_profile.trim() == "y") {
            break;
        }
        instruction_should_be_shown = true;
    }
    Ok(())
}
