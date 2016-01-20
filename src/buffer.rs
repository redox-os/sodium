use std::ops::{Range, RangeFull, RangeTo, RangeFrom, Index, IndexMut, Add};
use std::cmp::min;
use std::str::Chars;
use std::string::Drain;
use collections::range::RangeArgument;

pub trait Slice<'a> : 'a + ToString
                      + Index<Range<usize>, Output = Self>
                      + Index<RangeFull, Output = Self>
                      + Index<RangeTo<usize>, Output = Self>
                      + Index<RangeFrom<usize>, Output = Self>
                      + IndexMut<Range<usize>, Output = Self>
                      + IndexMut<RangeFull, Output = Self>
                      + IndexMut<RangeTo<usize>, Output = Self>
                      + IndexMut<RangeFrom<usize>, Output = Self> {
    /// Get the length of the slice
    fn len(&self) -> usize;

    /// Convert the slice to an &str
    fn as_str(&self) -> &str;

    /// New empty
    fn new_empty() -> &'a Self;

    /// To line
    fn to_line<T: Line<'a, Slice = Self>>(&self) -> T {
        T::from_slice(self)
    }
}

impl<'a> Slice<'a> for str {
    fn len(&self) -> usize {
        str::len(self)
    }

    fn as_str(&self) -> &str {
        self
    }

    fn new_empty() -> &'a str {
        ""
    }
}


pub trait Line<'a> : 'a + ToString + Add<&'a str, Output = Self> + From<&'a str> {
    // TODO: Try to remove these lifetimes
    /// The characters iterator type
    type Iter: Iterator<Item = char> + DoubleEndedIterator + 'a;
    /// The draining characters iterator type
    type Drain: Iterator<Item = char> + DoubleEndedIterator + 'a;
    /// The slice type
    // TODO: Move this to seperate trait.
    type Slice: Slice<'a> + ?Sized;

    /// Convert the line to a slice
    fn as_slice(&self) -> &Self::Slice;

    /// Convert the line to a slice (mutable)
    fn as_slice_mut(&mut self) -> &mut Self::Slice;

    /// Get an iterator over the characters
    fn chars(&self) -> Self::Iter;

    /// Get a draining iterator over the characters
    fn drain<R: RangeArgument<usize>>(&'a mut self, range: R) -> Self::Drain;

    /// Insert a character into the line
    fn insert(&mut self, idx: usize, ch: char);

    /// Removes a character and returns it
    fn remove(&mut self, idx: usize) -> char;

    /// Set char
    fn set_char(&mut self, idx: usize, ch: char);

    /// Push a string to the end of the line
    fn push_slice(&mut self, s: &Self::Slice);

    /// Convert a slice to a line
    fn from_slice(s: &Self::Slice) -> Self;

    /// Get the length of the line
    fn len(&self) -> usize;

    /// Clear the line (make it empty)
    fn clear(&mut self);

    /// Get the leading whitespaces of the nth line. Used for autoindenting.
    fn get_indent(&self) -> &Self::Slice {
        let mut len = 0;
        for c in self.chars() {
            match c {
                '\t' | ' ' => len += 1,
                _          => break,
            }
        }
        &self.as_slice()[..len]
    }
}


impl<'a> Line<'a> for String {
    type Iter = Chars<'a>;
    type Drain = Drain<'a>;
    type Slice = str;

    fn as_slice(&self) -> &Self::Slice {
        &self
    }

    fn as_slice_mut(&mut self) -> &mut Self::Slice {
        &mut self
    }

    fn chars(&self) -> Chars<'a> {
        String::chars(self)
    }

    fn drain<R: RangeArgument<usize>>(&'a mut self, range: R) -> Drain<'a> {
        String::drain(self, range)
    }

    fn insert(&mut self, idx: usize, ch: char) {
        String::insert(self, idx, ch);
    }

    fn remove(&mut self, idx: usize) -> char {
        String::remove(self, idx)
    }

    fn set_char(&mut self, idx: usize, ch: char) {
        // TODO: Performance
        self.remove(idx);
        self.insert(idx, ch);
    }

    fn push_slice(&mut self, s: &str) {
        String::push_str(self, s);
    }

    fn from_slice(s: &Self::Slice) -> Self {
        s.into()
    }

    fn len(&self) -> usize {
        String::len(self)
    }

    fn clear(&mut self) {
        self.clear();
    }
}

// TODO Take slices instead of Self::Line!
/// A buffer structure
pub trait Buffer<'a> : 'a {
    type Line: 'a + Line<'a>;
    type LineIter: Iterator<Item = &'a Self::Line>;

