#[cfg(feature = "orbital")]
use edit::buffer::TextBuffer;
#[cfg(feature = "orbital")]
use io::redraw::RedrawTask;
use state::editor::Editor;
#[cfg(feature = "orbital")]
use state::mode::{Mode, PrimitiveMode};

#[cfg(feature = "orbital")]
use orbclient::{Color, Renderer};

use std::iter;

#[cfg(feature = "orbital")]
impl Editor {
    /// Redraw the window
    pub fn redraw(&mut self) {
        let w = self.window.width() as usize;
        let h = self.window.height() as usize;

        let vert_offset: usize = 0;

        let horz_offset: usize = if self.options.line_numbers {
            let len = self.buffers.current_buffer_info().raw_buffer.len();
            let mut ret: usize = 3;
            while len >= 10usize.pow((ret - 1) as u32) {
                ret += 1;
            }
            ret
        } else {
            0
        };

        let max_vert_chars = h / self.char_height - 2 - vert_offset;
        let max_horz_chars = w / self.char_width - horz_offset;

        // Redraw window
        self.window.set(Color::rgb(25, 25, 25));

        let mut scr_lines: usize = 0;
        let mut scr_chars: usize = 0;

        self.cursor_in_window(max_horz_chars, max_vert_chars);

        let (_scroll_x, scroll_y) = {
            let current_buffer = self.buffers.current_buffer_info();

            (current_buffer.scroll_x, current_buffer.scroll_y)
        };

        let (pos_x, pos_y) = self.pos();

        let (window_pos_x, window_pos_y) =
            self.coords_to_window_coords((pos_x, pos_y), max_horz_chars);

        self.window.set(Color::rgb(25, 25, 25));

        if self.options.line_marker {
            self.window.rect(
                0,
                ((window_pos_y + vert_offset) * self.char_height) as i32,
                w as u32,
                16,
                Color::rgb(45, 45, 45),
            );
        }

        self.window.rect(
            ((window_pos_x + horz_offset) * self.char_width) as i32,
            ((window_pos_y + vert_offset) * self.char_height) as i32,
            self.char_width as u32,
            self.char_height as u32,
            Color::rgb(255, 255, 255),
        );

        let mut string = false;

        'outer: for (y, row) in self
            .buffers
            .current_buffer()
            .lines_from(scroll_y)
            .enumerate()
        {
            // Print line numbers
            if self.options.line_numbers {
                let mut line_number = scroll_y + y as usize + 1;
                // The amount of digits for this line number
                let mut digit_nr: usize = 0;
                while line_number >= 10usize.pow(digit_nr as u32) {
                    digit_nr += 1;
                }
                // Print the digits for this line number
                for i in 1..digit_nr + 1 {
                    let digit = ((line_number % 10) as u8 + ('0' as u32) as u8) as char;
                    line_number = (line_number - line_number % 10) / 10 as usize;
                    self.window.char(
                        (self.char_width * (horz_offset - 1 - i)) as i32,
                        (self.char_height * (scr_lines + vert_offset)) as i32,
                        digit,
                        Color::rgb(255, 255, 0),
                    );
                }
            }
            for (x, c) in row
                .chars()
                .flat_map(|c| {
                    if c == '\t' {
                        iter::repeat(' ').take(4)
                    } else {
                        iter::repeat(c).take(1)
                    }
                })
                .enumerate()
            {
                // New screen line
                if scr_chars >= max_horz_chars {
                    scr_chars = 0;
                    scr_lines += 1;
                    if scr_lines > max_vert_chars {
                        break 'outer;
                    }
                }

                // TODO: Move outta here
                let color = if self.options.highlight {
                    match c {
                        '\'' | '"' => {
                            string = !string;
                            (226, 225, 167) //(167, 222, 156)
                        }
                        _ if string => (226, 225, 167), //(167, 222, 156)
                        '!' | '@' | '#' | '$' | '%' | '^' | '&' | '|' | '*' | '+' | '-' | '/'
                        | ':' | '=' | '<' | '>' => (198, 83, 83), //(228, 190, 175), //(194, 106, 71),
                        '.' | ',' => (241, 213, 226),
                        '(' | ')' | '[' | ']' | '{' | '}' => (164, 212, 125), //(195, 139, 75),
                        '0'..='9' => (209, 209, 177),
                        _ => (255, 255, 255),
                    }
                } else {
                    (255, 255, 255)
                };

                if pos_x == x && (pos_y - scroll_y) == y {
                    self.window.char(
                        (self.char_width * (scr_chars + horz_offset)) as i32,
                        (self.char_height * (scr_lines + vert_offset)) as i32,
                        c,
                        Color::rgb(color.0 / 3, color.1 / 3, color.2 / 3),
                    );
                } else {
                    self.window.char(
                        (self.char_width * (scr_chars + horz_offset)) as i32,
                        (self.char_height * (scr_lines + vert_offset)) as i32,
                        c,
                        Color::rgb(color.0, color.1, color.2),
                    );
                }
                scr_chars += 1;
            }
            scr_lines += 1;
            scr_chars = 0;
            if scr_lines > max_vert_chars {
                break;
            }
        }
        self.redraw_status_bar();
        self.redraw_task = RedrawTask::None;
        self.window.sync();
    }

