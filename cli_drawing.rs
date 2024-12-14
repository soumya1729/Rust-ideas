use std::io;

// Function to draw a rectangle in the command line
fn draw_rectangle(width: u32, height: u32) {
    for _ in 0..height {
        println!("{}", "*".repeat(width as usize));
    }
}

// Function to draw a triangle in the command line
fn draw_triangle(height: u32) {
    for i in 1..=height {
        println!("{}", "*".repeat(i as usize));
    }
}

// Function to draw a circle-like pattern (approximated)
fn draw_circle(radius: u32) {
    let r_squared = (radius * radius) as f64;
    for y in -radius as i32..=radius as i32 {
        for x in -radius as i32..=radius as i32 {
            let distance = (x * x + y * y) as f64;
            if distance <= r_squared {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    println!("Welcome to the Command-Line Drawing Tool!");

    loop {
        println!("\nChoose a shape to draw:");
        println!("1. Rectangle");
        println!("2. Triangle");
        println!("3. Circle");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 4.");
                continue;
            }
        };

        match choice {
            1 => {
                let mut width = String::new();
                let mut height = String::new();

                println!("Enter the width of the rectangle:");
                io::stdin().read_line(&mut width).expect("Failed to read input");
                let width: u32 = width.trim().parse().expect("Please enter a valid number");

                println!("Enter the height of the rectangle:");
                io::stdin().read_line(&mut height).expect("Failed to read input");
                let height: u32 = height.trim().parse().expect("Please enter a valid number");

                draw_rectangle(width, height);
            }
            2 => {
                let mut height = String::new();

                println!("Enter the height of the triangle:");
                io::stdin().read_line(&mut height).expect("Failed to read input");
                let height: u32 = height.trim().parse().expect("Please enter a valid number");

                draw_triangle(height);
            }
            3 => {
                let mut radius = String::new();

                println!("Enter the radius of the circle:");
                io::stdin().read_line(&mut radius).expect("Failed to read input");
                let radius: u32 = radius.trim().parse().expect("Please enter a valid number");

                draw_circle(radius);
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 4."),
        }
    }
}
