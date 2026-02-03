//! composer-rs: An optimized Rust rewrite of PHP Composer
//!
//! This crate provides a complete implementation of Composer's functionality
//! including dependency resolution, package downloading, and installation.

pub mod composer;
pub mod composer_json;
pub mod composer_lock;
pub mod error;

// Re-export commonly used types
pub use composer::composer::Composer;
pub use composer_json::ComposerJson;
pub use composer_lock::ComposerLock;
pub use error::{ComposerError, Result};

/// Composer version constant
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Composer release date
pub const RELEASE_DATE: &str = "2026-02-03";

/// Composer branch alias
pub const BRANCH_ALIAS: &str = "2.x-dev";

