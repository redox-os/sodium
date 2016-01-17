use editor::Editor;
use redraw::RedrawTask;
use mode::Mode;
use mode::PrimitiveMode;
use mode::CommandMode;
use buffer::{Buffer, Line};

#[cfg(feature = "orbital")]
use orbital::Color;

#[cfg(feature = "ansi")]
mod terminal {
	extern crate libc;

    use std::mem;
    use self::libc::{c_int, c_uint, c_ushort, c_uchar, STDOUT_FILENO};
    use self::libc::ioctl;

    use std::io::stdout;
    use std::io::prelude::*;

    extern {
        static tiocgwinsz: c_int;

		fn tcgetattr(filedes: c_int, termptr: *mut Termios) -> c_int;
		fn tcsetattr(filedes: c_int, opt: c_int, termptr: *mut Termios) -> c_int;
		fn cfmakeraw(termptr: *mut Termios);
    }

    #[repr(C)]
    struct TermSize {
        row: c_ushort,
        col: c_ushort,
        _x: c_ushort,
        _y: c_ushort,
    }

    pub fn termsize() -> Option<(usize, usize)> {
		unsafe {
			let mut size: TermSize = mem::zeroed();

			if ioctl(STDOUT_FILENO, tiocgwinsz as u64, &mut size as *mut _) == 0 {
				Some((size.col as usize, size.row as usize))
			} else {
				None
			}
		}
	}

    #[derive(Clone)]
	struct Termios {
		c_iflag: c_uint,
		c_oflag: c_uint,
		c_cflag: c_uint,
		c_lflag: c_uint,
		c_line: c_uchar,
		c_cc: [c_uchar; 32],
		c_ispeed: c_uint,
		c_ospeed: c_uint,
	}

	fn get_terminal_attr() -> (Termios, c_int) {
		unsafe {
			let mut ios = Termios {
				c_iflag: 0,
				c_oflag: 0,
				c_cflag: 0,
				c_lflag: 0,
				c_line: 0,
				c_cc: [0; 32],
				c_ispeed: 0,
				c_ospeed: 0
			};

			(ios, tcgetattr(0, &mut ios))
		}
	}

	fn make_raw(ios: &mut Termios) {
		unsafe {
			cfmakeraw(&mut *ios);
		}
	}

	fn set_terminal_attr(ios: *mut Termios) -> c_int {
		unsafe {
			tcsetattr(0, 0, ios)
		}
	}

	pub struct TerminalRestorer {
		prev_ios: Termios
	}

	impl Drop for TerminalRestorer {
		fn drop(&mut self) {
			set_terminal_attr(&mut self.prev_ios as *mut _);
		}
	}

	pub fn set_terminal_raw_mode() -> TerminalRestorer {
		let (ios, err) = get_terminal_attr();
        let prev_ios = ios.clone();
		if err != 0 {
			panic!("Can't load termios settings properly");
		}

		make_raw(&mut ios);

		if set_terminal_attr(&mut ios as *mut _) != 0 {
			panic!("Can't init termios raw mode properly");
		}

		TerminalRestorer {
			prev_ios: prev_ios,
		}
	}

    pub fn csi(b: &[u8]) {
        let stdout = stdout();

        stdout.write(b"\x1B[");
        stdout.write(b);
        stdout.flush();
    }

    pub fn print(b: &[u8]) {
        let stdout = stdout();

        stdout.write(b);
        stdout.flush();
    }

    pub fn clear() {
        csi(b"2J");
    }

    pub fn show_cursor() {
        csi(b"?25h");
    }

    pub fn hide_cursor() {
        csi(b"?25l");
    }

    pub fn reset_style() {
        csi(b"0m");
    }

    pub fn move_cursor(x: usize, y: usize) {
        csi(&[]);
        print(y.to_string().as_bytes());
        print(b";");
        print(x.to_string().as_bytes());
        print(b"H");
    }
}

#[cfg(feature = "orbital")]
impl<'a, B: Buffer<'a>> Editor<B> {
    /// Redraw the window
    pub fn redraw(&'a mut self) {
        // TODO: Only draw when relevant for the window
        let (mut pos_x, pos_y) = self.pos();
        // Redraw window
        self.window.set(Color::rgb(25, 25, 25));

        let w = self.window.width();

        if self.options.line_marker {
            self.window.rect(0,
                             (pos_y - self.scroll_y) as i32 * 16,
                             w,
                             16,
                             Color::rgb(45, 45, 45));
        }

        self.window.rect(8 * (pos_x - self.scroll_x) as i32,
                         16 * (pos_y - self.scroll_y) as i32,
                         8,
                         16,
                         Color::WHITE);

        let mut string = false;


        for (y, row) in self.buffer.lines().enumerate() {
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
                    self.window.char(8 * (x - self.scroll_x) as i32,
                                     16 * (y - self.scroll_y) as i32,
                                     c,
                                     Color::rgb(color.0 / 3, color.1 / 3, color.2 / 3));
                } else {
                    self.window.char(8 * (x - self.scroll_x) as i32,
                                     16 * (y - self.scroll_y) as i32,
                                     c,
                                     Color::rgb(color.0, color.1, color.2));
                }
            }
        }

        self.redraw_task = RedrawTask::Null;


        self.redraw_status_bar();
        self.window.sync();
    }

    /// Redraw the status bar
    pub fn redraw_status_bar(&'a mut self) {
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

        let sb_mode = self.status_bar.mode.clone();
        status_bar(self, sb_mode, 0, 4);
        let sb_file = self.status_bar.file.clone();
        status_bar(self, sb_file, 1, 4);
        let sb_cmd = self.status_bar.cmd.clone();
        status_bar(self, sb_cmd, 2, 4);
        let sb_msg = self.status_bar.msg.clone();
        status_bar(self, sb_msg, 3, 4);

        for (n, c) in self.prompt.chars().enumerate() {
            self.window.char(n as i32 * 8, h as i32 - 16 - 1, c, Color::WHITE);
        }

        self.window.sync();
    }
}

#[cfg(feature = "orbital")]
fn status_bar<'a, B: Buffer<'a>>(editor: &'a mut Editor<B>, text: String, a: u32, b: u32) {

    let h = editor.window.height();
    let w = editor.window.width();
    // let y = editor.y();
    let mode = editor.cursor().mode;

    for (n, c) in (if text.len() as u32 > w / (8 * b) {
                      text.chars().take((w / (8 * b) - 5) as usize).chain(vec!['.'; 3]).collect::<Vec<_>>()
                  } else {
                      text.chars().collect()
                  })
                  .into_iter()
                  .enumerate() {

        editor.window.char(((w * a) / b) as i32 + (n as i32 * 8),
                           h as i32 - 16 - 1 -
                           {
                               if mode == Mode::Primitive(PrimitiveMode::Prompt) {
                                   16 + 1 + 1
                               } else {
                                   0
                               }
                           },
                           c,
                           Color::WHITE);
    }
}

/// The statubar (showing various info about the current state of the editor)
pub struct StatusBar {
    /// The current mode
    pub mode: String,
    /// The cureent char
    pub file: String,
    /// The current command
    pub cmd: String,
    /// A message (such as an error or other info to the user)
    pub msg: String,
}

impl StatusBar {
    /// Create new status bar
    pub fn new() -> Self {
        StatusBar {
            mode: "Normal".to_string(),
            file: String::new(),
            cmd: String::new(),
            msg: "Welcome to Sodium!".to_string(),
        }
    }
}
