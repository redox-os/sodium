use editor::Editor;
use buffer::{Buffer, Line};

/// Convert a usize tuple to isize
pub fn to_signed_pos((x, y): (usize, usize)) -> (isize, isize) {
    (x as isize, y as isize)
}

impl<'a, B: Buffer<'a>> Editor<B> {
    /// Get the position of the current cursor, bounded
    #[inline]
    pub fn pos(&self) -> (usize, usize) {
        let cursor = self.cursor();
        self.bound((cursor.x, cursor.y))
    }

    #[inline]
    /// Get the X coordinate of the current cursor (bounded)
    pub fn x(&self) -> usize {
        self.pos().0
    }

    #[inline]
    /// Get the Y coordinate of the current cursor (bounded)
    pub fn y(&self) -> usize {
        self.pos().1
    }

    /// Convert a position value to a bounded position value
    #[inline]
    pub fn bound(&self, (x, mut y): (usize, usize)) -> (usize, usize) {


        y = if y >= self.buffer.len() {
            self.buffer.len() - 1
        } else {
            y
        };

        let ln = self.buffer.get_line(y).len();
        if x >= ln {
            if ln == 0 {
                (0, y)
            } else {
                (ln - 1, y)
            }
        } else {
            (x, y)
        }
    }

    /// Bound horizontally, i.e. don't change the vertical axis only make sure that the horizontal
    /// axis is bounded.
    #[inline]
    pub fn bound_hor(&self, (x, y): (usize, usize)) -> (usize, usize) {

        (self.bound((x, y)).0, y)
    }
    /// Bound vertically, i.e. don't change the horizontal axis only make sure that the vertical
    /// axis is bounded.
    #[inline]
    pub fn bound_ver(&self, (x, mut y): (usize, usize)) -> (usize, usize) {

        // Is this premature optimization? Yes, yes it is!
        y = if y > self.buffer.len() - 1 {
            self.buffer.len() - 1
        } else {
            y
        };

        (x, y)
    }

    /// Give a hint about the cursor position to the buffer, used for setting the center/focus in
    /// which operations are most efficient.
    pub fn hint(&mut self) {
        let x = self.cursor().x;
        let y = self.cursor().y;

        self.buffer.focus_hint_y(y);
        self.buffer.focus_hint_x(x);
    }
}
