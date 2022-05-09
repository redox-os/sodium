use edit::buffer::TextBuffer;
use io::key::Key;
use io::redraw::RedrawTask;
use state::editor::Editor;

#[derive(Clone, PartialEq, Copy)]
/// The type of the insert mode
pub enum InsertMode {
    /// Insert text (before the cursor)
    Insert,
    /// Replace text (on the cursor)
    Replace,
}

#[derive(Clone, PartialEq, Copy)]
/// The insert options
pub struct InsertOptions {
    /// The mode type
    pub mode: InsertMode,
}

impl Editor {
    /// Insert text under the current cursor.
    pub fn insert(&mut self, k: Key, InsertOptions { mode }: InsertOptions) {
        let (mut x, mut y) = self.pos();
        match (mode, k) {
            (InsertMode::Insert, Key::Char('\n')) => {
                let first_part = self.buffers.current_buffer()[y][..x].to_owned();
                let second_part = self.buffers.current_buffer()[y][x..].to_owned();

                self.buffers.current_buffer_mut()[y] = first_part;

                let nl = if self.options.autoindent {
                    self.buffers.current_buffer().get_indent(y).to_owned()
                } else {
                    String::new()
                };
                let begin = nl.len();

                self.buffers
                    .current_buffer_mut()
                    .insert_line(y + 1, nl + &second_part);

                self.redraw_task = RedrawTask::LinesAfter(y);
                self.goto((begin, y + 1));
            }
            (InsertMode::Insert, Key::Backspace) => self.backspace(),
            (InsertMode::Insert, Key::Tab) => {
                for i in 0..4 {
                    self.buffers.current_buffer_mut()[y].insert(x + i, ' ');
                }
                self.redraw_task = RedrawTask::Lines(y..y + 1);
                let right = self.right(4, false);
                self.goto(right);
            }
            (InsertMode::Insert, Key::Char(c)) => {
                self.buffers.current_buffer_mut()[y].insert(x, c);

                self.redraw_task = RedrawTask::Lines(y..y + 1);
                let right = self.right(1, false);
                self.goto(right);
            }
            (InsertMode::Replace, Key::Char(c)) => {
                if x == self.buffers.current_buffer()[y].len() {
                    let next = self.next(1);
                    if let Some(p) = next {
                        self.goto(p);
                        x = self.x();
                        y = self.y();
                    }
                }

                if self.buffers.current_buffer_mut().len() != y {
                    if self.buffers.current_buffer()[y].len() == x {
                        let next = self.next(1);
                        if let Some(p) = next {
                            self.goto(p);
                        }
                    } else {
                        self.buffers.current_buffer_mut()[y].remove(x);
                        self.buffers.current_buffer_mut()[y].insert(x, c);
                    }
                }
                let next = self.next(1);
                if let Some(p) = next {
                    self.goto(p);
                }
                self.redraw_task = RedrawTask::Lines(y..y + 1);
            }
            _ => {}
        }

        self.hint();
    }

    /// Insert a string
    pub fn insert_str(&mut self, txt: String, opt: InsertOptions) {
        for c in txt.chars() {
            self.insert(Key::Char(c), opt);
        }
    }
}
