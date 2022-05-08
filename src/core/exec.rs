use core::prompt::PromptCommand;
use edit::buffer::TextBuffer;
use edit::insert::{InsertMode, InsertOptions};
use io::parse::{Inst, Parameter};
use io::redraw::RedrawTask;
use state::editor::Editor;
use state::mode::{CommandMode, Mode, PrimitiveMode};

// TODO: Move the command definitions outta here
impl Editor {
    /// Execute an instruction
    pub fn exec(&mut self, Inst(para, cmd): Inst) {
        use io::key::Key::*;
        use state::mode::CommandMode::*;
        use state::mode::Mode::*;
        use state::mode::PrimitiveMode::*;

        let n = para.d();
        let bef = self.pos();
        let mut mov = false;

        match (self.cursor().mode, cmd.key) {
            (Primitive(Prompt), Char(' ')) if self.key_state.shift => {
                self.prompt.insert(0, String::new());
                self.cursor_mut().mode = Mode::Command(CommandMode::Normal);
            }
            (Primitive(Insert(_)), Escape) => {
                let left = self.left(1);
                self.goto(left);
                self.cursor_mut().mode = Mode::Command(CommandMode::Normal);
            }
            (Primitive(Insert(_)), Char(' ')) if self.key_state.shift => {
                let left = self.left(1);
                self.goto(left);
                self.cursor_mut().mode = Mode::Command(CommandMode::Normal);
            }
            (_, Char(' ')) if self.key_state.shift => {
                self.cursor_mut().mode = Mode::Command(CommandMode::Normal)
            }
            (_, Char(' ')) if self.key_state.alt => self.next_cursor(),
            _ if self.key_state.alt => {
                if let Some(m) = self.to_motion(Inst(para, cmd)) {
                    self.goto(m);
                }
            }
            (Command(Normal), Char('i')) => {
                self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                    mode: InsertMode::Insert,
                }));
            }
            (Command(Normal), Char('I')) => {
                self.cursor_mut().x = 0;
                self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                    mode: InsertMode::Insert,
                }));
            }
            (Command(Normal), Char('a')) => {
                let pos = self.right(1, false);
                self.goto(pos);
                self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                    mode: InsertMode::Insert,
                }));
            }
            (Command(Normal), Char('A')) => {
                let pos = (self.buffers.current_buffer()[self.y()].len(), self.y());
                //let pos = self.right(1, false);
                self.goto(pos);
                self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                    mode: InsertMode::Insert,
                }));
            }
            (Command(Normal), Char('o')) => {
                let y = self.y();
                let ind = if self.options.autoindent {
                    self.buffers.current_buffer().get_indent(y).to_owned()
                } else {
                    String::new()
                };
                let last = ind.len();
                self.buffers
                    .current_buffer_mut()
                    .insert_line(y + 1, ind.into());
                self.goto((last, y + 1));
                self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                    mode: InsertMode::Insert,
                }));
            }
            (Command(Normal), Left) | (Command(Normal), Char('h')) => {
                let left = self.left(n);
                self.goto(left);
                mov = true;
            }
            (Command(Normal), Down) | (Command(Normal), Char('j')) => {
                let down = self.down(n);
                self.goto(down);
                mov = true;
            }
            (Command(Normal), Up) | (Command(Normal), Char('k')) => {
                let up = self.up(n);
                self.goto(up);
                mov = true;
            }
            (Command(Normal), Right) | (Command(Normal), Char('l')) => {
                let right = self.right(n, true);
                self.goto(right);
                mov = true;
            }
            (Command(Normal), Char('w')) => {
                let next_word = self.next_word(n, true);
                self.goto(next_word);
                mov = true;
            }
            (Command(Normal), Char('e')) => {
                let next_word = self.next_word_end(n, true);
                self.goto(next_word);
                mov = true;
            }
            (Command(Normal), Char('J')) => {
                let down = self.down(15 * n);
                self.goto(down);
                mov = true;
            }
            (Command(Normal), Char('K')) => {
                let up = self.up(15 * n);
                self.goto(up);
                mov = true;
            }
            (Command(Normal), Char('x')) => {
                self.delete();
                let bounded = self.bound(self.pos(), true);
                self.goto(bounded);
            }
            (Command(Normal), Char('X')) => {
                self.backspace();
                let bounded = self.bound(self.pos(), true);
                self.goto(bounded);
            }
            (Command(Normal), Char('L')) => {
                if self.buffers.current_buffer()[self.y()].len() != 0 {
                    let ln_end = (self.buffers.current_buffer()[self.y()].len() - 1, self.y());
                    self.goto(ln_end);
                    mov = true;
                }
            }
            (Command(Normal), Char('$')) => {
                if self.buffers.current_buffer()[self.y()].len() != 0 {
                    let ln_end = (self.buffers.current_buffer()[self.y()].len() - 1, self.y());
                    self.goto(ln_end);
                    mov = true;
                }
            }
            (Command(Normal), Char('H')) => {
                println!("H pressed");
                self.cursor_mut().x = 0;
                mov = true;
            }
            (Command(Normal), Char('0')) => {
                println!("0 pressed");
                self.cursor_mut().x = 0;
                mov = true;
            }
            (Command(Normal), Char('r')) => {
                let (x, y) = self.pos();
                let c = self.get_char();
                let current_buffer = self.buffers.current_buffer_info_mut();
                // If there is nothing in the current buffer
                // ignore the command
                if current_buffer.raw_buffer[y].len() > 0 {
                    current_buffer.raw_buffer[y].remove(x);
                }
                current_buffer.raw_buffer[y].insert(x, c);
            }
            (Command(Normal), Char('R')) => {
                self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                    mode: InsertMode::Replace,
                }));
            }
            (Command(Normal), Char('d')) => {
                let ins = self.get_inst();
                if let Some(m) = self.to_motion_unbounded(ins) {
                    self.remove_rb(m);
                }
            }
            (Command(Normal), Char('c')) => {
                let ins = self.get_inst();
                if let Some(m) = self.to_motion_unbounded(ins) {
                    self.remove_rb(m);
                    self.cursor_mut().mode =
                        Mode::Primitive(PrimitiveMode::Insert(InsertOptions {
                            mode: InsertMode::Insert,
                        }));
                }
            }
            (Command(Normal), Char('G')) => {
                let last = self.buffers.current_buffer().len() - 1;
                self.goto((0, last));
                mov = true;
            }
            (Command(Normal), Char('g')) => {
                if let Parameter::Int(n) = para {
                    self.goto((0, n.wrapping_sub(1)));
                    mov = true;
                } else {
                    let inst = self.get_inst();
                    if let Some(m) = self.to_motion(inst) {
                        self.goto(m); // fix
                        mov = true;
                    }
                }
            }
            (Command(Normal), Char('b')) => {
                // Branch cursor
                if self.buffers.current_buffer_info().cursors.len() < 255 {
                    let cursor = self.cursor().clone();
                    let current_cursor_index =
                        self.buffers.current_buffer_info().current_cursor as usize;
                    self.buffers
                        .current_buffer_info_mut()
                        .cursors
                        .insert(current_cursor_index, cursor);
                    self.next_cursor();
                } else {
                    self.status_bar.msg = format!("At max 255 cursors");
                }
            }
            (Command(Normal), Char('B')) => {
                // Delete cursor
                if self.buffers.current_buffer_info().cursors.len() > 1 {
                    let current_cursor_index = self.buffers.current_buffer_info().current_cursor;
                    self.buffers
                        .current_buffer_info_mut()
                        .cursors
                        .remove(current_cursor_index as usize);
                    self.prev_cursor();
                } else {
                    self.status_bar.msg = format!("No other cursors!");
                }
            }
            (Command(Normal), Char('t')) => {
                let ch = self.get_char();

                let pos = self.next_ocur(ch, n);
                if let Some(p) = pos {
                    let y = self.y();
                    self.goto((p, y));
                    mov = true;
                }
            }
            (Command(Normal), Char('f')) => {
                let ch = self.get_char();

                let pos = self.previous_ocur(ch, n);
                if let Some(p) = pos {
                    let y = self.y();
                    self.goto((p, y));
                    mov = true;
                }
            }
            // (Command(Normal), Char('W')) => {
            //     let pos = self.next_word_forward(n);
            //     if let Some(p) = pos {
            //         let y = self.y();
            //         self.goto((p, y));
            //         mov = true;
            //     }
            // }
            (Command(Normal), Char(';')) => {
                self.cursor_mut().mode = Mode::Primitive(PrimitiveMode::Prompt)
            }
            (Command(Normal), Char(' ')) => self.next_cursor(),
            (Command(Normal), Char('z')) => {
                let Inst(param, cmd) = self.get_inst();
                match param {
                    Parameter::Null => {
                        if let Some(m) = self.to_motion(Inst(param, cmd)) {
                            self.buffers.current_buffer_info_mut().scroll_y = m.1;
                            self.goto(m);
                        }
                    }
                    Parameter::Int(n) => {
                        self.buffers.current_buffer_info_mut().scroll_y = n;
                    }
                }
                self.redraw_task = RedrawTask::Full;
            }
            (Command(Normal), Char('Z')) => {
                self.buffers.current_buffer_info_mut().scroll_y = self.y() - 3;
                self.redraw_task = RedrawTask::Full;
            }
            (Command(Normal), Char('~')) => {
                self.invert_chars(n);
            }
            (Command(Normal), Char('.')) => {
                if let Some(inst) = self.previous_instruction {
                    self.exec(inst);
                } else {
                    self.status_bar.msg = "No previous command".into();
                    self.redraw_task = RedrawTask::StatusBar;
                }
            }
            (Command(Normal), Char(c)) => {
                self.status_bar.msg = format!("Unknown command: {}", c);
                self.redraw_task = RedrawTask::StatusBar;
            }
            (Primitive(Insert(opt)), k) => self.insert(k, opt),
            (Primitive(Prompt), Char('\n')) => {
                self.cursor_mut().mode = Command(Normal);
                if let Some(cmd) = PromptCommand::parse(&self.prompt[self.prompt_index].clone()) {
                    self.invoke(cmd);
                    self.redraw_task = RedrawTask::StatusBar;
                } else {
                    self.status_bar.msg =
                        format!("Unknown command: {}", self.prompt[self.prompt_index]);
                }

                // If we use a command we used before, don't add a new line to the vec
                let cmd = self.prompt[self.prompt_index].clone();
                if self.prompt_index != 0 {
                    self.prompt[0] = cmd;
                }
                // Don't insert anything if the user didn't write anything
                if self.prompt[self.prompt_index] != "" {
                    self.prompt.insert(0, String::new());
                }
                self.prompt_index = 0;
            }
            (Primitive(Prompt), Backspace) => {
                self.prompt[self.prompt_index].pop();
                self.redraw_task = RedrawTask::StatusBar;
            }
            (Primitive(Prompt), Up) => {
                if self.prompt_index < self.prompt.len() - 1 {
                    self.prompt_index += 1;
                }
                self.redraw_task = RedrawTask::StatusBar;
            }
            (Primitive(Prompt), Down) => {
                if self.prompt_index > 0 {
                    self.prompt_index -= 1;
                }
                self.redraw_task = RedrawTask::StatusBar;
            }
            (Primitive(Prompt), Char(c)) => {
                self.prompt[self.prompt_index].push(c);
                self.redraw_task = RedrawTask::StatusBar;
            }
            _ => {
                self.status_bar.msg = format!("Unknown command");
                self.redraw_task = RedrawTask::StatusBar;
            }
        }
        if mov {
            self.redraw_task = RedrawTask::Cursor(bef, self.pos());
        }

        if !(self.cursor().mode == Command(Normal) && cmd.key == Char('.')) {
            self.previous_instruction = Some(Inst(para, cmd));
        }
    }
}
