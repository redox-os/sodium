use std::slice::Iter;
use edit::buffer::{Buffer, SplitBuffer};
use state::cursor::Cursor;
use io::graphics::StatusBar;
use io::key::{Key, Cmd};
use io::key_state::KeyState;
use state::options::Options;
use io::parse::Inst;
use io::redraw::RedrawTask;

#[cfg(feature = "orbital")]
use orbclient::Window;

use std::env::args;

/// A SplitBuffer and related state
pub struct BufferInfo {
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
}

impl BufferInfo {
    /// Create a new BufferInfo with default values.
    fn new() -> BufferInfo {
        BufferInfo {
            raw_buffer: SplitBuffer::new(),
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            scroll_x: 0,
            scroll_y: 0,
            title: None,
        }
    }
}

impl From<SplitBuffer> for BufferInfo {
    fn from(b: SplitBuffer) -> BufferInfo {
        let mut info = BufferInfo::new();
        info.raw_buffer = b;

        info
    }
}

/// Provides access to buffer manipulation functions.
pub struct BufferManager {
    buffers: Vec<BufferInfo>,
    current_buffer_index: usize,
}

impl BufferManager {
    /// Create a new BufferManager with default values.
    pub fn new() -> BufferManager {
        BufferManager {
            buffers: vec![BufferInfo::new()],
            current_buffer_index: 0,
        }
    }

    /// Adds the specified buffer to the set of buffers and returns
    /// its index.
    pub fn new_buffer(&mut self, buffer: BufferInfo) -> usize {
        self.buffers.push(buffer);

        self.buffers.len() -1
    }

    /// Returns an iterator over the buffers.
    pub fn iter(&self) -> Iter<BufferInfo> {
        self.buffers.iter()
    }

    /// Gets the number of buffers.
    pub fn len(&self) -> usize {
        self.buffers.len()
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
    pub fn current_buffer_info(&self) -> &BufferInfo {
        &self.buffers[self.current_buffer_index]
    }

    /// Get a mutable reference to the currently open buffer information.
    pub fn current_buffer_info_mut(&mut self) -> &mut BufferInfo {
        &mut self.buffers[self.current_buffer_index]
    }

    /// Switch the current buffer to the specified buffer
    pub fn switch_to(&mut self, n: usize) {
        assert!(n < self.buffers.len(), "Buffer index out of bounds");

        self.current_buffer_index = n;
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
    pub prompt: String,
    /// The settings
    pub options: Options,
    /// The key state
    pub key_state: KeyState,
    /// Redraw
    pub redraw_task: RedrawTask,
    /// The previous instruction
    pub previous_instruction: Option<Inst>,
}

impl Editor {
    /// Create new default state editor
    pub fn init() {

        #[cfg(feature = "orbital")]
        let window = Window::new(-1, -1, 700, 500, &"Sodium").unwrap();

        #[cfg(feature = "orbital")]
        let mut editor = Editor {
            buffers: BufferManager::new(),
            window: *window, // ORBITAL SPECIFIC!
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::None,
            previous_instruction: None,
        };

        #[cfg(not(feature = "orbital"))]
        let mut editor = Editor {
            buffers: BufferManager::new(),
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::None,
            previous_instruction: None,
        };

        if let Some(x) = args().skip(1).next() {
            editor.open(&x);
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
