use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();

    if trimmed.len() == 4 && (trimmed.starts_with("0x") || trimmed.starts_with("0X")) {
        let hex_part = &trimmed[2..];
        match u8::from_str_radix(hex_part, 16) {
            Ok(value) => {
                let character = value as char;
                println!("Character: {}", character);
            }
            Err(_) => {
                println!("Invalid hex input.");
            }
        }
    } else {
        println!("Invalid format. Use 0xXX.");
    }
}
