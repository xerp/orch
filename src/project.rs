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
    fn get_env_vars(&self) -> Option<&HashMap<String, String>> {
        None
    }
}

pub trait ProjectElementMetadata {
    fn get_element(&self) -> &str;
    fn get_description(&self) -> Option<&str> {
        None
    }
}

impl Commands for Project {
    fn get_commands_metadata(&self) -> BTreeMap<&str, &impl ProjectElementMetadata> {
        self.commands
            .iter()
            .map(|(name, command)| (&**name, command))
            .collect()
    }

    fn get_command(&self, name: &str) -> Option<&impl Command> {
        self.commands.get(name)
    }
}

impl Functions for Project {
    fn get_functions_metadata(&self) -> BTreeMap<&str, &impl ProjectElementMetadata> {
        self.functions
            .iter()
            .flatten()
            .map(|(name, function)| (&**name, function))
            .collect()
    }

    fn get_functions<T: AsRef<str>>(&self, names: &[T]) -> HashMap<&str, &impl Function> {
        self.functions
            .iter()
            .flatten()
            .filter(|(name, _function)| names.iter().any(|n| n.as_ref() == name.0))
            .map(|(name, function)| (name.0.as_ref(), function))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_minimal_project_succesfully() {
        let minimal_yaml_data = "
            name: test-project
            description: test-project-description
            commands:
                example: echo 1
        ";

        let project: Project =
            serde_yaml::from_str(minimal_yaml_data).expect("Minimal YAML should parse");

        let commands: Vec<&str> = project
            .get_commands_metadata()
            .iter()
            .map(|(name, _metadata)| *name)
            .collect::<Vec<&str>>();

        let functions: Vec<&str> = project
            .get_functions_metadata()
            .iter()
            .map(|(name, _metadata)| *name)
            .collect::<Vec<&str>>();

        assert_eq!(project.name, "test-project");
        assert_eq!(project.description, "test-project-description");

        assert_eq!(functions.len(), 0);
        assert_eq!(commands, vec!["example"]);
    }

    #[test]
    fn should_not_load_project_without_name_description_and_commands() {
        let minimal_yaml_data = "";

        let result: Result<Project, _> = serde_yaml::from_str(minimal_yaml_data);

        assert!(result.is_err());
    }

    #[test]
    fn should_load_project_with_functions() {
        let yaml_data = "
            name: test-project
            description: test-project-description
            functions:
                hello-world:
                    body: echo 1
            commands:
                example: echo 1
        ";

        let project: Project =
            serde_yaml::from_str(yaml_data).expect("YAML with functions should parse");

        let functions: Vec<&str> = project
            .get_functions_metadata()
            .iter()
            .map(|(name, _metadata)| *name)
            .collect::<Vec<&str>>();

        assert_eq!(functions, vec!["hello-world"]);
    }

    #[test]
    fn should_load_project_with_envs() {
        let yaml_data = "
            name: test-project
            description: test-project-description
            env:
                APP_ENV: development
                API_URL: http://localhost:8080
            commands:
                example: echo $APP_ENV
        ";

        let project: Project =
            serde_yaml::from_str(yaml_data).expect("YAML with envs should parse");

        let expected_env = HashMap::from([
            ("APP_ENV".to_string(), "development".to_string()),
            ("API_URL".to_string(), "http://localhost:8080".to_string()),
        ]);

        assert_eq!(project.env, Some(expected_env));
    }
}
