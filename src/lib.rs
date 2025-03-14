//! Rust Multiplatform Framework
//!
//! This crate provides utilities for building cross-platform applications with Rust,
//! focusing on simplifying the FFI layer and eliminating boilerplate.

// Re-export uniffi for convenience in user crates
pub use uniffi;

// Internal modules
mod macros;
#[cfg(test)]
mod tests;
mod utils;

// Public exports
pub use utils::{create_app_builder, create_model_update_channel, listen_for_model_updates};

// Note: Macros exported with #[macro_export] are automatically available at the crate root
// and don't need to be re-exported

/// Traits that can be implemented by app models to integrate with the framework
pub mod traits;

// Re-export key traits for convenience
pub use traits::{AppBuilder, BuildableApp, RmpAppModel, RmpViewModel};

// Re-export frequently used types to make it easier for app developers
pub use crossbeam;
pub use once_cell;
