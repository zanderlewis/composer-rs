//! Base IO - Shared authentication and logging functionality
//!
//! This module provides common authentication handling used by all IO implementations.

use super::i_o_interface::Authentication;
use crate::composer::config::Config;
use std::collections::HashMap;
use std::path::PathBuf;

/// Load authentication from auth.json file
pub fn load_auth_json(path: &PathBuf) -> HashMap<String, Authentication> {
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
            if let Some(username) = creds.get("username").and_then(|v| v.as_str()) {
                let password = creds.get("password").and_then(|v| v.as_str());
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

    // Parse bitbucket-oauth
    if let Some(bitbucket_oauth) = json.get("bitbucket-oauth").and_then(|v| v.as_object()) {
        for (host, creds) in bitbucket_oauth {
            if let (Some(consumer_key), Some(consumer_secret)) = (
                creds.get("consumer-key").and_then(|v| v.as_str()),
                creds.get("consumer-secret").and_then(|v| v.as_str()),
            ) {
                auths.insert(
                    host.clone(),
                    Authentication {
                        username: consumer_key.to_string(),
                        password: Some(consumer_secret.to_string()),
                    },
                );
            }
        }
    }

    auths
}

/// Get the composer home directory
pub fn get_composer_home() -> Option<PathBuf> {
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

/// Load authentications from COMPOSER_AUTH environment variable
pub fn load_auth_from_env() -> HashMap<String, Authentication> {
    let mut auths = HashMap::new();
    
    if let Ok(auth_env) = std::env::var("COMPOSER_AUTH") {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&auth_env) {
            // Parse http-basic from env
            if let Some(http_basic) = json.get("http-basic").and_then(|v| v.as_object()) {
                for (host, creds) in http_basic {
                    if let Some(username) = creds.get("username").and_then(|v| v.as_str()) {
                        let password = creds.get("password").and_then(|v| v.as_str());
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
    
    auths
}

/// Load all authentications (global, local, and env)
pub fn load_all_authentications() -> HashMap<String, Authentication> {
    let mut auths = HashMap::new();
    
    // Load global auth.json
    if let Some(composer_home) = get_composer_home() {
        let global_auth = composer_home.join("auth.json");
        auths.extend(load_auth_json(&global_auth));
    }
    
    // Load local auth.json
    let local_auth = PathBuf::from("auth.json");
    auths.extend(load_auth_json(&local_auth));
    
    // Load from environment (highest priority)
    auths.extend(load_auth_from_env());
    
    auths
}

/// Save authentication to auth.json
pub fn save_auth_json(path: &PathBuf, auths: &HashMap<String, Authentication>) -> std::io::Result<()> {
    let mut json = serde_json::json!({});
    
    // Separate auths by type based on username patterns
    let mut http_basic = serde_json::Map::new();
    let mut github_oauth = serde_json::Map::new();
    let mut gitlab_oauth = serde_json::Map::new();
    let mut bearer = serde_json::Map::new();
    
    for (host, auth) in auths {
        match auth.username.as_str() {
            "x-oauth-basic" => {
                if let Some(ref token) = auth.password {
                    github_oauth.insert(host.clone(), serde_json::json!(token));
                }
            }
            "oauth2" => {
                if let Some(ref token) = auth.password {
                    gitlab_oauth.insert(host.clone(), serde_json::json!(token));
                }
            }
            "bearer" => {
                if let Some(ref token) = auth.password {
                    let actual_host = host.trim_end_matches("-bearer");
                    bearer.insert(actual_host.to_string(), serde_json::json!(token));
                }
            }
            _ => {
                http_basic.insert(host.clone(), serde_json::json!({
                    "username": auth.username,
                    "password": auth.password
                }));
            }
        }
    }
    
    if !http_basic.is_empty() {
        json["http-basic"] = serde_json::Value::Object(http_basic);
    }
    if !github_oauth.is_empty() {
        json["github-oauth"] = serde_json::Value::Object(github_oauth);
    }
    if !gitlab_oauth.is_empty() {
        json["gitlab-oauth"] = serde_json::Value::Object(gitlab_oauth);
    }
    if !bearer.is_empty() {
        json["bearer"] = serde_json::Value::Object(bearer);
    }
    
    let content = serde_json::to_string_pretty(&json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    std::fs::write(path, content)
}

