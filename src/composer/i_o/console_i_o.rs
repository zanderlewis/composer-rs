//! Console IO implementation for interactive terminal output

use super::i_o_interface::{Authentication, IOInterface, Verbosity};
use crate::composer::config::Config;
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::Instant;

/// Console IO for interactive terminal input/output
#[derive(Debug)]
pub struct ConsoleIO {
    verbosity: Verbosity,
    decorated: bool,
    interactive: bool,
    authentications: RwLock<HashMap<String, Authentication>>,
    start_time: Option<Instant>,
    last_message_length: RwLock<usize>,
}

impl Clone for ConsoleIO {
    fn clone(&self) -> Self {
        Self {
            verbosity: self.verbosity,
            decorated: self.decorated,
            interactive: self.interactive,
            authentications: RwLock::new(self.authentications.read().unwrap().clone()),
            start_time: self.start_time,
            last_message_length: RwLock::new(*self.last_message_length.read().unwrap()),
        }
    }
}

impl Default for ConsoleIO {
    fn default() -> Self {
        Self::new(Verbosity::Normal, true, true)
    }
}

impl ConsoleIO {
    /// Create a new ConsoleIO instance
    pub fn new(verbosity: Verbosity, decorated: bool, interactive: bool) -> Self {
        Self {
            verbosity,
            decorated,
            interactive,
            authentications: RwLock::new(HashMap::new()),
            start_time: None,
            last_message_length: RwLock::new(0),
        }
    }

    /// Enable debugging with timing information
    pub fn enable_debugging(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Get a progress bar for showing progress
    pub fn get_progress_bar(&self, max: u64) -> indicatif::ProgressBar {
        let pb = indicatif::ProgressBar::new(max);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb
    }

    /// Format messages with debug info if enabled
    fn format_message(&self, message: &str) -> String {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            format!(
                "[{:.2}MiB/{:.2}s] {}",
                get_memory_usage_mb(),
                elapsed.as_secs_f64(),
                message
            )
        } else {
            message.to_string()
        }
    }

    fn do_write(&self, messages: &[&str], newline: bool, stderr: bool, verbosity: Verbosity, _raw: bool) {
        if verbosity > self.verbosity {
            return;
        }

        let output: Box<dyn Write> = if stderr {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
        };

        let mut handle = output;

        for message in messages {
            let formatted = self.format_message(message);
            if newline {
                writeln!(handle, "{}", formatted).ok();
            } else {
                write!(handle, "{}", formatted).ok();
            }
        }

        handle.flush().ok();
    }

    fn do_overwrite(&self, messages: &[&str], newline: bool, size: Option<usize>, stderr: bool, verbosity: Verbosity) {
        if verbosity > self.verbosity {
            return;
        }

        let last_len = *self.last_message_length.read().unwrap();
        let clear_size = size.unwrap_or(last_len);

        let output: Box<dyn Write> = if stderr {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
        };

        let mut handle = output;

        // Clear previous output
        write!(handle, "\r{}\r", " ".repeat(clear_size)).ok();

        let mut new_len = 0;
        for message in messages {
            let formatted = self.format_message(message);
            new_len += formatted.len();
            write!(handle, "{}", formatted).ok();
        }

        if newline {
            writeln!(handle).ok();
            new_len = 0;
        }

        *self.last_message_length.write().unwrap() = new_len;
        handle.flush().ok();
    }

    /// Sanitize output for safe display
    pub fn sanitize(message: &str, allow_newlines: bool) -> String {
        let mut result = String::with_capacity(message.len());
        for c in message.chars() {
            match c {
                '\n' if allow_newlines => result.push(c),
                '\n' => result.push(' '),
                '\r' | '\t' => result.push(' '),
                c if c.is_control() => {}
                c => result.push(c),
            }
        }
        result
    }

    /// Get the composer home directory
    fn get_composer_home() -> Option<PathBuf> {
        // Check COMPOSER_HOME env var first
        if let Ok(home) = std::env::var("COMPOSER_HOME") {
            return Some(PathBuf::from(home));
        }

        // Fall back to XDG or home directory
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            return Some(PathBuf::from(xdg_config).join("composer"));
        }

