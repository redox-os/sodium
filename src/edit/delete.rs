use edit::buffer::TextBuffer;
use io::redraw::RedrawTask;
use state::cursor::Cursor;
use state::editor::Editor;

impl Editor {
    /// Delete a character.
    #[inline]
    pub fn delete(&mut self) {
        let &Cursor { x, y, .. } = self.cursor();
        if x == self.buffers.current_buffer()[y].len() {
            if y + 1 < self.buffers.current_buffer().len() {
                let s = self.buffers.current_buffer_mut().remove_line(y + 1);
                self.buffers.current_buffer_mut()[y].push_str(&s);
                self.redraw_task = RedrawTask::Lines(y..y + 1);
            }
        } else if x < self.buffers.current_buffer()[y].len() {
            self.buffers.current_buffer_mut()[y].remove(x);
            self.redraw_task = RedrawTask::LinesAfter(y);
        }

        self.hint();
    }

    /// Backspace.
    #[inline]
    pub fn backspace(&mut self) {
        let previous = self.previous(1);
        if let Some(p) = previous {
            self.goto(p);
            self.delete();
        } else {
            self.status_bar.msg = "Can't delete file start".to_owned();
        }
    }
}
