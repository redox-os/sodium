use state::editor::Editor;

impl Editor {
    /// Invert n characters next to the cursor in the buffer.
    pub fn invert_chars(&mut self, n: usize) {
        for _ in 0..n {
            let (x, y) = self.pos();
            let current = self.current();

            if let Some(cur) = current {
                self.buffers.current_buffer_mut()[y].remove(x);
                self.buffers.current_buffer_mut()[y].insert(x, invert(cur));
            }
            if let Some(m) = self.next(1) {
                self.goto(m);
            }
        }

        self.hint();
    }
}

/// "Invert" a character, meaning that it gets swapped with it's counterpart, if no counterpart
/// exists, swap the case of the character.
pub fn invert(c: char) -> char {
    match c {
        '<' => '>',
        '>' => '<',
        '&' => '|',
        '*' => '/',
        '(' => ')',
        ')' => '(',
        '+' => '-',
        '-' => '+',
        ';' => ':',
        ':' => ';',
        '\\' => '/',
        '/' => '\\',
        ',' => '.',
        '.' => ',',
        '\'' => '"',
        '"' => '\'',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '!' => '?',
        '?' => '!',
        a => {
            if a.is_lowercase() {
                a.to_uppercase().next().unwrap_or('?')
            } else {
                a.to_lowercase().next().unwrap_or('?')
            }
        }
    }
}
