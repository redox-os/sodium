use edit::buffer::{SplitBuffer, TextBuffer};
use io::file::FileStatus;
use io::redraw::RedrawTask;
use state::editor::{Buffer, BufferManager, Editor};

use std::process::exit;

/// Prompt mode commands.
pub enum PromptCommand<'a> {
    /// Set an option.
    Set {
        /// The option to set.
        option: &'a str,
    },
    /// Unset an option.
    Unset {
        /// The option to unset.
        option: &'a str,
    },
    /// Toggle an option.
    Toggle {
        /// The option to toggle.
        option: &'a str,
    },
    /// Get whether an option is set or not.
    Get {
        /// The option to get.
        option: &'a str,
    },
    /// Open the specified file in a new buffer.
    Open {
        /// The path to open.
        path: &'a str,
    },
    /// Write the current buffer to the specified path.
    Write {
        /// The path to write to.
        path: &'a str,
    },
    /// List the available buffers.
    ListBuffers,
    /// Create a new empty buffer.
    CreateBuffer,
    /// Delete the current buffer.
    DeleteBuffer,
    /// Switch to the nth buffer.
    SwitchToBuffer {
        /// The index of the buffer to switch to.
        buffer_index: usize,
    },
    /// Display help in a new buffer.
    Help,
    /// Exit Sodium.
    Quit,
}

impl<'a> PromptCommand<'a> {
    /// Parse a string to get a PromptCommand. If the parse fails,
    /// None is returned.
    pub fn parse(s: &'a str) -> Option<PromptCommand<'a>> {
        use self::PromptCommand::*;

        let mut split = s.split(' ');
        let base_cmd = split.nth(0).unwrap_or("");
        let sec_cmd = split.nth(0).unwrap_or("");

        Some(match base_cmd {
            "set" => Set { option: sec_cmd },
            "unset" => Unset { option: sec_cmd },
            "toggle" | "tog" => Toggle { option: sec_cmd },
            "get" => Get { option: sec_cmd },
            "o" | "open" => Open { path: sec_cmd },
            "w" | "write" => Write { path: sec_cmd },
            "ls" => ListBuffers,
            "bn" => CreateBuffer,
            "bd" => DeleteBuffer,
            "h" | "help" => Help,
            "q" | "quit" => Quit,
            bn if bn.starts_with("b") => {
                let rest: String = bn.chars().skip(1).collect();

                if let Ok(number) = rest.parse::<usize>() {
                    SwitchToBuffer {
                        buffer_index: number,
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        })
    }
}

impl Editor {
    /// Invoke a command in the prompt
    pub fn invoke<'a>(&mut self, cmd: PromptCommand<'a>) {
        use self::PromptCommand::*;

        match cmd {
            Set { option } => {
                self.status_bar.msg = match self.options.set(option) {
                    Ok(()) => format!("Option set: {}", option),
                    Err(()) => format!("Option does not exist: {}", option),
                }
            }
            Unset { option } => {
                self.status_bar.msg = match self.options.unset(option) {
                    Ok(()) => format!("Option unset: {}", option),
                    Err(()) => format!("Option does not exist: {}", option),
                }
            }
            Toggle { option } => {
                self.status_bar.msg = match self.options.toggle(option) {
                    Ok(()) => format!("Option toggled: {}", option),
                    Err(()) => format!("Option does not exist: {}", option),
                }
            }
            Get { option } => {
                self.status_bar.msg = match self.options.get(option) {
                    Some(true) => format!("Option set: {}", option),
                    Some(false) => format!("Option unset: {}", option),
                    None => format!("Option does not exist: {}", option),
                }
            }
            Open { path } => {
                let line = self.buffers.current_buffer().get_line(0).unwrap().len();
                let empty = self.buffers.current_buffer().len() == 1 && line == 0;
                let ix = self.buffers.current_buffer_index();
                self.status_bar.msg = match self.open(path) {
                    FileStatus::NotFound => format!("File {} could not be opened", path),
                    FileStatus::Ok => format!("File {} opened", path),
                    _ => unreachable!(),
                };
                if empty {
                    self.buffers.delete_buffer(ix);
                }
            }
            Write { path } => {
                if self.options.get("readonly") == Some(true) {
                    // TODO: add override (w!)
                    self.status_bar.msg = format!("File {} is opened in readonly mode", path)
                } else {
                    self.status_bar.msg = match self.write(path) {
                        FileStatus::NotFound => format!("File {} could not be opened", path),
                        FileStatus::Ok => format!("File {} written", path),
                        FileStatus::Other => format!("Couldn't write {}", path),
                    }
                }
            }
            ListBuffers => {
                let description = get_buffers_description(&self.buffers);
                let mut new_buffer: Buffer = SplitBuffer::from_str(&description).into();
                new_buffer.title = Some("<Buffers>".into());
                new_buffer.is_transient = true; // delete the buffer when the user switches away

                let new_buffer_index = self.buffers.new_buffer(new_buffer);
                self.buffers.switch_to(new_buffer_index);
                self.redraw_task = RedrawTask::Full;
            }
            SwitchToBuffer { buffer_index: ix } => {
                if !self.buffers.is_buffer_index_valid(ix) {
                    self.status_bar.msg = format!("Invalid buffer #{}", ix);
                } else if self.buffers.current_buffer_index() == ix {
                    self.status_bar.msg = format!("Already in buffer #{}", ix);
                } else {
                    self.buffers.switch_to(ix);
                    self.redraw_task = RedrawTask::Full;
                    self.status_bar.msg = format!("Switched to buffer #{}", ix);
                }
            }
            CreateBuffer => {
                let new = self.buffers.new_buffer(Buffer::new());
                self.buffers.switch_to(new);
                self.redraw_task = RedrawTask::Full;
                self.status_bar.msg = format!("Switched to buffer#{}", new);
            }
            DeleteBuffer => {
                let ix = self.buffers.current_buffer_index();
                self.buffers.delete_buffer(ix);
                self.redraw_task = RedrawTask::Full;
            }
            Help => {
                self.open("/apps/sodium/help.txt");
            }
            Quit => {
                exit(0);
            }
        }

        self.hint();
    }
}

fn get_buffers_description(buffers: &BufferManager) -> String {
    fn print_buffer(i: usize, b: &Buffer) -> String {
        let title = b.title.as_ref().map(|s| s.as_str()).unwrap_or("<No Title>");

        format!("b{}\t\t\t{}", i, title)
    }

    let descriptions = buffers
        .iter()
        // don't include transient buffers like the one
        // this is going to be shown in
        .filter(|b| !b.is_transient)
        .enumerate()
        .map(|(i, b)| print_buffer(i, b))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Buffers\n=====================================\n\n{}",
        descriptions
    )
}
