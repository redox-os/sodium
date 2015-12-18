use editor::Editor;
use parse::Inst;
use position::to_signed_pos;

impl Editor {
    /// Convert an instruction to a motion (new coordinate). Returns None if the instructions given
    /// either is invalid or has no movement.
    ///
    /// A motion is a namespace (i.e. non mode-specific set of commands), which represents
    /// movements. These are useful for commands which takes a motion as post-parameter, such as d.
    /// d deletes the text given by the motion following. Other commands can make use of motions,
    /// using this method.
    pub fn to_motion(&mut self, Inst(n, cmd): Inst) -> Option<(usize, usize)> {
        use key::Key::*;

        let x = self.x();
        let y = self.y();

        match cmd.key {
            Char('h') => Some(self.left(n.d())),
            Char('l') => Some(self.right(n.d())),
            Char('j') => Some(self.down(n.d())),
            Char('k') => Some(self.up(n.d())),
            Char('g') => Some((0, n.or(1) - 1)),
            Char('G') => Some((0, self.buffer.len() - 1)),
            Char('L') => Some((self.buffer[y].len() - 1, y)),
            Char('H') => Some((0, y)),
            Char('t') => {

                let ch = self.get_char();

                if let Some(o) = self.next_ocur(ch, n.d()) {
                    Some((o, y))
                } else {
                    None
                }
            }
            Char('f') => {

                let ch = self.get_char();

                if let Some(o) = self.previous_ocur(ch, n.d()) {
                    Some((o, y))
                } else {
                    None
                }
            }
            Char(c) => {
                self.status_bar.msg = format!("Motion not defined: '{}'", c);
                self.redraw_status_bar();
                None
            }
            _ => {
                self.status_bar.msg = format!("Motion not defined");
                None
            }
        }
    }
    /// Like to_motion() but does not bound to the text. Therefore it returns an isize, and in some
    /// cases it's a position which is out of bounds. This is useful when commands want to mesure
    /// the relative movement over the movement.
    pub fn to_motion_unbounded(&mut self, Inst(n, cmd): Inst) -> Option<(isize, isize)> {
        use key::Key::*;

        let x = self.x();
        let y = self.y();

        match cmd.key {
            Char('h') => Some(self.left_unbounded(n.d())),
            Char('l') => Some(self.right_unbounded(n.d())),
            Char('j') => Some(self.down_unbounded(n.d())),
            Char('k') => Some(self.up_unbounded(n.d())),
            Char('g') => Some((0, n.or(1) as isize - 1)),
            Char('G') => Some((0, self.buffer.len() as isize - 1)),
            Char('L') => Some(to_signed_pos((x, self.buffer[y].len()))),
            Char('H') => Some((0, y as isize)),
            Char('t') => {

                let ch = self.get_char();

                if let Some(o) = self.next_ocur(ch, n.d()) {
                    Some((to_signed_pos((o, y))))
                } else {
                    None
                }
            }
            Char('f') => {

                let ch = self.get_char();

                if let Some(o) = self.previous_ocur(ch, n.d()) {
                    Some((to_signed_pos((o, y))))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
