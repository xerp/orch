use convert_case::{Case, Casing};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
};

use serde::Deserialize;

use crate::project::{ProjectElementEnvironment, ProjectElementMetadata};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct FunctionName(pub String);

impl Borrow<str> for FunctionName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

pub trait Function: ProjectElementMetadata + ProjectElementEnvironment {
    fn build(&self, function_name: &str) -> String;
}

pub trait Functions {
    fn print_available_functions(&self) {
        for (function_name, metadata) in self.get_functions_metadata() {
            match metadata.get_description() {
                None => println!("- {function_name}"),
                Some(description) => println!("- {function_name}: {description}"),
            }
        }
    }
    fn get_functions_metadata(&self) -> BTreeMap<&str, &impl ProjectElementMetadata>;
    fn get_functions<T: AsRef<str>>(&self, names: &[T]) -> HashMap<&str, &impl Function>;
}

#[derive(Debug, Deserialize)]
pub struct DetailedFunction {
    description: Option<String>,
    env: Option<HashMap<String, String>>,
    body: String,
}

impl ProjectElementEnvironment for DetailedFunction {
    fn get_env_vars(&self) -> Option<&HashMap<String, String>> {
        self.env.as_ref()
    }
}

impl ProjectElementMetadata for DetailedFunction {
    fn get_element(&self) -> &str {
        self.body.as_str()
    }

    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl Function for DetailedFunction {
    fn build(&self, function_name: &str) -> String {
        let function_name_snake_case = function_name.to_case(Case::Snake);
        let body = self.body.lines().collect::<Vec<_>>().join("\n\t");

        return format!("{function_name_snake_case}() {{\n\t{body}\n}}");
    }
}
