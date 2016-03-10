use std::ops::Range;

#[derive(Clone)]
/// A task for the renderer for redrawing
pub enum RedrawTask {
    /// None redraw task.
    None,
    /// Redraw a range of lines.
    Lines(Range<usize>),
    /// Redraw the lines after a given line.
    LinesAfter(usize),
    /// Full screen redraw.
    Full,
    /// Status bar redraw.
    StatusBar,
    /// Move cursor.
    Cursor((usize, usize), (usize, usize)),
}