        // Default to ~/.composer
        dirs::home_dir().map(|h| h.join(".composer"))
    }

    /// Load auth.json from a path
    fn load_auth_json(path: &PathBuf) -> HashMap<String, Authentication> {
        let mut auths = HashMap::new();

        if !path.exists() {
            return auths;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return auths,
        };

        let json: serde_json::Value = match serde_json::from_str(&content) {
            Ok(j) => j,
            Err(_) => return auths,
        };

        // Parse http-basic authentication
        if let Some(http_basic) = json.get("http-basic").and_then(|v| v.as_object()) {
            for (host, creds) in http_basic {
                if let (Some(username), password) = (
                    creds.get("username").and_then(|v| v.as_str()),
                    creds.get("password").and_then(|v| v.as_str()),
                ) {
                    auths.insert(
                        host.clone(),
                        Authentication {
                            username: username.to_string(),
                            password: password.map(String::from),
                        },
                    );
                }
            }
        }

        // Parse github-oauth
        if let Some(github_oauth) = json.get("github-oauth").and_then(|v| v.as_object()) {
            for (host, token) in github_oauth {
                if let Some(token_str) = token.as_str() {
                    auths.insert(
                        host.clone(),
                        Authentication {
                            username: "x-oauth-basic".to_string(),
                            password: Some(token_str.to_string()),
                        },
                    );
                }
            }
        }

        // Parse gitlab-oauth
        if let Some(gitlab_oauth) = json.get("gitlab-oauth").and_then(|v| v.as_object()) {
            for (host, token) in gitlab_oauth {
                if let Some(token_str) = token.as_str() {
                    auths.insert(
                        host.clone(),
                        Authentication {
                            username: "oauth2".to_string(),
                            password: Some(token_str.to_string()),
                        },
                    );
                }
            }
        }

        // Parse gitlab-token
        if let Some(gitlab_token) = json.get("gitlab-token").and_then(|v| v.as_object()) {
            for (host, token) in gitlab_token {
                if let Some(token_str) = token.as_str() {
                    auths.insert(
                        format!("{}-token", host),
                        Authentication {
                            username: "private-token".to_string(),
                            password: Some(token_str.to_string()),
                        },
                    );
                }
            }
        }

        // Parse bearer tokens
        if let Some(bearer) = json.get("bearer").and_then(|v| v.as_object()) {
            for (host, token) in bearer {
                if let Some(token_str) = token.as_str() {
                    auths.insert(
                        format!("{}-bearer", host),
                        Authentication {
                            username: "bearer".to_string(),
                            password: Some(token_str.to_string()),
                        },
                    );
                }
            }
        }

        auths
    }
}

