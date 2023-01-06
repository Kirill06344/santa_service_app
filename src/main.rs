pub mod console;
use console::get_data;
pub mod structures;
use structures::{User, PersonalId};
use std::collections::HashMap;

#[tokio::main]
async fn do_post_request(url: &str, data: User) -> Result<(), reqwest::Error> {
    let response: User = reqwest::Client::new()
        .post(url)
        .json(&data)
        .send()
        .await?
        .json()
        .await?;//Todo : do error handling
    
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

    let mut user_login = String::new();
    println!("Enter your login:");
    get_data(&mut user_login).unwrap(); //Todo : do error handling
    // if get_data(&mut user_login).is_err() {
    //     break;
    // }
    let current_login: PersonalId = PersonalId {
        id: None,
        login: user_login.to_string(),
    };
    let current_user: PersonalId = reqwest::Client::new()
        .post("http://127.0.0.1:8080/userByLogin")
        .json(&current_login)
        .send()
        .await?
        .json()
        .await?;//Todo : do error handling
    
    let current_id = current_user.id.unwrap();
    
    loop {
        let mut command_args = String::new();
        println!("Enter your command:");
        if get_data(&mut command_args).is_err() {
            break;
        }
        let command_args: Vec<_> = command_args.split(" ").collect();
        let command: &str = command_args[0];
        let amount_of_args: usize = command_args.len();
        if (amount_of_args != 3 && command == "assign")  || (amount_of_args != 2 && command != "assign") {
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
            Some(url) => do_post_request(url, current_command)?,
            None => continue,
        } 
    }
    Ok(())
}
