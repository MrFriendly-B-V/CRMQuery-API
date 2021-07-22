pub type Result<T> = std::result::Result<T, Error>;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg:  String,
    pub line: u32,
    pub file: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let description = match &self.kind {
            ErrorKind::Req(_) => "An error occurred while sending a HTTP request.",
            ErrorKind::Json(_) => "An error occurred while (de)serializing JSON data.",
            ErrorKind::Envy(_) => "An error occurred while parsing environmental variables to a struct.",
            ErrorKind::Other(_) => "An error occurred",
            ErrorKind::Io(_) => "An error occurred while performing an IO operation.",
            ErrorKind::Yaml(_) => "An error occurred while (de)serializing YAML data.",
        };

        writeln!(f, "{} The error represented as a String: {}. The error occurred at line {} in file {}", description, self.kind, self.line, self.file)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Req(reqwest::Error),
    Envy(envy::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Other(String)
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: '{:?}'", self)
    }
}

impl From<envy::Error> for ErrorKind {
    fn from(a: envy::Error) -> Self {
        Self::Envy(a)
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(a: serde_json::Error) -> Self {
        Self::Json(a)
    }
}

impl From<String> for ErrorKind {
    fn from(a: String) -> Self {
        Self::Other(a)
    }
}

impl From<&str> for ErrorKind {
    fn from(a: &str) -> Self {
        Self::Other(a.to_string())
    }
}

impl From<reqwest::Error> for ErrorKind {
    fn from(a: reqwest::Error) -> Self {
        Self::Req(a)
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(a: std::io::Error) -> Self {
        Self::Io(a)
    }
}

impl From<serde_yaml::Error> for ErrorKind {
    fn from(a: serde_yaml::Error) -> Self {
        Self::Yaml(a)
    }
}

#[macro_export]
macro_rules! error {
    ($err:expr, $desc:expr) => {
        {
            let kind = $crate::result::ErrorKind::from($err);
            let error: $crate::result::Error = $crate::result::Error {
                line:   std::line!(),
                file:   std::file!().to_string(),
                kind,
                msg: $desc.to_string()
            };

            error
        }
    }
}