impl IOInterface for ConsoleIO {
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
        self.do_write(messages, newline, false, verbosity, false);
    }

    fn write_error(&self, messages: &[&str], newline: bool, verbosity: Verbosity) {
        self.do_write(messages, newline, true, verbosity, false);
    }

    fn write_raw(&self, messages: &[&str], newline: bool, verbosity: Verbosity) {
        self.do_write(messages, newline, false, verbosity, true);
    }

    fn write_error_raw(&self, messages: &[&str], newline: bool, verbosity: Verbosity) {
        self.do_write(messages, newline, true, verbosity, true);
    }

    fn overwrite(&self, messages: &[&str], newline: bool, size: Option<usize>, verbosity: Verbosity) {
        self.do_overwrite(messages, newline, size, false, verbosity);
    }

    fn overwrite_error(&self, messages: &[&str], newline: bool, size: Option<usize>, verbosity: Verbosity) {
        self.do_overwrite(messages, newline, size, true, verbosity);
    }

    fn ask(&self, question: &str, default: Option<&str>) -> Option<String> {
        if !self.interactive {
            return default.map(String::from);
        }

        print!("{}", question);
        if let Some(def) = default {
            print!(" [{}]", def.cyan());
        }
        print!(": ");
        io::stdout().flush().ok();

        let stdin = io::stdin();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_ok() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                default.map(String::from)
            } else {
                Some(trimmed.to_string())
            }
        } else {
            default.map(String::from)
        }
    }

    fn ask_confirmation(&self, question: &str, default: bool) -> bool {
        if !self.interactive {
            return default;
        }

        let default_str = if default { "Y/n" } else { "y/N" };
        print!("{} [{}]: ", question, default_str);
        io::stdout().flush().ok();

        let stdin = io::stdin();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_ok() {
            let trimmed = line.trim().to_lowercase();
            match trimmed.as_str() {
                "" => default,
                "y" | "yes" => true,
                "n" | "no" => false,
                _ => default,
            }
        } else {
            default
        }
    }

    fn ask_and_validate<F>(&self, question: &str, validator: F, attempts: Option<u32>, default: Option<&str>) -> Option<String>
    where
        F: Fn(&str) -> Result<String, String>,
    {
        let max_attempts = attempts.unwrap_or(3);
        
        for attempt in 0..max_attempts {
            if let Some(answer) = self.ask(question, default) {
                match validator(&answer) {
                    Ok(validated) => return Some(validated),
                    Err(e) => {
                        self.write_error(&[&format!("Invalid input: {}", e)], true, Verbosity::Normal);
                        if attempt + 1 < max_attempts {
                            self.write_error(&[&format!("Please try again ({} attempts remaining)", max_attempts - attempt - 1)], true, Verbosity::Normal);
                        }
                    }
                }
            } else {
                return None;
            }
        }
        
        None
    }

    fn ask_and_hide_answer(&self, question: &str) -> Option<String> {
        if !self.interactive {
            return None;
        }

        print!("{}: ", question);
        io::stdout().flush().ok();

        // Use rpassword for hidden input
        match rpassword::read_password() {
            Ok(password) => {
                if password.is_empty() {
                    None
                } else {
                    Some(password)
                }
            }
            Err(_) => None,
        }
    }

    fn select(&self, question: &str, choices: &[&str], default: Option<usize>, attempts: Option<u32>, error_message: &str, multiselect: bool) -> Vec<usize> {
        if !self.interactive {
            return default.map(|d| vec![d]).unwrap_or_default();
        }

        let max_attempts = attempts.unwrap_or(3);
        
        for _ in 0..max_attempts {
            println!("{}", question);
            for (i, choice) in choices.iter().enumerate() {
                let marker = if default == Some(i) { "*" } else { " " };
                println!("  [{}] {} {}", i, marker, choice);
            }

            if multiselect {
                print!("Enter selections (comma-separated): ");
            } else {
                print!("Enter selection: ");
            }
            io::stdout().flush().ok();

            let stdin = io::stdin();
            let mut line = String::new();
            if stdin.lock().read_line(&mut line).is_ok() {
                let trimmed = line.trim();
                
                if trimmed.is_empty() {
                    if let Some(d) = default {
                        return vec![d];
                    }
                }

                let selections: Vec<usize> = trimmed
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .filter(|&i| i < choices.len())
                    .collect();

                if !selections.is_empty() {
                    if !multiselect && selections.len() > 1 {
                        self.write_error(&[error_message], true, Verbosity::Normal);
                        continue;
                    }
                    return selections;
                }
            }

            self.write_error(&[error_message], true, Verbosity::Normal);
        }

        default.map(|d| vec![d]).unwrap_or_default()
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
        // Load authentications from global auth.json
        if let Some(composer_home) = Self::get_composer_home() {
            let global_auth = composer_home.join("auth.json");
            let global_auths = Self::load_auth_json(&global_auth);
            self.authentications.write().unwrap().extend(global_auths);
        }

        // Load local auth.json from current directory
        let local_auth = PathBuf::from("auth.json");
        let local_auths = Self::load_auth_json(&local_auth);
        self.authentications.write().unwrap().extend(local_auths);

        // Also check COMPOSER_AUTH environment variable
        if let Ok(auth_env) = std::env::var("COMPOSER_AUTH") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&auth_env) {
                // Parse http-basic from env
                if let Some(http_basic) = json.get("http-basic").and_then(|v| v.as_object()) {
                    let mut auths = self.authentications.write().unwrap();
                    for (host, creds) in http_basic {
                        if let (Some(username), password) = (
                            creds.get("username").and_then(|v| v.as_str()),
                            creds.get("password").and_then(|v| v.as_str()),
                        ) {
                            auths.insert(
                                host.clone(),
                                Authentication {
                                    username: username.to_string(),
                                    password: password.map(String::from),
                                },
                            );
                        }
                    }
                }

                // Parse github-oauth from env
                if let Some(github_oauth) = json.get("github-oauth").and_then(|v| v.as_object()) {
                    let mut auths = self.authentications.write().unwrap();
                    for (host, token) in github_oauth {
                        if let Some(token_str) = token.as_str() {
                            auths.insert(
                                host.clone(),
                                Authentication {
                                    username: "x-oauth-basic".to_string(),
                                    password: Some(token_str.to_string()),
                                },
                            );
                        }
                    }
                }

                // Parse gitlab-oauth from env
                if let Some(gitlab_oauth) = json.get("gitlab-oauth").and_then(|v| v.as_object()) {
                    let mut auths = self.authentications.write().unwrap();
                    for (host, token) in gitlab_oauth {
                        if let Some(token_str) = token.as_str() {
                            auths.insert(
                                host.clone(),
                                Authentication {
                                    username: "oauth2".to_string(),
                                    password: Some(token_str.to_string()),
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Get approximate memory usage in MB
fn get_memory_usage_mb() -> f64 {
    // On Linux, we can read from /proc/self/statm
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/self/statm") {
            if let Some(rss) = content.split_whitespace().nth(1) {
                if let Ok(pages) = rss.parse::<u64>() {
                    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u64;
                    return (pages * page_size) as f64 / 1024.0 / 1024.0;
                }
            }
        }
    }
    0.0
}
