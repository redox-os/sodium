use key::{Cmd, Key};
use editor::Editor;
use redraw::RedrawTask;
use mode::Mode;

#[cfg(feature = "orbital")]
use orbital::{EventOption, Event};

#[derive(Copy, Clone)]
/// An instruction, i.e. a command and a numeral parameter
pub struct Inst(pub Parameter, pub Cmd);

/// A numeral parameter, i.e. a number (or nothing) given before a command (toghether making an
/// instruction)
#[derive(Copy, Clone)]
pub enum Parameter {
    /// An integer as parameter
    Int(usize),
    /// Not given (the user have not defined any numeral parameter to this command)
    Null,
}
impl Parameter {
    /// Either unwrap the Int(n) to n or fallback to a given value
    #[inline]
    pub fn or(self, fallback: usize) -> usize {
        if let Parameter::Int(n) = self {
            n
        } else {
            fallback
        }
    }
    /// Fallback to one (default)
    #[inline]
    pub fn d(self) -> usize {
        self.or(1)
    }
}

impl Editor {
    /// Get the next character input. Useful for commands taking a character as post-parameter,
    /// such as r (replace).
    pub fn get_char(&mut self) -> char {
        #[cfg(feature = "orbital")]
        loop {
            match self.window
                      .poll()
                      .unwrap_or(Event::new())
                      .to_option() {
                EventOption::Key(k) => {
                    if let Some(Key::Char(c)) = self.key_state.feed(k) {
                        self.status_bar.cmd.push(c);
                        self.redraw_task = RedrawTask::StatusBar;
                        return c;
                    }
                }
                _ => {}
            }
        }
    }

    /// Get the next instruction, i.e. the next input of a command together with a numeral
    /// parameter.
    pub fn get_inst(&mut self) -> Inst {
        let mut n = 0;
        let mut unset = true;

        let mut key = Key::Null;
        self.status_bar.cmd = String::new();

        // self.status_bar.cmd = String::new();
        #[cfg(feature = "orbital")]
        loop {
            match self.window
                      .poll()
                      .unwrap_or(Event::new())
                      .to_option() {
                EventOption::Key(key_event) => {
                    if let Some(k) = self.key_state.feed(key_event) {
                        let c = k.to_char();
                        self.status_bar.cmd.push(c);
                        self.redraw_status_bar();

                        match self.cursor().mode {
                            Mode::Primitive(_) => {
                                key = k;
                            }
                            Mode::Command(_) => {
                                n = match c {
                                    '0'...'9' => {
                                        unset = false;
                                        n * 10 + ((c as u8) - b'0') as usize
                                    }
                                    _ => {

                                        key = k;
                                        n
                                    }
                                };
                            }

                        }
                    }
                    match key {
                        Key::Null => {}
                        _ => {
                            return Inst(if unset {
                                            Parameter::Null
                                        } else {
                                            Parameter::Int(n)
                                        },
                                        Cmd { key: key });
                        }
                    }
                }
                EventOption::Quit(_) => {
                    return Inst(Parameter::Null, Cmd { key: Key::Quit });
                }
                _ => {}
            }
        }

    }
}
