use edit::buffer::{SplitBuffer, TextBuffer};
use io::graphics::StatusBar;
use io::key::{Cmd, Key};
use io::key_state::KeyState;
use io::parse::Inst;
use io::redraw::RedrawTask;
use state::cursor::Cursor;
use state::options::Options;
use std::slice::Iter;

#[cfg(feature = "orbital")]
use orbclient::Window;
#[cfg(feature = "orbital")]
use orbclient::WindowFlag;

use std::env::args;

const HELP: &'static str = include_str!("../../help.txt");

/// A SplitBuffer and related state
pub struct Buffer {
    /// The document
    pub raw_buffer: SplitBuffer,
    /// The current cursor
    pub current_cursor: u8,
    /// The cursors
    pub cursors: Vec<Cursor>,
    /// The x coordinate of the scroll
    pub scroll_x: usize,
    /// The y coordinate of the scroll
    pub scroll_y: usize,
    /// The title of the document
    pub title: Option<String>,
    /// True if the buffer is transient and should be deleted when
    /// it is no longer the current buffer.
    pub is_transient: bool,
}

impl Buffer {
    /// Create a new Buffer with default values.
    pub fn new() -> Buffer {
        Buffer {
            raw_buffer: SplitBuffer::new(),
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            scroll_x: 0,
            scroll_y: 0,
            title: None,
            is_transient: false,
        }
    }
}

impl From<SplitBuffer> for Buffer {
    fn from(b: SplitBuffer) -> Buffer {
        let mut info = Buffer::new();
        info.raw_buffer = b;

        info
    }
}

/// Provides access to buffer manipulation functions.
pub struct BufferManager {
    buffers: Vec<Buffer>,
    current_buffer_index: usize,
}

impl BufferManager {
    /// Create a new BufferManager with default values.
    pub fn new() -> BufferManager {
        BufferManager {
            buffers: vec![Buffer::new()],
            current_buffer_index: 0,
        }
    }

    /// Adds the specified buffer to the set of buffers and returns
    /// its index.
    pub fn new_buffer(&mut self, buffer: Buffer) -> usize {
        self.buffers.push(buffer);

        self.buffers.len() - 1
    }

    /// Returns an iterator over the buffers.
    pub fn iter(&self) -> Iter<Buffer> {
        self.buffers.iter()
    }

    /// Gets the number of buffers.
    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    /// Gets the index of the current buffer.
    pub fn current_buffer_index(&self) -> usize {
        self.current_buffer_index
    }

    /// Get a reference to the currently open buffer.
    pub fn current_buffer(&self) -> &SplitBuffer {
        &self.current_buffer_info().raw_buffer
    }

    /// Get a mutable reference to the currently open buffer.
    pub fn current_buffer_mut(&mut self) -> &mut SplitBuffer {
        &mut self.current_buffer_info_mut().raw_buffer
    }

    /// Get a reference to the currently open buffer information.
    pub fn current_buffer_info(&self) -> &Buffer {
        &self.buffers[self.current_buffer_index]
    }

    /// Get a mutable reference to the currently open buffer information.
    pub fn current_buffer_info_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_index]
    }

    /// Switch the current buffer to the specified buffer
    pub fn switch_to(&mut self, n: usize) {
        debug_assert!(n < self.buffers.len(), "Buffer index out of bounds");

        // if the current view is transient, delete it
        let mut n = n;
        if self.current_buffer_info().is_transient {
            let index = self.current_buffer_index;
            self.delete_buffer(index);

            // if the current view is less than the view to switch to
            // then we need to account for the view we just removed
            if index <= n {
                n -= 1;
            }
        }

        self.current_buffer_index = n;
    }

    /// Delete the specified buffer
    pub fn delete_buffer(&mut self, n: usize) {
        assert!(n < self.buffers.len(), "Buffer index out of bounds");

        self.buffers.remove(n);

        if self.buffers.len() == 0 {
            self.buffers.push(Buffer::new());
            self.current_buffer_index = 0;
        } else if n == 0 {
            self.current_buffer_index = 0;
        } else if self.current_buffer_index >= n {
            self.current_buffer_index -= 1;
        }
    }

    /// Validates that the specifed buffer index is valid
    pub fn is_buffer_index_valid(&self, n: usize) -> bool {
        n < self.buffers.iter().filter(|b| !b.is_transient).count()
    }
}

