use std::mem::swap;
use std::ops::{Index, IndexMut};


/// The buffer data structure, that Sodium is using.
///
/// This structure consists of two "subbuffers", which are just vectors over lines (defined by
/// Strings). The split is where the cursor currently is.
///
/// The second buffer is in reverse order to get the particular efficiency we want.
pub struct SplitBuffer {
    before: Vec<String>,
    after: Vec<String>,
}

impl SplitBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        SplitBuffer {
            before: vec![String::new()],
            after: Vec::new(),
        }
    }

    pub fn cur_line(&self) -> &String {
        self.before.last().expect("Unexpected condition (the first part of the split buffer is empty)")
    }

    pub fn cur_line_mut(&mut self) -> &mut String {
        self.before.last_mut().expect("Unexpected condition (the first part of the split buffer is empty)")
    }

    pub fn up(&mut self) -> bool {
        if self.before.len() == 1 {
            false
        } else {
            self.after.push(self.before.pop());
            true
        }
    }

    pub fn down(&mut self) -> bool {
        if self.after.len() == 1 {
            false
        } else {
            self.before.push(self.after.pop());
            true
        }
    }

    pub fn y(&self) -> usize {
        self.before.len()
    }

    pub fn get_line(&self, n: usize) -> Option<&String> {
        if n < self.before.len() {
            Some(&self.before[n])
        } else if n < self.before.len() + self.after.len() {
            Some(&self.after[n - self.before.len()])
        } else {
            None
        }
    }

    pub fn get_line_mut(&mut self, n: usize) -> Option<&mut String> {
        if n < self.before.len() {
            Some(&mut self.before[n])
        } else if n < self.before.len() + self.after.len() {
            Some(&mut self.after[n - self.before.len()])
        } else {
            None
        }
    }

    pub fn remove_line(&mut self, n: usize) {
        if n < self.before.len() {
            self.before.remove(n);
        } else if n < self.before.len() + self.after.len() {
            self.after.remove(n - self.before.len());
        }
    }

    pub fn insert_line(&mut self, n: usize, line: String) {
        if n < self.before.len() {
            self.before.insert(n, line);
        } else if n < self.before.len() + self.after.len() {
            self.after.insert(n - self.before.len(), line);
        }
    }

    pub fn from_lines(vec: Vec<String>) -> SplitBuffer {
        SplitBuffer {
            before: vec,
            after: Vec::new(),
        }
    }

    pub fn goto(&mut self, y: usize) {
        if y < self.line() {
            for _ in 1..self.line() - y {
                if !self.up() {
                    break;
                }
            }
        } else if y > self.line() {
            for _ in 1..y - self.line() {
                if !self.down() {
                    break;
                }
            }
        }
    }

    pub fn pop_line(&mut self) -> String {
        self.before.expect("Unexpected condition (Popped the last line)")
    }

    pub fn len(&self) -> usize {
        self.before.len() + self.after.len()
    }

    pub fn lines(&self) -> ::std::iter::Once<Vec<char>> {
        self.after.iter().chain(self.before.iter().reverse())
    }
}


impl Index<usize> for SplitBuffer {
    type Output = String;

    fn index<'a>(&'a self, index: usize) -> &'a Foo {
        self.get_line(index).expect("Out of bound")
    }
}
impl IndexMut<usize> for SplitBuffer {
    type Output = String;

    fn index<'a>(&'a self, index: usize) -> &'a mut Foo {
        self.get_line(index).expect("Out of bound")
    }
}
