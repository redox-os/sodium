use editor::Editor;

impl Editor {
    /// Goto a given position. Does not automatically bound.
    #[inline]
    pub fn goto(&mut self, (x, y): (usize, usize)) {
        self.buffer.goto(y);
        self.cursor_mut().y = y;
        self.cursor_mut().x = x;
    }

    /// Get the previous position, i.e. the position before the cursor (*not* left to the cursor)
    #[inline]
    pub fn previous(&self, n: usize) -> Option<(usize, usize)> {
        self.before(n, self.pos())
    }
    /// Get the next position, i.e. the position after the cursor (*not* right to the cursor)
    #[inline]
    pub fn next(&self, n: usize) -> Option<(usize, usize)> {
        self.after(n, self.pos())
    }

    /// Get position after a given position, i.e. a generalisation of .next()
    #[inline]
    pub fn after(&self, n: usize, (x, y): (usize, usize)) -> Option<(usize, usize)> {

        // TODO: Make this more idiomatic {
        if x + n < self.buffer[y].len() {

            Some((x + n, y))
        } else {
            if y + 1 >= self.buffer.len() {
                None
            } else {
                let mut mv = n + x - self.buffer[y].len();
                let mut ry = y + 1;

                loop {
                    if mv < self.buffer[ry].len() {
                        return Some((mv, ry));
                    } else {
                        if ry + 1 < self.buffer.len() {
                            mv -= self.buffer[ry].len();
                            ry += 1;
                        } else {
                            return None;
                        }
                    }
                }

            }
        }
        // }
    }

    /// Get the position before a given position, i.e. a generalisation .before()
    #[inline]
    pub fn before(&self, n: usize, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if x >= n {
            Some((x - n, y))
        } else {
            if y == 0 {
                None
            } else {
                let mut mv = n - x;
                let mut ry = y - 1;

                loop {
                    if mv <= self.buffer[ry].len() {
                        return Some((self.buffer[ry].len() - mv, ry));
                    } else {
                        if ry > 0 && mv >= self.buffer[ry].len() {
                            mv -= self.buffer[ry].len();
                            ry -= 1;
                        } else if ry == 0 {
                            return None;

                        }
                    }
                }
            }
        }
    }

    /// Get the position of the character right to the cursor (horizontally bounded)
    #[inline]
    pub fn right(&self, n: usize) -> (usize, usize) {
        self.bound_hor((self.x() + n, self.y()))
    }
    /// Get the position of the character right to the cursor (unbounded)
    #[inline]
    pub fn right_unbounded(&self, n: usize) -> (isize, isize) {
        ((self.x() + n) as isize, self.y() as isize)
    }

    /// Get the position of the character left to the cursor (horizontally bounded)
    #[inline]
    pub fn left(&self, n: usize) -> (usize, usize) {
        if n <= self.x() {
            (self.x() - n, self.y())
        } else {
            (0, self.y())
        }
    }
    /// Get the position of the character left to the cursor (unbounded)
    #[inline]
    pub fn left_unbounded(&self, n: usize) -> (isize, isize) {
        (self.x() as isize - n as isize, self.y() as isize)
    }

    /// Get the position of the character above the cursor (vertically bounded)
    #[inline]
    pub fn up(&self, n: usize) -> (usize, usize) {
        if n <= self.y() {
            (self.cursor().x, self.y() - n)
        } else {
            (self.cursor().x, 0)
        }
    }
    /// Get the position of the character above the cursor (unbounded)
    #[inline]
    pub fn up_unbounded(&self, n: usize) -> (isize, isize) {
        (self.cursor().x as isize, self.y() as isize - n as isize)
    }

    /// Get the position of the character under the cursor (vertically bounded)
    #[inline]
    pub fn down(&self, n: usize) -> (usize, usize) {
        self.bound_ver((self.cursor().x, self.y() + n))
    }
    /// Get the position of the character above the cursor (unbounded)
    #[inline]
    pub fn down_unbounded(&self, n: usize) -> (isize, isize) {
        (self.cursor().x as isize, self.y() as isize + n as isize)

    }

    /// Get n'th next ocurrence of a given charecter (relatively to the cursor)
    pub fn next_ocur(&self, c: char, n: usize) -> Option<usize> {
        let mut dn = 0;
        let mut x  = self.x();

        for ch in self.buffer[self.y()].chars().skip(x) {
            if dn == n {
                if ch == c {
                    dn += 1;
                    if dn == n {
                        return Some(x);
                    }
                }
            }
        }

        None
    }

    /// Get n'th previous ocurrence of a given charecter (relatively to the cursor)
    pub fn previous_ocur(&self, c: char, n: usize) -> Option<usize> {
        let mut dn = 0;
        let mut x  = self.x();
        let y      = self.y();

        for ch in self.buffer[y].chars().rev().skip(self.buffer[y].len() - x) {
            if dn == n {
                if ch == c {
                    dn += 1;
                    if dn == n {
                        return Some(x);
                    }
                }
            }
        }

        None
    }


}
