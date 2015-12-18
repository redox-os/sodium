use editor::Editor;
use redraw::RedrawTask;

impl Editor {
    /// Delete a character
    #[inline]
    pub fn delete(&mut self) {
        let (x, y) = self.pos();
        if x == 0 {
            if y != 0 {
                let s = self.buffer.remove_line(y);
                self.buffer[y - 1].push_str(&s);
                let len = self.buffer[y - 1].len();
                self.goto((len, y - 1));
                self.redraw_task = RedrawTask::Lines(y..y + 1);
            }
        } else if x < self.buffer[y].len() {
            self.buffer[y].remove(x);
            self.redraw_task = RedrawTask::LinesAfter(y);
        }
    }
}
