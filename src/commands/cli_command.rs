use std::{collections::HashMap, process::Command as ProcessCommand};

use serde::Deserialize;

use crate::{
    commands::Command,
    functions::{Function, Functions},
    project::{Project, ProjectElementEnvironment, ProjectElementMetadata},
};

trait CLICommand: ProjectElementMetadata + ProjectElementEnvironment {
    fn get_uses(&self) -> Vec<String> {
        Vec::default()
    }
}

impl<T: CLICommand> Command for T {
    fn run(&self, command_name: &str, project: &Project, args: &[String]) -> bool {
        let command = self.get_element();
        let uses = self.get_uses();
        let command_env_vars = self.get_env_vars();
        let command_functions = project.get_functions(&uses);

        // Merging all env vars: command env vars will override the other env vars
        let all_env_vars: HashMap<_, _> = project
            .env
            .iter()
            .flatten()
            .map(|(key, value)| (key.clone(), value.clone()))
            .chain(
                command_functions
                    .iter()
                    .flat_map(|(_name, function)| function.get_env_vars()),
            )
            .chain(command_env_vars)
            .collect();

        // Taking all functions and build them
        let before_script = command_functions
            .iter()
            .map(|(name, function)| function.build(name))
            .collect::<Vec<_>>()
            .join("\n");

        let script = format!("{before_script}\n{command}");

        let status = ProcessCommand::new("sh")
            .arg("-c")
            .arg(script)
            .arg(env!("CARGO_BIN_NAME")) // becomes $0 inside the script
            .args(args) // becomes $1, $2
            .envs(all_env_vars)
            .env("ORCH_COMMAND", command_name)
            .status()
            .expect("Failed to execute");

        return status.success();
    }
}

#[derive(Debug, Deserialize)]
pub struct SingleCLICommand(String);

impl CLICommand for SingleCLICommand {}

impl ProjectElementMetadata for SingleCLICommand {
    fn get_element(&self) -> &str {
        self.0.as_str()
    }
}

impl ProjectElementEnvironment for SingleCLICommand {}

#[derive(Debug, Deserialize)]
pub struct DetailedSingleCLICommand {
    pub run: String,
    pub description: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub uses: Option<Vec<String>>,
}

impl CLICommand for DetailedSingleCLICommand {
    fn get_uses(&self) -> Vec<String> {
        self.uses.clone().unwrap_or_default()
    }
}

impl ProjectElementMetadata for DetailedSingleCLICommand {
    fn get_element(&self) -> &str {
        self.run.as_str()
    }

    fn get_description(&self) -> &str {
        self.description.as_deref().unwrap_or_default()
    }
}

impl ProjectElementEnvironment for DetailedSingleCLICommand {
    fn get_env_vars(&self) -> HashMap<String, String> {
        self.env.clone().unwrap_or_default()
    }
}
