//! Buffer IO implementation - captures all output to a buffer
//!
//! Useful for testing and capturing output programmatically.

use super::i_o_interface::{Authentication, IOInterface, Verbosity};
use crate::composer::config::Config;
use std::collections::HashMap;
use std::sync::RwLock;

/// Buffer IO that captures all output to internal buffers
/// and can provide pre-set inputs for testing
#[derive(Debug)]
pub struct BufferIO {
    verbosity: Verbosity,
    decorated: bool,
    interactive: bool,
    output: RwLock<String>,
    error_output: RwLock<String>,
    inputs: RwLock<Vec<String>>,
    input_index: RwLock<usize>,
    authentications: RwLock<HashMap<String, Authentication>>,
}

impl Clone for BufferIO {
    fn clone(&self) -> Self {
        Self {
            verbosity: self.verbosity,
            decorated: self.decorated,
            interactive: self.interactive,
            output: RwLock::new(self.output.read().unwrap().clone()),
            error_output: RwLock::new(self.error_output.read().unwrap().clone()),
            inputs: RwLock::new(self.inputs.read().unwrap().clone()),
            input_index: RwLock::new(*self.input_index.read().unwrap()),
            authentications: RwLock::new(self.authentications.read().unwrap().clone()),
        }
    }
}

impl Default for BufferIO {
    fn default() -> Self {
        Self::new(Verbosity::Normal, true)
    }
}

impl BufferIO {
    /// Create a new BufferIO instance
    pub fn new(verbosity: Verbosity, decorated: bool) -> Self {
        Self {
            verbosity,
            decorated,
            interactive: true,
            output: RwLock::new(String::new()),
            error_output: RwLock::new(String::new()),
            inputs: RwLock::new(Vec::new()),
            input_index: RwLock::new(0),
            authentications: RwLock::new(HashMap::new()),
        }
    }

    /// Get all captured stdout output
    pub fn get_output(&self) -> String {
        self.output.read().unwrap().clone()
    }

    /// Get all captured stderr output
    pub fn get_error_output(&self) -> String {
        self.error_output.read().unwrap().clone()
    }

    /// Clear all captured output
    pub fn clear_output(&self) {
        *self.output.write().unwrap() = String::new();
        *self.error_output.write().unwrap() = String::new();
    }

    /// Set predefined user inputs for testing
    pub fn set_user_inputs(&self, inputs: Vec<String>) {
        *self.inputs.write().unwrap() = inputs;
        *self.input_index.write().unwrap() = 0;
    }

    /// Get the next predefined input, or None if exhausted
    fn get_next_input(&self) -> Option<String> {
        let mut index = self.input_index.write().unwrap();
        let inputs = self.inputs.read().unwrap();
        if *index < inputs.len() {
            let input = inputs[*index].clone();
            *index += 1;
            Some(input)
        } else {
            None
        }
    }

    fn do_write(&self, messages: &[&str], newline: bool, stderr: bool, verbosity: Verbosity) {
        if verbosity > self.verbosity {
            return;
        }

        let buffer = if stderr {
            &self.error_output
        } else {
            &self.output
        };

        let mut buf = buffer.write().unwrap();
        for message in messages {
            buf.push_str(message);
            if newline {
                buf.push('\n');
            }
        }
    }
}

impl IOInterface for BufferIO {
    fn is_interactive(&self) -> bool {
        self.interactive
    }

    fn is_verbose(&self) -> bool {
        self.verbosity >= Verbosity::Verbose
    }

    fn is_very_verbose(&self) -> bool {
        self.verbosity >= Verbosity::VeryVerbose
    }

    fn is_debug(&self) -> bool {
        self.verbosity >= Verbosity::Debug
    }

    fn is_decorated(&self) -> bool {
        self.decorated
    }

    fn write(&self, messages: &[&str], newline: bool, verbosity: Verbosity) {
        self.do_write(messages, newline, false, verbosity);
    }

    fn write_error(&self, messages: &[&str], newline: bool, verbosity: Verbosity) {
        self.do_write(messages, newline, true, verbosity);
    }

    fn write_raw(&self, messages: &[&str], newline: bool, verbosity: Verbosity) {
        self.do_write(messages, newline, false, verbosity);
    }

