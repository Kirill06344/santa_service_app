use std::collections::HashMap;

pub fn get_urls_container() -> HashMap<String, String> {
    let mut urls:HashMap<String, String> = HashMap::new();
    urls.insert("create".to_string(), "http://127.0.0.1:8080/users/add_group".to_string());
    urls.insert("delete".to_string(), "http://127.0.0.1:8080/users/delete_group".to_string());
    urls.insert("join".to_string(), "http://127.0.0.1:8080/users/join_group".to_string());
    urls.insert("leave".to_string(), "http://127.0.0.1:8080/users/leave_group".to_string());
    urls.insert("assign".to_string(), "http://127.0.0.1:8080/users/make_admin".to_string());
    urls.insert("deleteAdmin".to_string(), "http://127.0.0.1:8080/users/resign".to_string());
    urls.insert("generateSantas".to_string(), "http://127.0.0.1:8080/users/start_santa".to_string());
    urls
}
