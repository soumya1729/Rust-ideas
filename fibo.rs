use std::env;

fn generate_fibonacci(n: u32) -> Vec<u32> {
    let mut fib_sequence = vec![0, 1]; // Starting values for Fibonacci sequence
    for i in 2..n as usize {
        let next_term = fib_sequence[i - 1] + fib_sequence[i - 2];
        fib_sequence.push(next_term);
    }
    fib_sequence
}

fn main() {
    // Get the number of terms from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: fibonacci <n>");
        std::process::exit(1);
    }

    let n: u32 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Please provide a valid number");
        std::process::exit(1);
    });

    let fibonacci_sequence = generate_fibonacci(n);

    // Print the Fibonacci sequence
    println!("Fibonacci sequence up to {} terms:", n);
    for num in fibonacci_sequence {
        print!("{} ", num);
    }
    println!();
}
