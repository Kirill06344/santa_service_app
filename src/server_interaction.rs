use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct User {
    pub group_name: String,
    pub user_id: i32,
    pub admin_name: String,
}

#[tokio::main]
pub async fn get_command_result(url: String, data: User) -> Result<String, reqwest::Error> {
    let response: String = reqwest::Client::new()
        .post(url)
        .json(&data)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
