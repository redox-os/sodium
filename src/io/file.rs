use edit::buffer::{Buffer, SplitBuffer};
use state::editor::Editor;
use std::fs::File;
use std::io::{Read, Write};

/// The status of a file IO operation.
pub enum FileStatus {
    /// Oll fino.
    Ok,
    /// File not found.
    NotFound,
    /// Other error.
    Other,
}

impl Editor {
    /// Open a file.
    pub fn open(&mut self, path: &str) -> FileStatus {
        self.status_bar.file = path.to_owned();
        if let Some(mut file) = File::open(path).ok() {
            let mut con = String::new();
            let _ = file.read_to_string(&mut con);

            self.buffers.push(SplitBuffer::from_str(&con));
            self.current_buffer_index = self.buffers.len() - 1;
            self.hint();
            FileStatus::Ok
        } else {
            FileStatus::NotFound
        }
    }

    /// Write the file.
    pub fn write(&mut self, path: &str) -> FileStatus {
        self.status_bar.file = path.to_owned();
        if let Some(mut file) = File::create(path).ok() {
            if file.write(self.current_buffer().to_string().as_bytes()).is_ok() {
                FileStatus::Ok
            } else {
                FileStatus::Other
            }
        } else {
            FileStatus::NotFound
        }
    }
}
