use image::{GenericImageView, ImageBuffer, RgbaImage};
use std::io::{self, Write};

/// Embeds a secret message into the image using LSB steganography.
fn hide_message(image_path: &str, output_path: &str, message: &str) -> Result<(), String> {
    let img = image::open(image_path).map_err(|e| e.to_string())?;
    let (width, height) = img.dimensions();

    let mut img_buffer: RgbaImage = img.to_rgba8();
    let message_bytes = message.as_bytes();
    let total_bytes = (width * height * 3) as usize;

    if message_bytes.len() * 8 + 32 > total_bytes {
        return Err("Message is too large to fit in the image.".to_string());
    }

    // Encode message length in the first 32 bits.
    let message_length = message_bytes.len() as u32;
    for i in 0..32 {
        let bit = (message_length >> (31 - i)) & 1;
        let pixel_index = i / 3;
        let channel_index = i % 3;

        let x = (pixel_index as u32 % width) as u32;
        let y = (pixel_index as u32 / width) as u32;

        let pixel = img_buffer.get_pixel_mut(x, y);
        let color_channel = &mut pixel[channel_index as usize];  // Explicitly cast to usize

        *color_channel = (*color_channel & 0xFE) | (bit as u8);
    }

    // Encode the message in the least significant bits of the pixels.
    for (i, &byte) in message_bytes.iter().enumerate() {
        for bit_index in 0..8 {
            let bit = (byte >> (7 - bit_index)) & 1;
            let pixel_index = (32 + i * 8 + bit_index) / 3;
            let channel_index = (32 + i * 8 + bit_index) % 3;

            let x = (pixel_index as u32 % width) as u32;
            let y = (pixel_index as u32 / width) as u32;

            let pixel = img_buffer.get_pixel_mut(x, y);
            let color_channel = &mut pixel[channel_index as usize];  // Explicitly cast to usize

            *color_channel = (*color_channel & 0xFE) | (bit as u8);
        }
    }

    img_buffer.save(output_path).map_err(|e| e.to_string())?;
    Ok(())
}

/// Retrieves a secret message from the image using LSB steganography.
fn retrieve_message(image_path: &str) -> Result<String, String> {
    let img = image::open(image_path).map_err(|e| e.to_string())?;
    let (width, height) = img.dimensions();

    let img_buffer = img.to_rgba8();

    // Decode message length from the first 32 bits.
    let mut message_length = 0u32;
    for i in 0..32 {
        let pixel_index = i / 3;
        let channel_index = i % 3;

        let x = (pixel_index as u32 % width) as u32;
        let y = (pixel_index as u32 / width) as u32;

        let pixel = img_buffer.get_pixel(x, y);
        let color_channel = pixel[channel_index as usize];  // Explicitly cast to usize

        message_length = (message_length << 1) | (color_channel & 1) as u32;
    }

    // Decode the message from the pixels.
    let mut message_bytes = Vec::new();
    for i in 0..message_length {
        let mut byte = 0u8;
        for bit_index in 0..8 {
            let pixel_index = (32 + i * 8 + bit_index) / 3;
            let channel_index = (32 + i * 8 + bit_index) % 3;

            let x = (pixel_index as u32 % width) as u32;
            let y = (pixel_index as u32 / width) as u32;

            let pixel = img_buffer.get_pixel(x, y);
            let color_channel = pixel[channel_index as usize];  // Explicitly cast to usize

            byte = (byte << 1) | (color_channel & 1);
        }
        message_bytes.push(byte);
    }

    String::from_utf8(message_bytes).map_err(|e| e.to_string())
}

fn main() {
    let mut input_image_path = String::new();
    let mut output_image_path = String::new();
    let mut secret_message = String::new();

    print!("Enter the path of the input image: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_image_path).unwrap();
    let input_image_path = input_image_path.trim();

    print!("Enter the path for the output image: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut output_image_path).unwrap();
    let output_image_path = output_image_path.trim();

    print!("Enter the secret message to hide: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut secret_message).unwrap();
    let secret_message = secret_message.trim();

    match hide_message(input_image_path, output_image_path, secret_message) {
        Ok(_) => println!("Message hidden successfully in {}", output_image_path),
        Err(e) => {
            println!("An error occurred while hiding the message: {}", e);
            return;
        }
    }

    println!("Retrieving the hidden message from the output image...");
    match retrieve_message(output_image_path) {
        Ok(message) => println!("Extracted Message: {}", message),
        Err(e) => println!("An error occurred while retrieving the message: {}", e),
    }
}
