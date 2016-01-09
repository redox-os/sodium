use std::ops::{Index, IndexMut};
use std::cmp::min;
use std::str::Chars;

pub trait Line<'a> {
    type Iter: Iterator<Item = char> + 'a;

    fn chars_iter(&'a self) -> Self::Iter;
}

impl<'a, T: AsRef<str>> Line<'a> for T {
    type Iter = Chars<'a>;

    fn chars_iter(&self) -> Chars {
        self.as_ref().chars()
    }
}

/// A buffer structure
pub trait Buffer<'a> {
    type Line: 'a + Line<'a>;
    type LineIter: Iterator<Item = &'a Self::Line>;

    /// Create a new empty split buffer
    fn new() -> Self;

    /// Convert a string to a split buffer
    fn from_str(s: &str) -> Self;

    /// Get the nth line in the buffer by option reference
    fn get_line(&self, n: usize) -> Option<&Self::Line>;

    /// Get the nth line in the buffer by optional mutable reference
    fn get_line_mut(&mut self, n: usize) -> Option<&mut Self::Line>;

    /// Remove the nth line and return it. Panics on out of bound.
    fn remove_line(&mut self, n: usize) -> Self::Line;

    /// Insert line at n. Panics on out of bound.
    fn insert_line(&mut self, n: usize, line: Self::Line);

    /// Convert a vector of lines to a split buffer
    fn from_lines(vec: &[Self::Line]) -> SplitBuffer;

    /// Give a hint on where the operations are most frequent (i.e. where the cursor is). X value.
    fn focus_hint_x(&mut self, x: usize);

    /// Give a hint on where the operations are most frequent (i.e. where the cursor is). Y value.
    fn focus_hint_y(&mut self, y: usize);

    /// Get the number of lines in the buffer
    fn len(&self) -> usize;

    /// Get an iterator over the lines in the buffer
    fn lines(&'a self) -> Self::LineIter;

    /// Get the leading whitespaces of the nth line. Used for autoindenting.
    fn get_indent(&self, n: usize) -> &str;
}


/// The buffer data structure, that Sodium is using.
///
/// This structure consists of two "subbuffers", which are just vectors over lines (defined by
/// Strings). The split is called a center.
///
/// The nearer a given operation is to the center, the better it performs.
///
/// The second buffer is in reverse order to get the particular efficiency we want.
pub struct SplitBuffer {
    before: Vec<String>,
    after: Vec<String>,
    #[cfg(debug)]
    _hinted_since_edit: bool,
}

impl SplitBuffer {
    fn cur_line(&self) -> &String {
        self.before
            .last()
            .expect("Unexpected condition (the first part of the split buffer is empty)")
    }

    fn cur_line_mut(&mut self) -> &mut String {
        self.before
            .last_mut()
            .expect("Unexpected condition (the first part of the split buffer is empty)")
    }

    fn up(&mut self) {
        self.after.push(self.before.pop().expect("Popped last element"));
    }

    fn down(&mut self) {
        self.before.push(self.after.pop().expect("Popped last element"));
    }

    fn y(&self) -> usize {
        self.before.len()
    }

    fn pop_line(&mut self) -> String {
        self.before.pop().expect("Unexpected condition (Popped the last line)")
    }
}

impl<'a> Buffer<'a> for SplitBuffer {
    type Line = String;
    type LineIter = SplitBufIter<'a>;

    /// Create a new empty split buffer
    fn new() -> Self {
        SplitBuffer {
            before: vec![String::new()],
            after: Vec::new(),
        }
    }

    /// Convert a string to a split buffer
    fn from_str(s: &str) -> Self {
        SplitBuffer {
            before: s.lines().map(str::to_string).collect(),
            after: Vec::new(),
        }
    }

    /// Get the nth line in the buffer by option reference
    fn get_line(&self, n: usize) -> Option<&String> {
        if n < self.before.len() {
            Some(&self.before[n])
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            Some(&self.after[n])
        } else {
            None
        }
    }

    /// Get the nth line in the buffer by optional mutable reference
    fn get_line_mut(&mut self, n: usize) -> Option<&mut String> {
        if n < self.before.len() {
            Some(&mut self.before[n])
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            Some(&mut self.after[n])
        } else {
            None
        }
    }

    /// Remove the nth line and return it. Panics on out of bound.
    fn remove_line(&mut self, n: usize) -> String {
        if n < self.before.len() {
            self.before.remove(n)
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            self.after.remove(n)
        } else {
            panic!("Out of bound");
        }
    }

    /// Insert line at n. Panics on out of bound.
    fn insert_line(&mut self, n: usize, line: String) {
        if n < self.before.len() {
            self.before.insert(n, line);
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            self.after.insert(n, line);
        } else {
            panic!("Out of bound");
        }
    }

    /// Convert a vector of lines to a split buffer
    fn from_lines(ln: &[String]) -> SplitBuffer {
        SplitBuffer {
            before: ln.to_owned(),
            after: Vec::new(),
        }
    }

    /// Move the center (i.e. efficient point/split) of the split buffer
    ///
    /// Panics on out of bound.
    fn focus_hint_y(&mut self, y: usize) {
        if y < self.y() {
            for _ in 0..min(self.y() - y, self.before.len()) {
                self.up();
            }
        } else if y > self.y() {
            for _ in 0..min(y - self.y(), self.after.len()) {
                self.down();
            }
        } else if y >= self.len() {
            panic!("Out of bound");
        }
    }

    fn focus_hint_x(&mut self, _: usize) {}

    /// Get the number of lines in the buffer
    fn len(&self) -> usize {
        self.before.len() + self.after.len()
    }

    /// Get an iterator over the lines in the buffer
    fn lines(&'a self) -> SplitBufIter<'a> {
        SplitBufIter {
            buffer: self,
            line: 0,
        }
    }

    /// Get the leading whitespaces of the nth line. Used for autoindenting.
    fn get_indent(&self, n: usize) -> &str {
        if let Some(ln) = self.get_line(n) {
            let mut len = 0;
            for c in ln.chars() {
                match c {
                    '\t' | ' ' => len += 1,
                    _ => break,
                }
            }
            &ln[..len]
        } else {
            ""
        }
    }
}


impl Index<usize> for SplitBuffer {
    type Output = String;

    fn index<'a>(&'a self, index: usize) -> &'a String {
        self.get_line(index).expect("Out of bound")
    }
}
impl IndexMut<usize> for SplitBuffer {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut String {
        #[cfg(debug)]
        fn debug_check(b: &mut SplitBuffer) {
            if b._hinted_since_edit {
                b._hinted_since_edit = false;
            } else {
                panic!("No focus hint given since last edit!");
            }
        }

        #[cfg(not(debug))]
        fn debug_check(_: &mut SplitBuffer) {}

        debug_check(&mut *self);

        self.get_line_mut(index).expect("Out of bound")
    }
}

/// A iterator over the lines of a split buffer
pub struct SplitBufIter<'a> {
    buffer: &'a SplitBuffer,
    line: usize,
}

impl<'a> Iterator for SplitBufIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<&'a String> {
        self.nth(1)
    }

    fn nth(&mut self, n: usize) -> Option<&'a String> {
        let res = self.buffer.get_line(self.line);
        self.line += n;

        res
    }

    fn count(self) -> usize {
        self.buffer.len()
    }
}

impl<'a> DoubleEndedIterator for SplitBufIter<'a> {
    fn next_back(&mut self) -> Option<&'a String> {
        if self.line == 0 {
            None
        } else {
            self.line -= 1;
            self.buffer.get_line(self.line)
        }
    }
}
