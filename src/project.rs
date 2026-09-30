use crate::commands::{AnyCommand, Command, CommandMetadata, CommandName, Commands};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub commands: HashMap<CommandName, AnyCommand>,
}

impl Project {
    pub fn print_project_info(&self) {
        println!("Name: {}", self.name);
        println!("Description: {}", self.description);
    }
}

impl Commands for HashMap<CommandName, AnyCommand> {
    fn get_metadata(&self) -> BTreeMap<&str, &impl CommandMetadata> {
        return self
            .iter()
            .map(|(name, command)| (name.0.as_str(), command))
            .collect();
    }

    fn get_command(&self, name: &str) -> Option<&impl Command> {
        let command = CommandName(name.to_string());

        return self.get(&command);
    }
}