    /// Create a new empty split buffer
    fn new() -> Self;

    /// Convert a string to a split buffer
    fn from_str(s: &str) -> Self;

    /// Get the nth line in the buffer by option reference
    fn get_line(&self, n: usize) -> &Self::Line;

    /// Get the nth line in the buffer by optional mutable reference
    fn get_line_mut(&mut self, n: usize) -> &mut Self::Line;

    /// Remove the nth line and return it. Panics on out of bound.
    fn remove_line(&mut self, n: usize) -> Self::Line;

    /// Insert line at n. Panics on out of bound.
    fn insert_line(&mut self, n: usize, line: &<Self::Line as Line<'a>>::Slice);

    /// Convert a vector of lines to a split buffer
    fn from_lines(vec: &[Self::Line]) -> SplitBuffer;

    /// Give a hint on where the operations are most frequent (i.e. where the cursor is). X value.
    fn focus_hint_x(&mut self, x: usize);

    /// Give a hint on where the operations are most frequent (i.e. where the cursor is). Y value.
    fn focus_hint_y(&mut self, y: usize);

    /// Get the number of lines in the buffer
    fn len(&self) -> usize;

    /// Get an iterator over the lines in the buffer
    fn lines<'b: 'a>(&'b self) -> Self::LineIter;

    /// Insert a newline at a given point (yields the indentation of the previous line)
    fn insert_newline<'b: 'a>(&'b mut self, x: usize, y: usize, autoindent: bool) -> usize {
        let first_part;
        let second_part;

        let slice = self.get_line(y).as_slice();

        // TODO Is this efficient?
        // TODO Make RangeTo work
        // (instead of `0..`)
        first_part  = &slice[x..]; // Fuck you, borrowck
        second_part = &slice[x..];

        *self.get_line_mut(y) = first_part.as_str().into();

        let mut nl = if autoindent {
            self.get_line(y).get_indent()
        } else {
            <<Self as Buffer<'a>>::Line as Line<'a>>::Slice::new_empty()
        };
        let begin = nl.len();

        nl.to_line::<Self::Line>().push_slice(second_part);

        self.insert_line(y, nl);

        begin
    }
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
        self.before.last().expect("Unexpected condition (the first part of the split buffer is empty)")
    }

    fn cur_line_mut(&mut self) -> &mut String {
        self.before.last_mut().expect("Unexpected condition (the first part of the split buffer is empty)")
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
    fn get_line(&self, n: usize) -> &String {
        if n < self.before.len() {
            &self.before[n]
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            &self.after[n]
        } else {
            panic!("Out of bound");
        }
    }

    /// Get the nth line in the buffer by optional mutable reference
    fn get_line_mut(&mut self, n: usize) -> &mut String {
        #[cfg(debug)]
        fn debug_check(b: &mut B) {
            if b._hinted_since_edit {
                b._hinted_since_edit = false;
            } else {
                panic!("No focus hint given since last edit!");
            }
        }

        #[cfg(debug)]
        debug_check(&mut *self);

        if n < self.before.len() {
            &mut self.before[n]
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            &mut self.after[n]
        } else {
            panic!("Out of bound");
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
    fn insert_line(&mut self, n: usize, line: &str) {
        if n < self.before.len() {
            self.before.insert(n, line.into());
        } else if n < self.len() {
            let n = self.len() - 1 - n;
            self.after.insert(n, line.into());
        } else {
            panic!("Out of bound");
        }
    }

    /// Convert a vector of lines to a split buffer
    fn from_lines(ln: &[String]) -> SplitBuffer {
        SplitBuffer {
            before: ln.into(),
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
    fn lines<'b: 'a>(&'b self) -> SplitBufIter<'b> {
        SplitBufIter {
            buffer: self,
            line: 0,
        }
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
        if n + self.line < self.buffer.len() {
            let res = self.buffer.get_line(self.line);
            self.line += n;

            Some(res)
        } else {
            None
        }
    }

    fn count(self) -> usize {
        self.buffer.len()
    }
}

impl<'a> DoubleEndedIterator for SplitBufIter<'a> {
    fn next_back(&mut self) -> Option<&'a String> {
        if self.line == 0 || self.line > self.buffer.len() {
            None
        } else {
            self.line -= 1;
            Some(self.buffer.get_line(self.line))
        }
    }
}
