//! Sodium is a next generation Vi-like editor.

#![feature(stmt_expr_attributes)]
#![deny(warnings)]
#![deny(missing_docs)]

#[cfg(feature = "orbital")]
extern crate orbclient;

/// Core functionality.
#[macro_use]
pub mod core;
/// Carret primitives.
pub mod caret;
/// Editing.
pub mod edit;
/// Input/Output
pub mod io;
/// State of the editor.
pub mod state;


fn main() {
    self::state::editor::Editor::init();
}
