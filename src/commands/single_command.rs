use serde::Deserialize;

use crate::commands::{Command, CommandMetadata};

#[derive(Debug, Deserialize)]
pub struct SingleCommand(String);

impl Command for SingleCommand {}

impl CommandMetadata for SingleCommand {
    fn get_command(&self) -> &str {
        return self.0.as_str();
    }
}

#[derive(Debug, Deserialize)]
pub struct DetailedSingleCommand {
    pub command: String,
    pub description: Option<String>,
}

impl Command for DetailedSingleCommand {}

impl CommandMetadata for DetailedSingleCommand {
    fn get_command(&self) -> &str {
        return self.command.as_str();
    }

    fn get_description(&self) -> &str {
        return self.description.as_deref().unwrap_or("");
    }
}
