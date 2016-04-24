use io::file::FileStatus;
use io::redraw::RedrawTask;
use state::editor::Editor;
use edit::buffer::{Buffer, SplitBuffer};

use std::process::exit;

enum BufferCommand {
    SwitchToBuffer(usize),
}

fn try_get_buffer_command(c: &str) -> Option<BufferCommand> {
    if !c.starts_with("b") {
        return None;
    }

    let rest : String = c.chars().skip(1).collect();

    //TODO more buffer commands

    if let Ok(number) = rest.parse::<usize>() {
        Some(BufferCommand::SwitchToBuffer(number))
    } else {
        None
    }
}

impl Editor {
    /// Invoke a command in the prompt
    pub fn invoke(&mut self, cmd: String) {
        let mut split = cmd.split(' ');
        let base_cmd = split.nth(0).unwrap_or("");
        let sec_cmd = split.nth(0).unwrap_or("");

        match base_cmd {
            "set" => {
                self.status_bar.msg = match self.options.set(sec_cmd) {
                    Ok(()) => format!("Option set: {}", sec_cmd),
                    Err(()) => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "unset" => {
                self.status_bar.msg = match self.options.unset(sec_cmd) {
                    Ok(()) => format!("Option unset: {}", sec_cmd),
                    Err(()) => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "toggle" | "tog" => {
                self.status_bar.msg = match self.options.toggle(sec_cmd) {
                    Ok(()) => format!("Option toggled: {}", sec_cmd),
                    Err(()) => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "get" => {
                self.status_bar.msg = match self.options.get(sec_cmd) {
                    Some(true) => format!("Option set: {}", sec_cmd),
                    Some(false) => format!("Option unset: {}", sec_cmd),
                    None => format!("Option does not exist: {}", sec_cmd),
                }
            },
            "o" | "open" => {
                self.status_bar.msg = match self.open(sec_cmd) {
                    FileStatus::NotFound => format!("File {} could not be opened", sec_cmd),
                    FileStatus::Ok => format!("File {} opened", sec_cmd),
                    _ => unreachable!(),
                }
            },
            "w" | "write" => {
                self.status_bar.msg = match self.write(sec_cmd) {
                    FileStatus::NotFound => format!("File {} could not be opened", sec_cmd),
                    FileStatus::Ok => format!("File {} written", sec_cmd),
                    FileStatus::Other => format!("Couldn't write {}", sec_cmd),
                }
            },
            "ls" => {
                fn print_buffer(i: usize, b: &SplitBuffer) -> String {
                    let title = b.get_title().unwrap_or("<No Title>");

                    format!("b{}\t\t\t{}", i, title)
                }

                let buffer_descriptions : Vec<_> =
                    self.buffers.iter().enumerate().map(|(i, b)| print_buffer(i, b)).collect();

                self.buffers.push(SplitBuffer::from_str(&buffer_descriptions.join("\n"), "<Buffers>"));
                self.current_buffer_index = self.buffers.len() - 1;
                self.redraw_task = RedrawTask::Full;
            },
            "help" => {
                self.open("/apps/sodium/help.txt");
            },
            "q" | "quit" => {
                exit(0);
            },
            c => {
                if let Some(buffer_command) = try_get_buffer_command(c) {
                    match buffer_command {
                        BufferCommand::SwitchToBuffer(n) => {
                            if n >= self.buffers.len() {
                                self.status_bar.msg = format!("Invalid buffer #{}", n);
                            } else {
                                self.current_buffer_index = n;
                                self.redraw_task = RedrawTask::Full;
                                self.status_bar.msg = format!("Switched to buffer #{}", n);
                            }
                        },
                    }
                } else {
                    self.status_bar.msg = format!("Unknown command: {}", c);
                }
            }
        }

        self.hint();
    }
}
