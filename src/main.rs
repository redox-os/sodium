#![feature(stmt_expr_attributes)]
#![deny(warnings)]
#![deny(missing_docs)]

#[cfg(feature = "orbital")]
extern crate orbclient;

#[macro_use]
pub mod debug;

pub mod buffer;
pub mod cursor;
pub mod delete;
pub mod editor;
pub mod exec;
pub mod file;
pub mod graphics;
pub mod insert;
pub mod invert;
pub mod key;
pub mod key_state;
pub mod mode;
pub mod motion;
pub mod movement;
pub mod options;
pub mod parse;
pub mod position;
pub mod prompt;
pub mod redraw;
pub mod selection;

fn main() {
    editor::Editor::init();
}
