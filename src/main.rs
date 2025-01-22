use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

use clap::Parser;
use log::info;
use serde::Deserialize;

/// Simple program to greet a person
#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub command: String,
    pub dependencies: Option<Vec<String>>, // Optional list of dependencies
}

#[derive(Debug, Deserialize)]
pub struct CustomTaskConfig {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tasks: Option<Vec<TaskConfig>>,
    pub custom_tasks: Option<Vec<CustomTaskConfig>>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.config {
        Some(config) => match load_config(config) {
            Ok(config) => {
                info!("Loaded configuration file");
                info!("Has {:?} tasks", config.tasks.as_ref().unwrap().len());
                info!(
                    "Has {:?} custom_tasks",
                    config.custom_tasks.as_ref().unwrap().len()
                );
                match config.tasks {
                    Some(tasks) => {
                        for task in tasks {
                            println!("-------");
                            println!("Executing task: {}", task.name);
                            match execute_command(&task.command) {
                                Ok(output) => {
                                    if output.status.success() {
                                        println!(
                                            "Task '{}' completed successfully:\n{}",
                                            task.name,
                                            String::from_utf8_lossy(&output.stdout)
                                        );
                                    } else {
                                        eprintln!(
                                            "Task '{}' failed:\n{}",
                                            task.name,
                                            String::from_utf8_lossy(&output.stderr)
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to execute task '{}': {}", task.name, e)
                                }
                            }
                            match task.dependencies {
                                Some(dependencies) => {
                                    println!("Dependencies: {:?}", dependencies);
                                }
                                None => {
                                    println!("No dependencies");
                                }
                            }
                        }
                    }
                    None => {
                        println!("No tasks");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error loading config: {}", e);
            }
        },
        None => {
            eprintln!("No config file specified");
            return;
        }
    }
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn execute_command(command: &str) -> Result<Output, Box<dyn std::error::Error>> {
    // Split the command into program and arguments
    let mut parts = command.split_whitespace();
    let program = parts.next().ok_or("Command cannot be empty")?;
    let args: Vec<&str> = parts.collect();

    // Execute the command
    let output = Command::new(program).args(&args).output()?; // Capture the output
    Ok(output)
}