/// The current state of the editor, including the file, the cursor, the scrolling info, etc.
pub struct Editor {
    /// The buffers and related state
    pub buffers: BufferManager,
    /// The window
    #[cfg(feature = "orbital")]
    pub window: Window,
    /// The status bar
    pub status_bar: StatusBar,
    /// The prompt
    pub prompt: Vec<String>,
    /// The prompt index, usually 0
    pub prompt_index: usize,
    /// The settings
    pub options: Options,
    /// The key state
    pub key_state: KeyState,
    /// Redraw
    pub redraw_task: RedrawTask,
    /// The previous instruction
    pub previous_instruction: Option<Inst>,
    /// The character width in pixels
    pub char_width: usize,
    /// The character height in pixels
    pub char_height: usize,
    /// The files currently open
    pub files: Vec<String>,
}

impl Editor {
    /// Create new default state editor
    pub fn init() {
        #[cfg(feature = "orbital")]
        let window =
            Window::new_flags(-1, -1, 700, 500, &"Sodium", &[WindowFlag::Resizable]).unwrap();

        #[cfg(feature = "orbital")]
        let mut editor = Editor {
            buffers: BufferManager::new(),
            window: window,
            status_bar: StatusBar::new(),
            prompt: vec![String::new()],
            prompt_index: 0,
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::None,
            previous_instruction: None,
            char_width: 8,
            char_height: 16,
            files: Vec::new(),
        };

        #[cfg(not(feature = "orbital"))]
        let mut editor = Editor {
            buffers: BufferManager::new(),
            status_bar: StatusBar::new(),
            prompt: vec![String::new()],
            prompt_index: 0,
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::None,
            previous_instruction: None,
            char_width: 8,
            char_height: 16,
            files: Vec::new(),
        };

        let mut files: Vec<String> = Vec::new();

        let mut args_iter = args().skip(1).peekable();
        loop {
            let arg = match args_iter.next() {
                Some(x) => x,
                None => break,
            };

            match arg.as_str() {
                "--version" => {
                    println!(
                        "Sodium {}",
                        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
                    );
                    return;
                }
                "--help" => {
                    println!("{}", HELP);
                    return;
                }
                "-u" => {
                    unimplemented!();
                    /*
                    match args_iter.peek() {
                        Some(config) => {
                            // this is the config file to use for this session
                            println!("{}", config);
                        },
                        None => {
                            panic!("No config file specified.");
                        }
                    }
                    args_iter.next();
                    */
                }
                "--" => {
                    // everything from here on out is a file
                    for file in args_iter {
                        files.push(file);
                    }
                    editor.files = files.clone();
                    break;
                }
                _ => {
                    {
                        let mut arg_chars = arg.chars();
                        if arg_chars.next() == Some('-') {
                            for ch in arg_chars {
                                match ch {
                                    'R' => match editor.options.set("readonly") {
                                        Ok(_) => debugln!(editor, "Set readonly mode"),
                                        Err(_) => println!("Could not set readonly mode"),
                                    },
                                    'h' => {
                                        println!("{}", HELP);
                                        return;
                                    }
                                    _ => {
                                        unimplemented!();
                                    }
                                }
                            }

                            continue;
                        }
                    }

                    files.push(arg);
                    editor.files = files.clone()
                }
            }
        }

        if files.len() > 0 {
            // TODO: open multiple files into separate buffers
            editor.open(&files[0]);
        }

        debugln!(editor, "Starting Sodium");

        editor.redraw();

        debugln!(editor, "First redraw of the screen");

        loop {
            let inp = editor.get_inst();
            if let Inst(_, Cmd { key: Key::Quit }) = inp {
                debugln!(editor, "C'ya");
                break;
            }
            editor.exec(inp);
            editor.status_bar.mode = editor.cursor().mode.to_string();
            editor.redraw();
        }
    }

    /// Hint the buffer about the cursor position.
    pub fn hint(&mut self) {
        let x = self.cursor().x;
        let y = self.cursor().y;

        self.buffers.current_buffer_mut().focus_hint_y(y);
        self.buffers.current_buffer_mut().focus_hint_x(x);
    }
}
