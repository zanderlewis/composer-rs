//! IO Interface for Composer operations
//!
//! Defines the interface for handling input/output operations.

use crate::composer::config::Config;
use std::collections::HashMap;

/// Verbosity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Quiet = 0,
    Normal = 1,
    Verbose = 2,
    VeryVerbose = 3,
    Debug = 4,
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::Normal
    }
}

impl From<i32> for Verbosity {
    fn from(v: i32) -> Self {
        match v {
            0 => Verbosity::Quiet,
            1 => Verbosity::Normal,
            2 => Verbosity::Verbose,
            3 => Verbosity::VeryVerbose,
            _ => Verbosity::Debug,
        }
    }
}

/// Authentication credentials
#[derive(Debug, Clone, Default)]
pub struct Authentication {
    pub username: String,
    pub password: Option<String>,
}

impl Authentication {
    pub fn new(username: impl Into<String>, password: Option<impl Into<String>>) -> Self {
        Self {
            username: username.into(),
            password: password.map(|p| p.into()),
        }
    }

    /// Create HTTP Basic auth header value
    pub fn to_basic_auth(&self) -> String {
        let credentials = match &self.password {
            Some(pass) => format!("{}:{}", self.username, pass),
            None => self.username.clone(),
        };
        format!("Basic {}", base64::Engine::encode(&base64::engine::general_purpose::STANDARD, credentials.as_bytes()))
    }

    /// Create Bearer auth header value
    pub fn to_bearer_auth(&self) -> Option<String> {
        self.password.as_ref().map(|token| format!("Bearer {}", token))
    }
}

/// IO Interface trait for handling input/output
pub trait IOInterface: Send + Sync {
    /// Check if the IO is interactive
    fn is_interactive(&self) -> bool;

    /// Check if verbosity is at least Verbose
    fn is_verbose(&self) -> bool;

    /// Check if verbosity is at least VeryVerbose
    fn is_very_verbose(&self) -> bool;

    /// Check if verbosity is Debug
    fn is_debug(&self) -> bool;

    /// Check if output should be decorated (colored)
    fn is_decorated(&self) -> bool;

    /// Write a message to stdout
    fn write(&self, messages: &[&str], newline: bool, verbosity: Verbosity);

    /// Write a message to stderr
    fn write_error(&self, messages: &[&str], newline: bool, verbosity: Verbosity);

    /// Write raw output (no formatting) to stdout
    fn write_raw(&self, messages: &[&str], newline: bool, verbosity: Verbosity);

    /// Write raw output (no formatting) to stderr
    fn write_error_raw(&self, messages: &[&str], newline: bool, verbosity: Verbosity);

    /// Overwrite previous output on stdout
    fn overwrite(&self, messages: &[&str], newline: bool, size: Option<usize>, verbosity: Verbosity);

    /// Overwrite previous output on stderr
    fn overwrite_error(&self, messages: &[&str], newline: bool, size: Option<usize>, verbosity: Verbosity);

    /// Ask a question and return the answer
    fn ask(&self, question: &str, default: Option<&str>) -> Option<String>;

    /// Ask for confirmation
    fn ask_confirmation(&self, question: &str, default: bool) -> bool;

    /// Ask a question with validation
    fn ask_and_validate<F>(&self, question: &str, validator: F, attempts: Option<u32>, default: Option<&str>) -> Option<String>
    where
        F: Fn(&str) -> Result<String, String>;

    /// Ask for input with hidden answer (for passwords)
    fn ask_and_hide_answer(&self, question: &str) -> Option<String>;

    /// Present a selection of choices
    fn select(&self, question: &str, choices: &[&str], default: Option<usize>, attempts: Option<u32>, error_message: &str, multiselect: bool) -> Vec<usize>;

    /// Get all stored authentications (returns cloned map)
    fn get_authentications(&self) -> HashMap<String, Authentication>;

    /// Check if authentication exists for a repository
    fn has_authentication(&self, repository_name: &str) -> bool;

    /// Get authentication for a repository (returns cloned auth)
    fn get_authentication(&self, repository_name: &str) -> Option<Authentication>;

    /// Set authentication for a repository
    fn set_authentication(&mut self, repository_name: &str, username: &str, password: Option<&str>);

    /// Load configuration (authentication from auth.json etc)
    fn load_configuration(&mut self, config: &Config);

    // Convenience methods with defaults

    /// Write a single line to stdout with normal verbosity
    fn writeln(&self, message: &str) {
        self.write(&[message], true, Verbosity::Normal);
    }

    /// Write a single line to stderr with normal verbosity
    fn write_errorln(&self, message: &str) {
        self.write_error(&[message], true, Verbosity::Normal);
    }

    /// Write info message (blue)
    fn info(&self, message: &str) {
        self.write(&[message], true, Verbosity::Normal);
    }

    /// Write warning message (yellow)
    fn warning(&self, message: &str) {
        self.write_error(&[&format!("<warning>{}</warning>", message)], true, Verbosity::Normal);
    }

    /// Write error message (red)
    fn error(&self, message: &str) {
        self.write_error(&[&format!("<error>{}</error>", message)], true, Verbosity::Normal);
    }
}

