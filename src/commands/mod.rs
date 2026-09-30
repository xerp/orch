use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    process::{Child, Command as ProcessCommand},
};

use crate::commands::single_command::{DetailedSingleCommand, SingleCommand};

pub mod single_command;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct CommandName(pub String);

pub trait Commands {
    fn print_available_commands(&self) {
        for (command_name, metadata) in self.get_metadata() {
            match metadata.get_description() {
                "" => println!("- {}", command_name),
                description => println!("- {}: {}", command_name, description),
            }
        }
    }
    fn get_metadata(&self) -> BTreeMap<&str, &impl CommandMetadata>;
    fn get_command(&self, name: &str) -> Option<&impl Command>;
}

pub trait CommandMetadata {
    fn get_command(&self) -> &str;
    fn get_description(&self) -> &str {
        ""
    }
}

pub trait Command: CommandMetadata {
    fn run(&self) -> Child {
        let command = self.get_command();

        let child = ProcessCommand::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("Failed to execute");

        return child;
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnyCommand {
    Simple(SingleCommand),
    Detailed(DetailedSingleCommand),
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

impl Command for AnyCommand {
    fn run(&self) -> Child {
        return self.as_command().run();
    }
}
