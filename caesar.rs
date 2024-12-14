use std::io;

// Function to encrypt a message using Caesar cipher
fn caesar_cipher_encrypt(message: &str, shift: u8) -> String {
    let shift = shift % 26; // Ensure the shift is within the alphabet range
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let a = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                ((c as u8 - a + shift) % 26 + a) as char
            } else {
                c // Non-alphabetic characters remain unchanged
            }
        })
        .collect()
}

// Function to decrypt a message using Caesar cipher
fn caesar_cipher_decrypt(message: &str, shift: u8) -> String {
    caesar_cipher_encrypt(message, 26 - (shift % 26))
}

fn main() {
    println!("Welcome to the Caesar Cipher Program!");

    let mut input = String::new();
    println!("Enter your message:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let message = input.trim();

    let mut shift_input = String::new();
    println!("Enter the shift value (0-25):");
    io::stdin()
        .read_line(&mut shift_input)
        .expect("Failed to read shift value");
    let shift: u8 = shift_input.trim().parse().expect("Please enter a valid number");

    let encrypted_message = caesar_cipher_encrypt(message, shift);
    println!("Encrypted message: {}", encrypted_message);

    let decrypted_message = caesar_cipher_decrypt(&encrypted_message, shift);
    println!("Decrypted message: {}", decrypted_message);
}
