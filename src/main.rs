pub mod console;
use console::get_data;
pub mod structures;
use structures::{User, PersonalId};
use std::collections::HashMap;
use reqwest::StatusCode;

#[tokio::main]
async fn do_post_request(url: &str, data: User) -> Result<(), reqwest::Error> {
    let response: User = reqwest::Client::new()
        .post(url)
        .json(&data)
        .send()
        .await?
        .json()
        .await?;//Todo : do error handling
    println!("{:?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut urls_container:HashMap<&str, &str> = HashMap::new();
    urls_container.insert("create", "http://127.0.0.1:8080/users/addGroup");
    urls_container.insert("delete", "http://127.0.0.1:8080/users/deleteGroup");
    urls_container.insert("join", "http://127.0.0.1:8080/users/joinGroup");
    urls_container.insert("leave", "http://127.0.0.1:8080/users/leaveGroup");
    urls_container.insert("assign", "http://127.0.0.1:8080/users/assignAdmin");
    urls_container.insert("deleteAdmin", "http://127.0.0.1:8080/users/deleteAdmin");
    urls_container.insert("generateSantas", "http://127.0.0.1:8080/users/generateSanta");


    loop {
        let mut user_login = String::new();
        println!("Enter your login:");
        get_data(&mut user_login).unwrap();
        if get_data(&mut user_login).is_err() {
            break;
        }
        let current_login: PersonalId = PersonalId {
            id: None,
            login: user_login.to_string(),
        };
        let current_user_response = reqwest::Client::new()
            .post("http://127.0.0.1:8080/get_login_id")
            .json(&current_login)
            .send()
            .await?;
        let mut current_id: Option<u32> = None;
        if current_user_response.status() == StatusCode::OK {
            let current_user: PersonalId = current_user_response.json().await?;
            current_id.replace(current_user.id.unwrap());
        }
        else {
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
            let command_args: Vec<_> = command_args.split(" ").collect();
            let command: &str = command_args[0];
            let amount_of_args: usize = command_args.len();
            if (amount_of_args == 3 && command != "assign")  || amount_of_args != 2 {
                continue;
            }
            
            let mut admin_login: Option<String> = None;
            if command == "assign" {
                admin_login = Some(command_args[2].to_string());
            }
            let current_command: User = User {
                id: (current_id),
                group_name: (command_args[1].to_string()),
                admin_login
            };
            match urls_container.get(command) {
                Some(url) => {
                    //1-st bariant
                    let command_response = reqwest::Client::new()
                        .post(*url)
                        .json(&current_command)
                        .send()
                        .await?;
                    
                    if command_response.status() == StatusCode::OK {
                        match command {
                            "assign" => printAssignAdminResult(command_response.text()),
                            "create" => printCreateGroupResult(command_response.text()),
                            "delete" => printDeleteGroupResult(command_response.text()),
                            "join" => printJoinGroupResult(command_response.text()),
                            "leave" => printLeaveGroupResult(command_response.text()),
                            "deleteAdmin" => printDeleteAdminResult(command_response.text()),
                            "generateSantas" => printGenerateSantasResul(command_response.text()),
                        }
                    }
                    else {
                        let error_message: String = current_user_response.json().await?;
                        println!("{}", error_message);
                        continue;
                    }

                    //2-nd bariant
                    // tokio::task::spawn_blocking(|| {
                    //     match do_post_request(*url, current_command) {
                    //         Ok(response) => {
                    //             println!("{:?}", response);
                    //         },
                    //         Err(error) => {
                    //             println!("Error: {:?}", error);
                    //         }
                    //     };
                    // })
                    // .await
                    // .expect("Task panicked")
                },
                None => continue,
            } 
        }

        println!("You have been log out of profile. Would you like to log in to your profile?[Y/N]");
        let mut change_profile: String = String::new();
        if !(get_data(&mut change_profile).is_ok() && (change_profile == "Y" || change_profile == "y")) {
            break;
        }
    }

    
    Ok(())
}
