use editor::Editor;
use parse::{Inst, Parameter};
use key::Key;
use mode::{Mode, CommandMode, PrimitiveMode};
use insert::{InsertOptions, InsertMode};
use redraw::RedrawTask;
use buffer::{Buffer, Line, Slice};

use std::iter::FromIterator;

// TODO: Move the command definitions outta here
impl<'a, B: Buffer<'a>> Editor<B> {
    /// Execute an instruction
    pub fn exec<'b: 'a>(&'b mut self, Inst(para, cmd): Inst) {
        use key::Key::*;
        use mode::Mode::*;
        use mode::PrimitiveMode::*;
        use mode::CommandMode::*;

        let n = para.d();
        let bef = self.pos();
        let mut mov = false;

        if cmd.key == Key::Char(' ') && self.key_state.shift {

            let mode = self.cursor().mode;

            match mode {
                Primitive(Prompt) => self.prompt = String::new(),
                Primitive(Insert(_)) => {
                    let left = self.left(1);
                    self.goto(left);
                },
                _ => {},
            }
            self.cursor_mut().mode = Mode::Command(CommandMode::Normal);

        } else if self.key_state.alt && cmd.key == Key::Char(' ') {

            self.next_cursor();

        } else if self.key_state.alt {

            if let Some(m) = self.to_motion(Inst(para, cmd)) {
                self.goto(m);
            }

        } else {
            match self.cursor().mode {
                Command(Normal) => match cmd.key {
                    //                    // { for debuging
                    //                    Char('n') => {
                    //                        let p = self.next(n);
                    //                        if let Some(p) = p {
                    //                            self.goto(p);
                    //                        }
                    //                    },
                    //                    Char('p') => {
                    //                        let p = self.previous(n);
                    //                        if let Some(p) = p {
                    //                            self.goto(p);
                    //                        }
                    //                    },
                    //                    // }
                    Char('i') => {
                        self.cursor_mut().mode =
                            Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                                mode: InsertMode::Insert,
                            }));

                    },
                    Char('a') => {
                        let pos = self.right(1, false);
                        self.goto( pos );
                        self.cursor_mut().mode =
                            Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                                mode: InsertMode::Insert,
                            }));
                    },
                    Char('o') => {
                        let y  = self.y();
                        let ln = self.buffer.get_line(y);
                        let ind = if self.options.autoindent {
                            ln.get_indent()
                        } else {
                            <B::Line as Line>::Slice::new_empty()
                        };
                        let last = ind.len();
                        self.buffer.insert_line(y, ind);
                        self.goto((last, y + 1));
                        self.cursor_mut().mode =
                            Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                                mode: InsertMode::Insert,
                            }));
                    },
                    Char('h') => {
                        let left = self.left(n);
                        self.goto(left);
                        mov = true;
                    },
                    Char('j') => {
                        let down = self.down(n);
                        self.goto(down);
                        mov = true;
                    },
                    Char('k') => {
                        let up = self.up(n);
                        self.goto(up);
                        mov = true;
                    },
                    Char('l') => {
                        let right = self.right(n, true);
                        self.goto(right);
                        mov = true;
                    },
                    Char('J') => {
                        let down = self.down(15 * n);
                        self.goto(down);
                        mov = true;
                    },
                    Char('K') => {
                        let up = self.up(15 * n);
                        self.goto(up);
                        mov = true;
                    },
                    Char('x') => {
                        self.delete();
                        let bounded = self.bound(self.pos(), true);
                        self.goto(bounded);
                    },
                    Char('X') => {
                        self.backspace();
                        let bounded = self.bound(self.pos(), true);
                        self.goto(bounded);
                    },
                    Char('L') => {
                        let ln_end = (self.buffer.get_line(self.y()).len(), self.y());
                        self.goto(ln_end);
                        mov = true;
                    },
                    Char('H') => {
                        self.cursor_mut().x = 0;
                        mov = true;
                    },
                    Char('r') => {
                        let (x, y) = self.pos();
                        let c = self.get_char();
                        self.buffer.get_line_mut(y).set_char(x, c);
                    },
                    Char('R') => {
                        self.cursor_mut().mode =
                            Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                                mode: InsertMode::Replace,
                            }));
                    },
                    Char('d') => {
                        let ins = self.get_inst();
                        if let Some(m) = self.to_motion_unbounded(ins) {
                            self.remove_rb(m);
                        }
                    },
                    Char('G') => {
                        let last = self.buffer.len() - 1;
                        self.goto((0, last));
                        mov = true;
                    },
                    Char('g') => {
                        if let Parameter::Int(n) = para {
                            self.goto((0, n - 1));
                            mov = true;
                        } else {
                            let inst = self.get_inst();
                            if let Some(m) = self.to_motion(inst) {
                                self.goto(m); // fix
                                mov = true;
                            }
                        }

                    },
                    Char('b') => {
                        // Branch cursor
                        if self.cursors.len() < 255 {
                            let cursor = self.cursor().clone();
                            self.cursors.insert(self.current_cursor as usize, cursor);
                            self.next_cursor();
                        }
                        else {
                            self.status_bar.msg = format!("At max 255 cursors");
                        }
                    },
                    Char('B') => {
                        // Delete cursor
                        if self.cursors.len() > 1 {
                            self.cursors.remove(self.current_cursor as usize);
                            self.prev_cursor();
                        }
                        else {
                            self.status_bar.msg = format!("No other cursors!");
                        }
                    },
                    Char('t') => {
                        let ch = self.get_char();

                        let pos = self.next_ocur(ch, n);
                        if let Some(p) = pos {
                            let y = self.y();
                            self.goto((p, y));
                            mov = true;
                        }
                    },
                    Char('f') => {
                        let ch = self.get_char();

                        let pos = self.previous_ocur(ch, n);
                        if let Some(p) = pos {
                            let y = self.y();
                            self.goto((p, y));
                            mov = true;
                        }
                    },
                    Char(';') => {
                        self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Prompt);
                    },
                    Char(' ') => {
                        self.next_cursor();
                    },
                    Char('z') => {
                        let Inst(param, cmd) = self.get_inst();
                        match param {
                            Parameter::Null => {
                                if let Some(m) = self.to_motion(Inst(param, cmd)) {
                                    self.scroll_y = m.1;
                                    self.goto(m);
                                }
                            }
                            Parameter::Int(n) => {
                                self.scroll_y = n;
                            }
                        }
                        self.redraw_task = RedrawTask::Full;
                    },
                    Char('Z') => {
                        self.scroll_y = self.y() - 3;
                        self.redraw_task = RedrawTask::Full;
                    },
                    Char('~') => {
                        self.invert_chars(n);
                    },
                    Char(c) => {
                        self.status_bar.msg = format!("Unknown command: {}", c);
                        self.redraw_task = RedrawTask::StatusBar;
                    },
                    _ => {
                        self.status_bar.msg = format!("Unknown command");
                        self.redraw_task = RedrawTask::StatusBar;
                    },
                },
                Primitive(Insert(opt)) => {
                    self.insert(cmd.key, opt);
                },
                Primitive(Prompt) => {
                    match cmd.key {
                        Char('\n') => {
                            self.cursor_mut().mode = Command(Normal);
                            let cmd = self.prompt.clone();

                            self.invoke(cmd);
                            self.prompt = String::new();
                        },
                        Backspace => {
                            self.prompt.pop();
                        },
                        Char(c) => self.prompt.push(c),
                        _ => {},
                    }
                    self.redraw_task = RedrawTask::StatusBar;
                },
            }
        }
        if mov {
            self.redraw_task = RedrawTask::Cursor(bef, self.pos());
        }
    }
}
