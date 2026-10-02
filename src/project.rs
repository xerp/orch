use crate::{
    commands::{AnyCommand, Command, CommandName, Commands},
    functions::{DetailedFunction, Function, FunctionName, Functions},
};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub env: Option<HashMap<String, String>>,
    pub functions: Option<HashMap<FunctionName, DetailedFunction>>,
    pub commands: HashMap<CommandName, AnyCommand>,
}

impl Project {
    pub fn print_project_info(&self) {
        let Project {
            name, description, ..
        } = self;

        println!("Name: {name}");
        println!("Description: {description}");
    }
}

pub trait ProjectElementEnvironment {
    fn get_env_vars(&self) -> HashMap<String, String> {
        HashMap::default()
    }
}

pub trait ProjectElementMetadata {
    fn get_element(&self) -> &str;
    fn get_description(&self) -> &str {
        ""
    }
}

impl Commands for Project {
    fn get_commands_metadata(&self) -> BTreeMap<&str, &impl ProjectElementMetadata> {
        self.commands
            .iter()
            .map(|(name, command)| (name.0.as_str(), command))
            .collect()
    }

    fn get_command(&self, name: &str) -> Option<&impl Command> {
        let command = CommandName(name.to_string());

        return self.commands.get(&command);
    }
}

impl Functions for Project {
    fn get_functions_metadata(&self) -> BTreeMap<&str, &impl ProjectElementMetadata> {
        self.functions
            .iter()
            .flatten()
            .map(|(name, function)| (name.0.as_str(), function))
            .collect()
    }

    fn get_functions(&self, names: &[String]) -> Vec<(&str, &impl Function)> {
        self.functions
            .iter()
            .flatten()
            .filter_map(|(name, function)| {
                names
                    .contains(&name.0)
                    .then_some((name.0.as_str(), function))
            })
            .collect()
    }
}
