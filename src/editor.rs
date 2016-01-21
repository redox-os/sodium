use cursor::Cursor;
use graphics::StatusBar;
use options::Options;
use key::{Key, Cmd};
use key_state::KeyState;
use redraw::RedrawTask;
use buffer::{Buffer, SplitBuffer};
use parse::Inst;

#[cfg(feature = "orbital")]
use orbital::Window;

/// The current state of the editor, including the file, the cursor, the scrolling info, etc.
pub struct Editor<B> {
    /// The current cursor
    pub current_cursor: u8,
    /// The cursors
    pub cursors: Vec<Cursor>,
    /// The buffer (document)
    pub buffer: B,
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

impl<'a, B: Buffer<'a>> Editor<B> {
    /// Create a new default `Editor`
    pub fn new() -> Self {

        #[cfg(feature = "orbital")]
        let window = Window::new(-1, -1, 700, 500, &"Sodium").unwrap();

        #[cfg(feature = "orbital")]
        let e = Editor {
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            buffer: B::new(),
            scroll_x: 0,
            scroll_y: 0,
            window: *window, // ORBITAL SPECIFIC!
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::Null,
        };

        #[cfg(not(feature = "orbital"))]
        let e = Editor {
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            buffer: B::new(),
            scroll_x: 0,
            scroll_y: 0,
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::Null,
        };

        e
    }

    /// Create new default state editor
    pub fn init() {

        let mut editor = Self::new();

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
            editor.redraw();
            editor.status_bar.mode = editor.cursor().mode.to_string();
        }
    }
}
