use buffer::Buffer;
use cursor::Cursor;
use editor::Editor;
use redraw::RedrawTask;

impl Editor {
    /// Delete a character.
    #[inline]
    pub fn delete(&mut self) {
        let &Cursor{ x, y, .. } = self.cursor();
        if x == self.buffer[y].len() {
            if y + 1 < self.buffer.len() {
                let s = self.buffer.remove_line(y + 1);
                self.buffer[y].push_str(&s);
                self.redraw_task = RedrawTask::Lines(y..y + 1);
            }
        } else if x < self.buffer[y].len() {
            self.buffer[y].remove(x);
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
