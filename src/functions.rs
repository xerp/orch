use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

use crate::project::{ProjectElementEnvironment, ProjectElementMetadata};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct FunctionName(pub String);

pub trait Function: ProjectElementMetadata + ProjectElementEnvironment {
    fn build(&self, function_name: &str) -> String;
}

pub trait Functions {
    fn print_available_functions(&self) {
        for (function_name, metadata) in self.get_functions_metadata() {
            match metadata.get_description() {
                "" => println!("- {function_name}"),
                description => println!("- {function_name}: {description}"),
            }
        }
    }
    fn get_functions_metadata(&self) -> BTreeMap<&str, &impl ProjectElementMetadata>;
    fn get_functions(&self, names: &[String]) -> Vec<(&str, &impl Function)>;
}

#[derive(Debug, Deserialize)]
pub struct DetailedFunction {
    description: Option<String>,
    env: Option<HashMap<String, String>>,
    body: String,
}

impl ProjectElementEnvironment for DetailedFunction {
    fn get_env_vars(&self) -> HashMap<String, String> {
        self.env.clone().unwrap_or_default()
    }
}

impl ProjectElementMetadata for DetailedFunction {
    fn get_element(&self) -> &str {
        self.body.as_str()
    }

    fn get_description(&self) -> &str {
        self.description.as_deref().unwrap_or_default()
    }
}

impl Function for DetailedFunction {
    fn build(&self, function_name: &str) -> String {
        let function_name_snake_case = function_name.replace("-", "_");
        let body = self.body.lines().collect::<Vec<_>>().join("\n\t");

        return format!("{function_name_snake_case}() {{\n\t{body}\n}}");
    }
}
