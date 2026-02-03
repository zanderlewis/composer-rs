//! Null IO implementation - discards all output

use super::i_o_interface::{Authentication, IOInterface, Verbosity};
use crate::composer::config::Config;
use std::collections::HashMap;

/// Null IO that discards all output and returns defaults for input
/// Useful for non-interactive/silent operations
#[derive(Debug, Clone, Default)]
pub struct NullIO {
    authentications: HashMap<String, Authentication>,
}

impl NullIO {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IOInterface for NullIO {
    fn is_interactive(&self) -> bool {
        false
    }

    fn is_verbose(&self) -> bool {
        false
    }

    fn is_very_verbose(&self) -> bool {
        false
    }

    fn is_debug(&self) -> bool {
        false
    }

    fn is_decorated(&self) -> bool {
        false
    }

    fn write(&self, _messages: &[&str], _newline: bool, _verbosity: Verbosity) {
        // Discard output
    }

    fn write_error(&self, _messages: &[&str], _newline: bool, _verbosity: Verbosity) {
        // Discard output
    }

    fn write_raw(&self, _messages: &[&str], _newline: bool, _verbosity: Verbosity) {
        // Discard output
    }

    fn write_error_raw(&self, _messages: &[&str], _newline: bool, _verbosity: Verbosity) {
        // Discard output
    }

    fn overwrite(&self, _messages: &[&str], _newline: bool, _size: Option<usize>, _verbosity: Verbosity) {
        // Discard output
    }

    fn overwrite_error(&self, _messages: &[&str], _newline: bool, _size: Option<usize>, _verbosity: Verbosity) {
        // Discard output
    }

    fn ask(&self, _question: &str, default: Option<&str>) -> Option<String> {
        default.map(String::from)
    }

    fn ask_confirmation(&self, _question: &str, default: bool) -> bool {
        default
    }

    fn ask_and_validate<F>(&self, _question: &str, _validator: F, _attempts: Option<u32>, default: Option<&str>) -> Option<String>
    where
        F: Fn(&str) -> Result<String, String>,
    {
        default.map(String::from)
    }

    fn ask_and_hide_answer(&self, _question: &str) -> Option<String> {
        None
    }

    fn select(&self, _question: &str, _choices: &[&str], default: Option<usize>, _attempts: Option<u32>, _error_message: &str, _multiselect: bool) -> Vec<usize> {
        default.map(|d| vec![d]).unwrap_or_default()
    }

    fn get_authentications(&self) -> HashMap<String, Authentication> {
        self.authentications.clone()
    }

    fn has_authentication(&self, repository_name: &str) -> bool {
        self.authentications.contains_key(repository_name)
    }

    fn get_authentication(&self, repository_name: &str) -> Option<Authentication> {
        self.authentications.get(repository_name).cloned()
    }

    fn set_authentication(&mut self, repository_name: &str, username: &str, password: Option<&str>) {
        self.authentications.insert(
            repository_name.to_string(),
            Authentication {
                username: username.to_string(),
                password: password.map(String::from),
            },
        );
    }

    fn load_configuration(&mut self, _config: &Config) {
        // Nothing to load for NullIO
    }
}

