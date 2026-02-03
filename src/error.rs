//! Error types for composer-rs

use thiserror::Error;

/// Main error type for composer-rs operations
#[derive(Error, Debug)]
pub enum ComposerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Invalid composer.json: {0}")]
    InvalidComposerJson(String),

    #[error("Invalid composer.lock: {0}")]
    InvalidComposerLock(String),

    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("Version not found: {package} version {version}")]
    VersionNotFound { package: String, version: String },

    #[error("Version constraint parse error: {0}")]
    VersionConstraintError(String),

    #[error("Dependency resolution failed: {0}")]
    ResolutionFailed(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Installation failed: {0}")]
    InstallationFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("Archive extraction failed: {0}")]
    ArchiveError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Authentication required for {0}")]
    AuthenticationRequired(String),

    #[error("Checksum mismatch for {package}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        package: String,
        expected: String,
        actual: String,
    },

    #[error("Lock file out of sync with composer.json")]
    LockFileOutOfSync,

    #[error("Platform requirement not met: {0}")]
    PlatformRequirementNotMet(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Script execution failed: {0}")]
    ScriptError(String),

    #[error("{0}")]
    Custom(String),
}

/// Result type alias for composer-rs operations
pub type Result<T> = std::result::Result<T, ComposerError>;

impl ComposerError {
    /// Create a custom error with a message
    pub fn custom<S: Into<String>>(msg: S) -> Self {
        ComposerError::Custom(msg.into())
    }
}
