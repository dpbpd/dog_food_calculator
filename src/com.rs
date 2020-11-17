use std::io; 
 
pub fn request_response(request: &str) -> String {
    let mut response = String::new();
    println!("{}", request);
    io::stdin().read_line(&mut response)
        .expect("Failed to read line");
    response
}
pub fn get_command(command: String) -> Option<char> {
    let command: char = match command.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("{} is not a char", command.trim());
            return None
        }
    };
    match command {
        'k' => return Some(command),
        'p' => return Some(command),
        's' => return Some(command),
        _ => return None
    }
}