use edit::buffer::TextBuffer;
use io::redraw::RedrawTask;
use state::editor::Editor;
use state::mode::{Mode, PrimitiveMode};

#[cfg(feature = "orbital")]
use orbclient::Color;

#[cfg(feature = "orbital")]
impl Editor {
    /// Redraw the window
    pub fn redraw(&mut self) {
        // TODO: Only draw when relevant for the window
        let (scroll_x, scroll_y) = {
            let current_buffer = self.buffers.current_buffer_info();

            (current_buffer.scroll_x, current_buffer.scroll_y)
        };
        let (pos_x, pos_y) = self.pos();
        // Redraw window
        self.window.set(Color::rgb(25, 25, 25));

        let w = self.window.width();

        if self.options.line_marker {
            self.window.rect(0,
                             (pos_y - scroll_y) as i32 * 16,
                             w,
                             16,
                             Color::rgb(45, 45, 45));
        }

        self.window.rect(8 * (pos_x - scroll_x) as i32,
                         16 * (pos_y - scroll_y) as i32,
                         8,
                         16,
                         Color::rgb(255, 255, 255));

        let mut string = false;


        for (y, row) in self.buffers.current_buffer().lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                // TODO: Move outta here
                let color = if self.options.highlight {
                    match c {
                        '\'' | '"' => {
                            string = !string;
                            (226, 225, 167) //(167, 222, 156)
                        }
                        _ if string => (226, 225, 167), //(167, 222, 156)
                        '!' |
                        '@' |
                        '#' |
                        '$' |
                        '%' |
                        '^' |
                        '&' |
                        '|' |
                        '*' |
                        '+' |
                        '-' |
                        '/' |
                        ':' |
                        '=' |
                        '<' |
                        '>' => (198, 83, 83), //(228, 190, 175), //(194, 106, 71),
                        '.' | ',' => (241, 213, 226),
                        '(' | ')' | '[' | ']' | '{' | '}' => (164, 212, 125), //(195, 139, 75),
                        '0' ... '9' => (209, 209, 177),
                        _ => (255, 255, 255),
                    }
                } else {
                    (255, 255, 255)
                };

                let c = if c == '\t' {
                    ' '
                } else {
                    c
                };

                if pos_x == x && pos_y == y {
                    self.window.char(8 * (x - scroll_x) as i32,
                                     16 * (y - scroll_y) as i32,
                                     c,
                                     Color::rgb(color.0 / 3, color.1 / 3, color.2 / 3));
                } else {
                    self.window.char(8 * (x - scroll_x) as i32,
                                     16 * (y - scroll_y) as i32,
                                     c,
                                     Color::rgb(color.0, color.1, color.2));
                }
            }
        }

        self.redraw_task = RedrawTask::None;


        self.redraw_status_bar();
        self.window.sync();
    }

    /// Redraw the status bar
    pub fn redraw_status_bar(&mut self) {
        let h = self.window.height();
        let w = self.window.width();
        let mode = self.cursor().mode;
        self.window.rect(0,
                         h as i32 - 18 -
                         {
                             if mode == Mode::Primitive(PrimitiveMode::Prompt) {
                                 18
                             } else {
                                 0
                             }
                         },
                         w,
                         18,
                         Color::rgba(74, 74, 74, 255));

        self.draw_status_bar();

        for (n, c) in self.prompt.chars().enumerate() {
            self.window.char(n as i32 * 8, h as i32 - 16 - 1, c, Color::rgb(255, 255, 255));
        }

        self.window.sync();
    }

    #[cfg(feature = "orbital")]
    fn draw_status_bar(&mut self) {
        let h = self.window.height();
        let w = self.window.width();

        let mode = self.cursor().mode;

        let current_title = 
            self.buffers.current_buffer_info().title.as_ref().map(|s| s.as_str()).unwrap_or("");

        let items = [
            (self.status_bar.mode, 0, 4),
            (current_title, 1, 4),
            (&self.status_bar.cmd, 2, 4),
            (&self.status_bar.msg, 3, 4)
        ];

        for &(text, a, b) in items.iter() {
            for (n, c) in (if text.len() as u32 > w / (8 * b) {
                              text.chars().take((w / (8 * b) - 5) as usize).chain(vec!['.'; 3]).collect::<Vec<_>>()
                          } else {
                              text.chars().collect()
                          })
                          .into_iter()
                          .enumerate() {

                self.window.char(((w * a) / b) as i32 + (n as i32 * 8),
                                    h as i32 - 16 - 1 -
                                    {
                                        if mode == Mode::Primitive(PrimitiveMode::Prompt) {
                                            16 + 1 + 1
                                        } else {
                                            0
                                        }
                                    },
                                    c,
                                    Color::rgb(255, 255, 255));
            }
        }
    }
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
