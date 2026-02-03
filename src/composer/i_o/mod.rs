//! IO module - Input/Output handling for Composer operations
//!
//! This module provides different IO implementations:
//! - `ConsoleIO` - Interactive terminal IO with colors and progress bars
//! - `BufferIO` - Captures output for testing
//! - `NullIO` - Discards all output for silent operations

pub mod base_i_o;
pub mod buffer_i_o;
pub mod console_i_o;
pub mod i_o_interface;
pub mod null_i_o;

// Re-export commonly used types
pub use buffer_i_o::BufferIO;
pub use console_i_o::ConsoleIO;
pub use i_o_interface::{Authentication, IOInterface, Verbosity};
pub use null_i_o::NullIO;
