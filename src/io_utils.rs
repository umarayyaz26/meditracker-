use std::io::{self, Write};

pub fn read_line(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_line(prompt);
        match input.parse() {
            Ok(n) => return n,
            Err(_) => println!("Invalid number. Please enter a valid positive integer."),
        }
    }
}