    fn write_error_raw(&self, messages: &[&str], newline: bool, verbosity: Verbosity) {
        self.do_write(messages, newline, true, verbosity);
    }

    fn overwrite(&self, messages: &[&str], newline: bool, _size: Option<usize>, verbosity: Verbosity) {
        self.do_write(messages, newline, false, verbosity);
    }

    fn overwrite_error(&self, messages: &[&str], newline: bool, _size: Option<usize>, verbosity: Verbosity) {
        self.do_write(messages, newline, true, verbosity);
    }

    fn ask(&self, _question: &str, default: Option<&str>) -> Option<String> {
        self.get_next_input().or_else(|| default.map(String::from))
    }

    fn ask_confirmation(&self, _question: &str, default: bool) -> bool {
        match self.get_next_input() {
            Some(input) => {
                let lower = input.to_lowercase();
                match lower.as_str() {
                    "y" | "yes" | "true" | "1" => true,
                    "n" | "no" | "false" | "0" => false,
                    _ => default,
                }
            }
            None => default,
        }
    }

    fn ask_and_validate<F>(&self, _question: &str, validator: F, _attempts: Option<u32>, default: Option<&str>) -> Option<String>
    where
        F: Fn(&str) -> Result<String, String>,
    {
        let input = self.get_next_input().or_else(|| default.map(String::from))?;
        validator(&input).ok()
    }

    fn ask_and_hide_answer(&self, _question: &str) -> Option<String> {
        self.get_next_input()
    }

    fn select(&self, _question: &str, _choices: &[&str], default: Option<usize>, _attempts: Option<u32>, _error_message: &str, _multiselect: bool) -> Vec<usize> {
        match self.get_next_input() {
            Some(input) => {
                input
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect()
            }
            None => default.map(|d| vec![d]).unwrap_or_default(),
        }
    }

    fn get_authentications(&self) -> HashMap<String, Authentication> {
        self.authentications.read().unwrap().clone()
    }

    fn has_authentication(&self, repository_name: &str) -> bool {
        self.authentications.read().unwrap().contains_key(repository_name)
    }

    fn get_authentication(&self, repository_name: &str) -> Option<Authentication> {
        self.authentications.read().unwrap().get(repository_name).cloned()
    }

    fn set_authentication(&mut self, repository_name: &str, username: &str, password: Option<&str>) {
        self.authentications.write().unwrap().insert(
            repository_name.to_string(),
            Authentication {
                username: username.to_string(),
                password: password.map(String::from),
            },
        );
    }

    fn load_configuration(&mut self, _config: &Config) {
        // BufferIO doesn't load from config - it's for testing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_io_captures_output() {
        let io = BufferIO::new(Verbosity::Normal, false);
        io.write(&["Hello"], true, Verbosity::Normal);
        io.write(&["World"], true, Verbosity::Normal);
        
        assert_eq!(io.get_output(), "Hello\nWorld\n");
    }

    #[test]
    fn test_buffer_io_captures_error_output() {
        let io = BufferIO::new(Verbosity::Normal, false);
        io.write_error(&["Error!"], true, Verbosity::Normal);
        
        assert_eq!(io.get_error_output(), "Error!\n");
    }

    #[test]
    fn test_buffer_io_respects_verbosity() {
        let io = BufferIO::new(Verbosity::Normal, false);
        io.write(&["Normal"], true, Verbosity::Normal);
        io.write(&["Verbose"], true, Verbosity::Verbose);
        
        assert_eq!(io.get_output(), "Normal\n");
    }

    #[test]
    fn test_buffer_io_user_inputs() {
        let io = BufferIO::new(Verbosity::Normal, false);
        io.set_user_inputs(vec!["input1".to_string(), "input2".to_string()]);
        
        assert_eq!(io.ask("Question?", None), Some("input1".to_string()));
        assert_eq!(io.ask("Question?", None), Some("input2".to_string()));
        assert_eq!(io.ask("Question?", Some("default")), Some("default".to_string()));
    }

    #[test]
    fn test_buffer_io_confirmation() {
        let io = BufferIO::new(Verbosity::Normal, false);
        io.set_user_inputs(vec!["y".to_string(), "no".to_string()]);
        
        assert!(io.ask_confirmation("Continue?", false));
        assert!(!io.ask_confirmation("Continue?", true));
    }
}

