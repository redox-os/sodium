//! Sodium is a next generation Vi-like editor.

#![feature(stmt_expr_attributes)]
#![deny(warnings)]
#![deny(missing_docs)]

#[cfg(feature = "orbital")]
extern crate orbclient;

/// Primitives for debugging.
#[macro_use]
pub mod debug;
/// The text buffer.
pub mod buffer;
/// Cursors.
///
/// A cursor contains various non-global information about the editor state. You can switch between
/// cursor, for reusing older editor states.
pub mod cursor;
/// Delete text, defined by a motion.
pub mod delete;
/// The global editor state.
pub mod editor;
/// Executing commands.
pub mod exec;
/// Loading and writing files.
pub mod file;
/// Graphics and rendering.
pub mod graphics;
/// Insertion of text.
pub mod insert;
/// "Invertion" of text.
pub mod invert;
/// Key input and parsing.
pub mod key;
/// The "key state" of the editor.
///
/// The key state contains information about the current state of modifiers.
pub mod key_state;
/// Editor modes.
pub mod mode;
/// Motions.
///
/// A motion is a command defining some movement from point A to point B, these can be used in
/// mulitple context, for example as argument for other commands.
pub mod motion;
/// Movement.
pub mod movement;
/// Options and configuration of the editor.
pub mod options;
/// Parsing of input commands.
pub mod parse;
/// Calculations and bounding of positions.
pub mod position;
/// The command prompt.
pub mod prompt;
/// Partial redraws.
pub mod redraw;
/// Selection through motions.
pub mod selection;

fn main() {
    editor::Editor::init();
}
