use editor::Editor;
use redraw::RedrawTask;

impl Editor {
    /// Delete a character
    #[inline]
    pub fn delete(&mut self) {
        let (x, y) = self.pos();
        if self.buffer[y].is_empty() {
            if self.buffer.len() != 1 {
                self.buffer.remove_line(y);
                self.redraw_task = RedrawTask::Lines(y..y + 1);
            }
        } else if x < self.buffer[y].len() {
            self.buffer[y].remove(x);
            self.redraw_task = RedrawTask::LinesAfter(y);
        }
    }
}
