/// The global editor state.
pub mod editor;
/// Cursors.
///
/// A cursor contains various non-global information about the editor state. You can switch between
/// cursor, for reusing older editor states.
pub mod cursor;
/// Options and configuration of the editor.
pub mod options;
/// Editor modes.
pub mod mode;
