use std::mem::swap;
use std::ops::{Index, IndexMut};


// Wow, I would really love having unboxed trait return types...
pub type BufIter<'a> = ::std::iter::Chain<::std::slice::Iter<'a, String>, ::std::iter::Rev<::std::slice::Iter<'a, String>>>;


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

    pub fn from_str(s: &str) -> Self {
        SplitBuffer {
            before: s.lines().map(str::to_string).collect(),
            after: Vec::new(),
        }
    }

    fn cur_line(&self) -> &String {
        self.before.last().expect("Unexpected condition (the first part of the split buffer is empty)")
    }

    fn cur_line_mut(&mut self) -> &mut String {
        self.before.last_mut().expect("Unexpected condition (the first part of the split buffer is empty)")
    }

    fn up(&mut self) -> bool {
        if self.before.len() == 1 {
            false
        } else {
            self.after.push(self.before.pop().unwrap());
            true
        }
    }

    fn down(&mut self) -> bool {
        if self.after.len() == 0 {
            false
        } else {
            self.before.push(self.after.pop().unwrap());
            true
        }
    }

    fn y(&self) -> usize {
        self.before.len()
    }

    pub fn get_line(&self, n: usize) -> Option<&String> {
        if n < self.before.len() {
            Some(&self.before[n])
        } else if n < self.len() {
            Some(&self.after[n - self.before.len()])
        } else {
            None
        }
    }

    pub fn get_line_mut(&mut self, n: usize) -> Option<&mut String> {
        if n < self.before.len() {
            Some(&mut self.before[n])
        } else if n < self.len() {
            Some(&mut self.after[n - self.before.len()])
        } else {
            None
        }
    }

    pub fn remove_line(&mut self, n: usize) {
        if n < self.before.len() {
            self.before.remove(n);
        } else if n < self.len() {
            self.after.remove(n - self.before.len());
        } else {
            panic!("Out of bound");
        }
    }

    pub fn insert_line(&mut self, n: usize, line: String) {
        if n < self.before.len() {
            println!("C1 n: {}", n);
            self.before.insert(n, line);
        } else if n < self.len() {
            println!("C2");
            self.after.insert(n - self.before.len(), line);
        } else {
            panic!("Out of bound");
        }
    }

    pub fn from_lines(vec: Vec<String>) -> SplitBuffer {
        SplitBuffer {
            before: vec,
            after: Vec::new(),
        }
    }

    pub fn goto(&mut self, y: usize) {
        if y < self.y() {
            for _ in 0..self.y() - y {
                if !self.up() {
                    break;
                }
            }
        } else if y > self.y() {
            for _ in 0..y - self.y() {
                if !self.down() {
                    break;
                }
            }
        }
    }

    fn pop_line(&mut self) -> String {
        self.before.pop().expect("Unexpected condition (Popped the last line)")
    }

    pub fn len(&self) -> usize {
        self.before.len() + self.after.len()
    }

    pub fn lines<'a>(&'a self) -> BufIter<'a> {
        self.after.iter().chain(self.before.iter().rev())
    }

    /// Get the leading whitespaces of the nth line. Used for autoindenting.
    pub fn get_indent(&self, n: usize) -> &str {
        let ln = if let Some(s) = self.get_line(n) {
            s
        } else {
            return "";
        };
        let mut len = 0;
        for c in ln.chars() {
            match c {
                '\t' | ' ' => len += 1,
                _ => break,
            }
        }
        &ln[..len]
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
        self.get_line_mut(index).expect("Out of bound")
    }
}
