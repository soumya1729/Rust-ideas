use std::io;
use std::time::Instant;

fn typing_speed_test() {
    let test_text = "The quick brown fox jumps over the lazy dog.";
    println!("Typing Speed Test!");
    println!("Type the following text as quickly and accurately as you can:");
    println!("\n{}\n", test_text);

    let start_time = Instant::now();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let elapsed_time = start_time.elapsed();

    let user_input = user_input.trim();
    if user_input == test_text {
        let words_per_minute = (test_text.split_whitespace().count() as f64 / elapsed_time.as_secs_f64()) * 60.0;
        println!("Great job! You typed the text correctly.");
        println!("Time taken: {:.2} seconds", elapsed_time.as_secs_f64());
        println!("Typing speed: {:.2} words per minute", words_per_minute);
    } else {
        println!("Oops! The text you typed doesn't match exactly.");
        println!("Time taken: {:.2} seconds", elapsed_time.as_secs_f64());
        println!("Accuracy matters too! Try again.");
    }
}

fn main() {
    println!("Welcome to the Typing Speed Test!");
    typing_speed_test();
}