    fn coords_to_window_coords(
        &mut self,
        point: (usize, usize),
        max_horz_chars: usize,
    ) -> (usize, usize) {
        let (_, scroll_y) = {
            let current_buffer = self.buffers.current_buffer_info();

            (current_buffer.scroll_x, current_buffer.scroll_y)
        };

        let to_y = point.1 - scroll_y;

        let mut ret_y = 0;

        let ret_x = point.0 % max_horz_chars;
        for (y, row) in self
            .buffers
            .current_buffer()
            .lines_from(scroll_y)
            .enumerate()
        {
            if to_y > y {
                ret_y += row.len() / max_horz_chars + 1;
            } else {
                ret_y += point.0 / max_horz_chars;
                break;
            }
        }
        (ret_x, ret_y)
    }

    // Ensure that the cursor is visible
    fn cursor_in_window(&mut self, max_horz_chars: usize, max_vert_chars: usize) {
        let (_pos_x, pos_y) = self.pos();
        if self.buffers.current_buffer_info().scroll_y > 0
            && pos_y <= self.buffers.current_buffer_info().scroll_y
        {
            self.buffers.current_buffer_info_mut().scroll_y =
                if pos_y == 0 { pos_y } else { pos_y - 1 };
            return;
        }

        let scroll_y = self.buffers.current_buffer_info().scroll_y;
        let mut line_counter = 0;
        let mut result_y = 0;

        for (y, row) in self
            .buffers
            .current_buffer()
            .lines_from(pos_y + 1)
            .rev()
            .enumerate()
        {
            if pos_y - y < scroll_y {
                return;
            }
            line_counter += row.len() / max_horz_chars + 1;
            if line_counter > max_vert_chars {
                result_y = pos_y - y;
                break;
            }
        }
        self.buffers.current_buffer_info_mut().scroll_y = result_y;
    }

    /// Redraw the status bar
    pub fn redraw_status_bar(&mut self) {
        let h = self.window.height();
        let w = self.window.width();
        let mode = self.cursor().mode;
        self.window.rect(
            0,
            h as i32 - 18 - {
                if mode == Mode::Primitive(PrimitiveMode::Prompt) {
                    18
                } else {
                    0
                }
            },
            w,
            18,
            Color::rgba(74, 74, 74, 255),
        );

        self.draw_status_bar();

        for (n, c) in self.prompt[self.prompt_index].chars().enumerate() {
            self.window.char(
                n as i32 * 8,
                h as i32 - 16 - 1,
                c,
                Color::rgb(255, 255, 255),
            );
        }

        self.window.sync();
    }

    #[cfg(feature = "orbital")]
    fn draw_status_bar(&mut self) {
        let h = self.window.height();
        let w = self.window.width();

        let mode = self.cursor().mode;

        let current_title = self
            .buffers
            .current_buffer_info()
            .title
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("");

        let items = [
            (self.status_bar.mode, 0, 4),
            (current_title, 1, 4),
            (&self.status_bar.cmd, 2, 4),
            (&self.status_bar.msg, 3, 4),
        ];

        for &(text, a, b) in items.iter() {
            for (n, c) in (if text.len() as u32 > w / (8 * b) {
                text.chars()
                    .take((w / (8 * b) - 5) as usize)
                    .chain(vec!['.'; 3])
                    .collect::<Vec<_>>()
            } else {
                text.chars().collect()
            })
            .into_iter()
            .enumerate()
            {
                self.window.char(
                    ((w * a) / b) as i32 + (n as i32 * 8),
                    h as i32 - 16 - 1 - {
                        if mode == Mode::Primitive(PrimitiveMode::Prompt) {
                            16 + 1 + 1
                        } else {
                            0
                        }
                    },
                    c,
                    Color::rgb(255, 255, 255),
                );
            }
        }
    }
}

#[cfg(not(feature = "orbital"))]
impl Editor {
    /// Redraw the window
    pub fn redraw(&mut self) {}
    /// Redraw the status bar
    pub fn redraw_status_bar(&mut self) {}
}

/// The statubar (showing various info about the current state of the editor)
pub struct StatusBar {
    /// The current mode
    pub mode: &'static str,
    /// The current command
    pub cmd: String,
    /// A message (such as an error or other info to the user)
    pub msg: String,
}

impl StatusBar {
    /// Create new status bar
    pub fn new() -> Self {
        StatusBar {
            mode: "Normal",
            cmd: String::new(),
            msg: "Welcome to Sodium!".to_string(),
        }
    }
}
