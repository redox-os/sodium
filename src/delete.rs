use editor::Editor;
use redraw::RedrawTask;
use buffer::Buffer;
use cursor::Cursor;

impl Editor {
    /// Delete a character
    #[inline]
    pub fn delete(&mut self) {
        let &Cursor{ x, y, ..} = self.cursor();
        if x == self.buffer[y].len() {
            if y < self.buffer.len() - 1 {
                let s = self.buffer.remove_line(y + 1);
                self.buffer[y].push_str(&s);
                self.redraw_task = RedrawTask::Lines(y..y+1);
            }
        } else if x < self.buffer[y].len() {
            self.buffer[y].remove(x);
            self.redraw_task = RedrawTask::LinesAfter(y);
        }

        self.hint();
    }
}
