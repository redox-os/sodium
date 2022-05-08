#[cfg(feature = "orbital")]
use io::key::Key;
#[cfg(feature = "orbital")]
use orbclient::KeyEvent;

#[cfg(feature = "ansi")]
use std::io::prelude::*;
#[cfg(feature = "ansi")]
use std::io::Stdin;

/// Key state
pub struct KeyState {
    /// Ctrl modifier.
    pub ctrl: bool,
    /// Alt modifier.
    pub alt: bool,
    /// Shift modifier.
    pub shift: bool,
}

impl KeyState {
    /// Create a new default key state.
    pub fn new() -> KeyState {
        KeyState {
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    /// Feed the keystate with a new key input.
    #[cfg(feature = "orbital")]
    pub fn feed(&mut self, k: KeyEvent) -> Option<Key> {
        use orbclient::{K_ALT, K_CTRL, K_LEFT_SHIFT, K_RIGHT_SHIFT};

        let c = k.character;
        match c {
            '\0' => {
                // "I once lived here" - bug
                match k.scancode {
                    K_ALT => self.alt = k.pressed,
                    K_CTRL => self.ctrl = k.pressed,
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
