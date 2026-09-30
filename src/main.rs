mod commands;
mod project;

use clap::Parser;
use colored::Colorize;
use commands::{Command, Commands};
use project::Project;
use std::{fs, process::exit};

#[derive(Parser)]
#[command(name = "orch")]
#[command(about = "Command orchestrator")]
struct CliOptions {
    // Command to execute
    command: Option<String>,

    // List available commands
    #[arg(short, long)]
    list: bool,

    // Info about project
    #[arg(short, long)]
    project: bool,
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
        eprintln!("{}", err);
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
        project.commands.print_available_commands();
        exit(0);
    }

    let command_name = &cli_options.command.expect("Error: command required");

    let command = project
        .commands
        .get_command(command_name)
        .expect("Command not found.");

    println!(
        "{}",
        format!("Running {} command...", command_name)
            .green()
            .bold()
    );

    let status = command.run().wait().expect("Failed to wait");

    if status.success() {
        println!(
            "{}",
            format!("Command {} ran successfully...", command_name)
                .green()
                .bold()
        )
    };
}
