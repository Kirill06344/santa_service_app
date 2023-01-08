use serde::{Serialize};

pub enum Method {
    POST,
    GET,
}

#[derive(Serialize, Debug)]
pub struct User {
    pub group_name: String,
    pub user_id: i32,
    pub admin_name: String,
}

#[tokio::main]
pub async fn get_command_result(url: String, data: User, method: Method) -> Result<String, reqwest::Error> {
    let response: String;
    match method {
        Method::POST => {
            response = reqwest::Client::new()
                .post(url)
                .json(&data)
                .send()
                .await?
                .json()
                .await?;
        },
        Method::GET => {
            let data_from_db: Vec<String> = reqwest::Client::new()
                .get(url)
                .send()
                .await?
                .json()
                .await?;
            let mut res: String = String::new();
            for x in data_from_db.iter() {
                res = res + x;
                res = res + "\n";
            }
            response = res;
        }
    }
    Ok(response)
    
}
