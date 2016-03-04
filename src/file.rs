use buffer::{Buffer, SplitBuffer};
use editor::Editor;
use std::fs::File;
use std::io::{Read, Write};

pub enum FileStatus {
    Ok,
    NotFound,
    Other,
}

impl Editor {
    /// Open a file.
    pub fn open(&mut self, path: &str) -> FileStatus {
        self.status_bar.file = path.to_string();
        if let Some(mut file) = File::open(path).ok() {
            let mut con = String::new();
            let _ = file.read_to_string(&mut con);

            self.buffer = SplitBuffer::from_str(&con);
            self.hint();
            FileStatus::Ok
        } else {
            FileStatus::NotFound
        }
    }

    /// Write the file.
    pub fn write(&mut self, path: &str) -> FileStatus {
        self.status_bar.file = path.to_string();
        if let Some(mut file) = File::create(path).ok() {
            if file.write(self.buffer.to_string().as_bytes()).is_ok() {
                FileStatus::Ok
            } else {
                FileStatus::Other
            }
        } else {
            FileStatus::NotFound
        }
    }
}
