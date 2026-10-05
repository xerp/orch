pub mod cli_command;

use serde::Deserialize;
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
};

use crate::{
    commands::cli_command::{DetailedSingleCLICommand, SingleCLICommand},
    project::{Project, ProjectElementEnvironment, ProjectElementMetadata},
};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct CommandName(pub String);

impl Borrow<str> for CommandName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

pub trait Command: ProjectElementMetadata + ProjectElementEnvironment {
    fn run(&self, command_name: &str, project: &Project, args: &[String]) -> bool;
}

pub trait Commands {
    fn print_available_commands(&self) {
        for (command_name, metadata) in self.get_commands_metadata() {
            match metadata.get_description() {
                None => println!("- {command_name}"),
                Some(description) => println!("- {command_name}: {description}"),
            }
        }
    }
    fn get_commands_metadata(&self) -> BTreeMap<&str, &impl ProjectElementMetadata>;
    fn get_command(&self, name: &str) -> Option<&impl Command>;
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

impl ProjectElementMetadata for AnyCommand {
    fn get_element(&self) -> &str {
        self.as_command().get_element()
    }

    fn get_description(&self) -> Option<&str> {
        self.as_command().get_description()
    }
}

impl ProjectElementEnvironment for AnyCommand {
    fn get_env_vars(&self) -> Option<&HashMap<String, String>> {
        self.as_command().get_env_vars()
    }
}

impl Command for AnyCommand {
    fn run(&self, command_name: &str, project: &Project, args: &[String]) -> bool {
        self.as_command().run(command_name, project, args)
    }
}
