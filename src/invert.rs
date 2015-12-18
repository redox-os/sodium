use editor::Editor;

impl Editor {
    pub fn invert_chars(&mut self, n: usize) {
        for _ in 0..n {
            let (x, y) = self.pos();
            let current = self.current();

            if let Some(cur) = current {
                self.buffer[y].remove(x);
                self.buffer[y].insert(x, invert(cur));
            }
            if let Some(m) = self.next(1) {
                self.goto(m);
            }
        }
    }
}

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
        a => if a.is_lowercase() {
            a.to_uppercase().next().unwrap_or('?')
        } else {
            a.to_lowercase().next().unwrap_or('?')
        },
    }
}
