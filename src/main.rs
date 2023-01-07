pub mod console;
use console::get_data;
pub mod structures;
use structures::User;
pub mod generator;
use generator::get_urls_container;
use std::collections::HashMap;
use reqwest::StatusCode;

#[tokio::main]
async fn print_command_result(url: String, data: User) -> Result<String, reqwest::Error> {
    let response: String = reqwest::Client::new()
        .post(url)
        .json(&data)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let urls_container:HashMap<String, String> = get_urls_container();
    loop {
        let mut user_login = String::new();
        println!("Enter your login:");
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
            println!("{}", error_message);
            continue;
        }
        let current_id = current_id.unwrap();
        loop {
            let mut command_args = String::new();
            println!("Enter your command:");
            if get_data(&mut command_args).is_err() {
                break;
            }
            let command_args: Vec<_> = command_args.trim().split(" ").collect();
            let command: &str = command_args[0];
            let amount_of_args: usize = command_args.len();
            if (amount_of_args == 3 && command != "assign")  || amount_of_args != 2 {
                continue;
            }
            let mut admin_name: String = user_login.to_string();
            if command == "assign" {
                admin_name = command_args[2].to_string();
            }
            let current_command: User = User {
                group_name: (command_args[1].to_string()),
                user_id: (current_id),
                admin_name
            };
            match urls_container.get(command) {
                Some(url) => {
                    let needed_url  = url.clone();
                    tokio::task::spawn_blocking(|| {
                        println!("{}", print_command_result(needed_url, current_command).unwrap());
                    }).await.expect("Task panicked")
                },
                None => continue,
            }
        }
        println!("You have been log out of profile. Would you like to log in to your profile?[Y/N]");
        let mut change_profile: String = String::new();
        if get_data(&mut change_profile).is_err() || !(change_profile == "Y" || change_profile == "y") {
            break;
        }
    }
    Ok(())
}
