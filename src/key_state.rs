use key::Key;
#[cfg(feature = "orbital")]
use orbital::{
    KeyEvent,
    K_ALT,
    K_CTRL,
    K_LEFT_SHIFT,
    K_RIGHT_SHIFT
};

/// Key state
pub struct KeyState {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    /// Feed the keystate
    pub fn feed(&mut self, k: KeyEvent) -> Option<Key> {

        let c = k.character;
        match c {
            '\0' => {
                // "I once lived here" - bug
                match k.scancode {
                    #[cfg(feature = "orbital")]
                    K_ALT => self.alt = k.pressed,
                    #[cfg(feature = "orbital")]
                    K_CTRL => self.ctrl = k.pressed,
                    #[cfg(feature = "orbital")]
                    K_LEFT_SHIFT | K_RIGHT_SHIFT => self.shift = k.pressed,
                    _ if k.pressed => {
                        return Some(Key::from_event(k));
                    }
                    _ => {}
                }
            }
            _ if k.pressed => {
                return Some(Key::from_event(k));
            }
            _ => {}
        }

        None
    }
}
