use std::{collections::HashMap, io::Result};

use serde::Deserialize;
// use ureq::{Body, http::Response};

use crate::{
    commands::Command,
    project::{Project, ProjectElementEnvironment, ProjectElementMetadata},
};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    #[default]
    #[serde(alias = "get")]
    Get,
    // #[serde(alias = "post")]
    // Post,
}

#[derive(Debug, Deserialize)]
pub struct HttpCommand {
    pub description: Option<String>,
    pub url: String,

    #[serde(default)]
    pub method: HttpMethod,
    pub env: Option<HashMap<String, String>>,
    // pub body: Option<HashMap<String, String>>,
    pub query: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<String, String>>,
    pub run: String,
}

impl ProjectElementMetadata for HttpCommand {
    fn get_element(&self) -> &str {
        &*self.run
    }
    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl ProjectElementEnvironment for HttpCommand {
    fn get_env_vars(&self) -> Option<&HashMap<String, String>> {
        self.env.as_ref()
    }
}

impl Command for HttpCommand {
    fn run(
        &self,
        _command_name: &str,
        _project: &Project,
        _args: &[String],
    ) -> Result<Option<i32>> {
        let request_url = &self.url;
        let request_query = self.query.iter().flatten();
        let request_headers = self.headers.iter().flatten();

        let _result = match self.method {
            HttpMethod::Get => {
                let mut request = ureq::get(request_url).query_pairs(request_query);

                for (header_name, header_value) in request_headers {
                    request = request.header(header_name, header_value);
                }

                request.call()
            } // Some(HttpMethod::Post) => ureq::post(request_url).send_json(""),
        };
        // let request = ;

        // let result = request.call();
        Ok(Some(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_default_http_method() {
        let yaml = "url: http://localhost\nrun: echo done";

        let command: HttpCommand = serde_yaml::from_str(yaml).expect("Error on parsing YAML");

        assert_eq!(command.method, HttpMethod::Get)
    }
}
