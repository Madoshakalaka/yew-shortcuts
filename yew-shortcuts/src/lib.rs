//! Productivity macros and components for Yew applications
//! 
//! This crate provides shortcuts and utilities to make Yew development more ergonomic.

// Re-export macros from the proc-macro crate
pub use yew_shortcuts_macros::{cs, comp};

// FontAwesome module
pub mod fontawesome;

// Component module
mod component;

// Re-export FontAwesomeSvg component
pub use component::FontAwesomeSvg;