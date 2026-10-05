use std::{collections::HashMap, ops::Deref, process::Command as ProcessCommand};

use serde::Deserialize;

use crate::{
    commands::Command,
    functions::{Function, Functions},
    project::{Project, ProjectElementEnvironment, ProjectElementMetadata},
};

trait CLICommand: ProjectElementMetadata + ProjectElementEnvironment {
    fn get_uses(&self) -> &[String] {
        &[]
    }
}

impl<T: CLICommand> Command for T {
    fn run(&self, command_name: &str, project: &Project, args: &[String]) -> bool {
        let command = self.get_element();
        let uses = self.get_uses();
        let command_env_vars = self.get_env_vars();
        let command_functions = project.get_functions(uses);

        let all_env_vars: HashMap<&String, &String> = project
            .env
            .iter()
            .flatten()
            .chain(
                command_functions
                    .iter()
                    .filter_map(|(_name, function)| function.get_env_vars())
                    .flatten(),
            )
            .chain(command_env_vars.into_iter().flatten())
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
        self.0.deref()
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
    fn get_uses(&self) -> &[String] {
        self.uses.as_deref().unwrap_or(&[])
    }
}

impl ProjectElementMetadata for DetailedSingleCLICommand {
    fn get_element(&self) -> &str {
        self.run.deref()
    }

    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl ProjectElementEnvironment for DetailedSingleCLICommand {
    fn get_env_vars(&self) -> Option<&HashMap<String, String>> {
        self.env.as_ref()
    }
}
