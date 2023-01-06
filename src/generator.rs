use std::collections::HashMap;

pub fn get_urls_container() -> HashMap<String, String> {
    let mut urls:HashMap<String, String> = HashMap::new();
    urls.insert("create".to_string(), "http://127.0.0.1:8080/users/add_group".to_string());
    urls.insert("delete".to_string(), "http://127.0.0.1:8080/users/deleteGroup".to_string());
    urls.insert("join".to_string(), "http://127.0.0.1:8080/users/joinGroup".to_string());
    urls.insert("leave".to_string(), "http://127.0.0.1:8080/users/leaveGroup".to_string());
    urls.insert("assign".to_string(), "http://127.0.0.1:8080/users/assignAdmin".to_string());
    urls.insert("deleteAdmin".to_string(), "http://127.0.0.1:8080/users/deleteAdmin".to_string());
    urls.insert("generateSantas".to_string(), "http://127.0.0.1:8080/users/generateSanta".to_string());
    urls
}