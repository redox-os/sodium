use editor::Editor;
use mode::{Mode, PrimitiveMode, CommandMode};
use redraw::RedrawTask;
use key::Key;
use buffer::Buffer;

use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Clone, PartialEq, Copy)]
/// The type of the insert mode
pub enum InsertMode {
    /// Append text (after the cursor)
    Append,
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
    /// Delta x, i.e. the cursors visual position's x coordinate relative to the cursors actual
    /// position. For example append will append character after the cursor, but visually it have
    /// delta x = 1, so it will look like normal insert mode, except when going back to normal
    /// mode, the cursor will move back (visually), because the delta x will drop to 0.
    pub fn delta(&self) -> usize {
        let (x, y) = self.pos();
        match self.cursor().mode {
            _ if x > self.buffer[y].len() => 0,
            Mode::Primitive(PrimitiveMode::Insert(InsertOptions { mode: InsertMode::Append }))
                if x == self.buffer[y].len() => 0,

            Mode::Primitive(PrimitiveMode::Insert(InsertOptions { mode: InsertMode::Append })) => 1,
            _ => 0,
        }
    }

    /// Insert text under the current cursor.
    pub fn insert(&mut self, k: Key, InsertOptions { mode }: InsertOptions) {
        let (mut x, mut y) = self.pos();
        match mode {
            InsertMode::Insert | InsertMode::Append => {
                let d = self.delta();

                match k {
                    Key::Char('\n') => {
                        let first_part = self.buffer[y][..x + d].to_owned();
                        let second_part = self.buffer[y][x + d..].to_owned();

                        self.buffer[y] = first_part;

                        let nl = if self.options.autoindent {
                            self.buffer.get_indent(y).to_owned()
                        } else {
                            String::new()
                        };
                        let begin = nl.len();

                        self.buffer.insert_line(y, nl + &second_part);

                        self.redraw_task = RedrawTask::LinesAfter(y);
                        self.goto((begin, y + 1));
                    }
                    Key::Backspace => {
                        // NEEDS REWRITE {
                        // Backspace
                        let del = if self.buffer[y].len() == 0 {
                            1
                        } else if d == 0 && x == 0 {
                            self.cursor_mut().mode =
                                Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                                    mode: InsertMode::Append,
                                }));
                            1

                        } else {
                            1 - d
                        };

                        let prev = self.previous(del);
                        if let Some((x, y)) = prev {
                            // if self.x() != 0 || self.y() != 0 {
                            self.goto((x + d, y));
                            self.delete();
                            // }
                        }

                        // }

                    }
                    Key::Char(c) => {
                        self.buffer[y].insert(x + d, c);

                        // TODO: Are there a better way than switching?
                        match mode {
                            InsertMode::Insert if x + 1 == self.buffer[y].len() => {
                                self.cursor_mut().mode =
                                    Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                                        mode: InsertMode::Append,
                                    }));
                            }
                            _ => {}
                        }

                        self.redraw_task = RedrawTask::Lines(y..y + 1);
                        let right = self.right(1);
                        self.goto(right);
                    }
                    _ => {}
                }
            }
            InsertMode::Replace => {
                match k {
                    Key::Char(c) => {
                        if x == self.buffer[y].len() {
                            let next = self.next(1);
                            if let Some(p) = next {
                                self.goto(p);
                                x = self.x();
                                y = self.y();
                            }
                        }

                        if self.buffer.len() != y {
                            if self.buffer[y].len() == x {
                                let next = self.next(1);
                                if let Some(p) = next {
                                    self.goto(p);
                                }
                            } else {
                                self.buffer[y].remove(x);
                                self.buffer[y].insert(x, c);
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
            }
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
