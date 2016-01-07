use editor::Editor;
use std::fs::File;
use std::io::Read;
use buffer::{Buffer, SplitBuffer};

pub enum OpenStatus {
    Ok,
    NotFound,
}

impl<'a, B: Buffer<'a>> Editor<B> {
    /// Open a file
    pub fn open(&'a mut self, path: &str) -> OpenStatus {
        self.status_bar.file = path.to_string();
        if let Some(mut file) = File::open(path).ok() {
            let mut con = String::new();
            file.read_to_string(&mut con);

            self.buffer = B::from_str(&con);
            self.hint();
            OpenStatus::Ok
        } else {
            OpenStatus::NotFound
        }
    }
}
