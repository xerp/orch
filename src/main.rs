mod commands;
mod functions;
mod project;

use clap::Parser;
use colored::Colorize;
use commands::{Command, Commands};
use project::Project;
use std::{fs, process::exit};

use crate::functions::Functions;

#[derive(Parser)]
#[command(name = "orch")]
#[command(about = "Command orchestrator")]
struct CliOptions {
    /// Command to execute
    command: Option<String>,

    /// Extra arguments forwarded to the command
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,

    /// List available commands
    #[arg(short, long)]
    list: bool,

    /// Info about project
    #[arg(short, long)]
    project: bool,

    #[arg(short, long)]
    verbose: bool,
}

fn get_project() -> Project {
    let project_yaml = fs::read_to_string(".orch.yaml");

    let Ok(project_yaml_str) = project_yaml else {
        eprintln!("Project yaml not found. Create an \".orch.yaml\" file in the current dir");
        exit(1);
    };

    let project_data: Result<Project, _> = serde_yaml::from_str(&project_yaml_str);

    if let Err(err) = project_data {
        eprintln!("Project data is not valid");
        eprintln!("{err}");
        exit(1);
    };

    return project_data.expect("Already validated");
}

fn main() {
    let cli_options = CliOptions::parse();

    let project = get_project();

    if cli_options.project {
        project.print_project_info();

        exit(0);
    }

    if cli_options.list {
        println!("Available commands:");
        project.print_available_commands();

        println!();
        println!("Available functions:");
        project.print_available_functions();
        exit(0);
    }

    let Some(command_name) = &cli_options.command else {
        println!("Command required");
        exit(1);
    };

    let command = project
        .get_command(command_name)
        .expect("Command not found.");

    if cli_options.verbose {
        println!(
            "{}",
            format!("Running {command_name} command...").green().bold()
        );
        println!();
    }

    let result = command.run(command_name, &project, &cli_options.args);

    match result {
        Ok(Some(0)) => {
            if cli_options.verbose {
                println!();
                println!(
                    "{}",
                    format!("Command {command_name} ran successfully...")
                        .green()
                        .bold()
                )
            }
        }
        Ok(Some(code)) => {
            if cli_options.verbose {
                println!();
                println!(
                    "{}",
                    format!("Command {command_name} failed with exit code {code}")
                        .red()
                        .bold()
                )
            }
        }
        Ok(None) => {
            eprintln!(
                "{}",
                format!("Command {command_name} was terminated by a signal")
                    .red()
                    .bold()
            );
            exit(1);
        }
        Err(error) => {
            eprintln!("Failed to start command {command_name}: {error}");
            exit(1);
        }
    }
}
