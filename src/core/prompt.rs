use io::file::FileStatus;
use state::editor::Editor;

use std::process::exit;

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
            "help" => {
                self.open("/apps/sodium/help.txt");
            },
            "q" | "quit" => {
                exit(0);
            },
            c => {
                self.status_bar.msg = format!("Unknown command: {}", c);
            }
        }

        self.hint();
    }
}
