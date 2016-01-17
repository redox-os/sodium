use editor::Editor;
use mode::{Mode, PrimitiveMode, CommandMode};
use redraw::RedrawTask;
use key::Key;
use buffer::{Buffer, Line};

use std::collections::VecDeque;
use std::iter::FromIterator;

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

impl<'a, B: Buffer<'a>> Editor<B> {
    /// Insert text under the current cursor.
    pub fn insert(&mut self, k: Key, InsertOptions { mode }: InsertOptions) {
        let (mut x, mut y) = self.pos();
        match mode {
            InsertMode::Insert => {
               match k {
                    Key::Char('\n') => {
                        let begin = self.buffer.insert_newline(x, y, self.options.autoindent);

                        self.redraw_task = RedrawTask::LinesAfter(y);
                        self.goto((begin, y + 1));
                    }
                    Key::Backspace => self.backspace(),
                    Key::Char(c) => {
                        self.buffer.get_line_mut(y).insert(x, c);

                        self.redraw_task = RedrawTask::Lines(y..y + 1);
                        let right = self.right(1, false);
                        self.goto(right);
                    }
                    _ => {}
                }
            }
            InsertMode::Replace => match k {
                Key::Char(c) => {
                    if x == self.buffer.get_line(y).len() {
                        let next = self.next(1);
                        if let Some(p) = next {
                            self.goto(p);
                            x = self.x();
                            y = self.y();
                        }
                    }

                    if self.buffer.len() != y {
                        if self.buffer.get_line(y).len() == x {
                            let next = self.next(1);
                            if let Some(p) = next {
                                self.goto(p);
                            }
                        } else {
                            self.buffer.get_line_mut(y).remove(x);
                            self.buffer.get_line_mut(y).insert(x, c);
                        }
                    }
                    let next = self.next(1);
                    if let Some(p) = next {
                        self.goto(p);
                    }
                    self.redraw_task = RedrawTask::Lines(y..y + 1);
                }
                _ => {}
            },
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
