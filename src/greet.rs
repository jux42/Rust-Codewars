fn greet(name: &str, owner: &str) -> String {
    let role = if owner == name { "boss" } else { "guest" };
    format!("Hello {role}")
}
