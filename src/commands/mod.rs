pub mod cli_command;

use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

use crate::commands::cli_command::{DetailedSingleCLICommand, SingleCLICommand};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct CommandName(pub String);

pub trait Commands {
    fn print_available_commands(&self) {
        for (command_name, metadata) in self.get_metadata() {
            match metadata.get_description() {
                "" => println!("- {command_name}"),
                description => println!("- {command_name}: {description}"),
            }
        }
    }
    fn get_metadata(&self) -> BTreeMap<&str, &impl CommandMetadata>;
    fn get_command(&self, name: &str) -> Option<&impl Command>;
}

pub trait CommandEnvironment {
    fn get_env_vars(&self) -> HashMap<String, String>;
}

pub trait CommandMetadata {
    fn get_command(&self) -> &str;
    fn get_description(&self) -> &str {
        ""
    }
}

pub trait Command: CommandMetadata + CommandEnvironment {
    fn run(&self, command_name: &str, env_vars: &HashMap<String, String>, args: &[String]) -> bool;
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnyCommand {
    Simple(SingleCLICommand),
    Detailed(DetailedSingleCLICommand),
}

impl AnyCommand {
    fn as_command(&self) -> &dyn Command {
        match self {
            AnyCommand::Simple(cmd) => cmd as &dyn Command,
            AnyCommand::Detailed(cmd) => cmd as &dyn Command,
        }
    }
}

impl CommandMetadata for AnyCommand {
    fn get_command(&self) -> &str {
        return self.as_command().get_command();
    }

    fn get_description(&self) -> &str {
        return self.as_command().get_description();
    }
}

impl CommandEnvironment for AnyCommand {
    fn get_env_vars(&self) -> HashMap<String, String> {
        return self.as_command().get_env_vars();
    }
}

impl Command for AnyCommand {
    fn run(&self, command_name: &str, env_vars: &HashMap<String, String>, args: &[String]) -> bool {
        return self.as_command().run(command_name, env_vars, args);
    }
}
