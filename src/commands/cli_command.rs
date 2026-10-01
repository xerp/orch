use std::{collections::HashMap, process::Command as ProcessCommand};

use serde::Deserialize;

use crate::commands::{Command, CommandEnvironment, CommandMetadata};

trait CLICommand: CommandMetadata + CommandEnvironment {}

impl<T: CLICommand> Command for T {
    fn run(&self, command_name: &str, env_vars: &HashMap<String, String>, args: &[String]) -> bool {
        let command = self.get_command();
        let command_env_vars = self.get_env_vars();

        // Merging all env vars: command env vars will override the other env vars
        let all_env_vars: HashMap<_, _> = env_vars.iter().chain(command_env_vars.iter()).collect();

        let functions = ["function say_hello() {\necho \"Hello\"\necho \"$1\"\n}"];
        let script = format!(
            "{before_scripts}\n{command}",
            before_scripts = functions.join("\n")
        );

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

impl CommandMetadata for SingleCLICommand {
    fn get_command(&self) -> &str {
        return self.0.as_str();
    }
}

impl CommandEnvironment for SingleCLICommand {
    fn get_env_vars(&self) -> HashMap<String, String> {
        return HashMap::default();
    }
}

#[derive(Debug, Deserialize)]
pub struct DetailedSingleCLICommand {
    pub command: String,
    pub description: Option<String>,
    pub env: Option<HashMap<String, String>>,
}

impl CLICommand for DetailedSingleCLICommand {}

impl CommandMetadata for DetailedSingleCLICommand {
    fn get_command(&self) -> &str {
        return self.command.as_str();
    }

    fn get_description(&self) -> &str {
        return self.description.as_deref().unwrap_or_default();
    }
}

impl CommandEnvironment for DetailedSingleCLICommand {
    fn get_env_vars(&self) -> HashMap<String, String> {
        return self.env.clone().unwrap_or_default();
    }
}
