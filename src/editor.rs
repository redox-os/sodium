use buffer::{Buffer, SplitBuffer};
use cursor::Cursor;
use graphics::StatusBar;
use key::{Key, Cmd};
use key_state::KeyState;
use options::Options;
use parse::Inst;
use redraw::RedrawTask;

#[cfg(feature = "orbital")]
use orbclient::Window;

use std::env::args;

/// The current state of the editor, including the file, the cursor, the scrolling info, etc.
pub struct Editor {
    /// The current cursor
    pub current_cursor: u8,
    /// The cursors
    pub cursors: Vec<Cursor>,
    /// The buffer (document)
    pub buffer: SplitBuffer,
    /// The x coordinate of the scroll
    pub scroll_x: usize,
    /// The y coordinate of the scroll
    pub scroll_y: usize,
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
}

impl Editor {
    /// Create new default state editor
    pub fn init() {

        #[cfg(feature = "orbital")]
        let window = Window::new(-1, -1, 700, 500, &"Sodium").unwrap();

        #[cfg(feature = "orbital")]
        let mut editor = Editor {
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            buffer: SplitBuffer::new(),
            scroll_x: 0,
            scroll_y: 0,
            window: *window, // ORBITAL SPECIFIC!
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::None,
        };

        #[cfg(not(feature = "orbital"))]
        let mut editor = Editor {
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            buffer: SplitBuffer::new(),
            scroll_x: 0,
            scroll_y: 0,
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::Null,
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

    pub fn hint(&mut self) {
        let x = self.cursor().x;
        let y = self.cursor().y;

        self.buffer.focus_hint_y(y);
        self.buffer.focus_hint_x(x);
    }
}
