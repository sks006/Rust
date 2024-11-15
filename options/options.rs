fn find_username(user_id: u32) -> Option<String> {
    // Use pattern matching to return an Option directly
    match user_id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

fn main() {
    let username = find_username(2);

    // Use match to handle the Option result
    match username {
        Some(name) => println!("Username found: {}", name),
        None => println!("Username not found"),
    }
}
