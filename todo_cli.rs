use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

const FILE_PATH: &str = "todo.json";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { description: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { id: usize },
    /// Remove a task
    Remove { id: usize },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            let id = tasks.len() + 1;
            tasks.push(Task {
                id,
                description,
                done: false,
            });
            save_tasks(&tasks);
            println!("✅ Task added!");
        }
        Commands::List => {
            for task in &tasks {
                let status = if task.done { "[Done]" } else { "[Pending]" };
                println!("{} - {} {}", task.id, status, task.description);
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_tasks(&tasks);
                println!("✅ Task marked as done!");
            } else {
                println!("❌ Task not found!");
            }
        }
        Commands::Remove { id } => {
            if tasks.iter().any(|t| t.id == id) {
                tasks.retain(|t| t.id != id);
                save_tasks(&tasks);
                println!("🗑️ Task removed!");
            } else {
                println!("❌ Task not found!");
            }
        }
    }
}

fn load_tasks() -> Vec<Task> {
    if let Ok(content) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let content = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open tasks file");
    file.write_all(content.as_bytes()).expect("Failed to write to file");
}
