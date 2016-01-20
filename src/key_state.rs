use key::Key;
#[cfg(feature = "orbital")]
use orbital::KeyEvent;

#[cfg(feature = "ansi")]
use std::io::Stdin;
#[cfg(feature = "ansi")]
use std::io::prelude::*;

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
    #[cfg(feature = "orbital")]
    pub fn feed(&mut self, k: KeyEvent) -> Option<Key> {
        use orbital::{
            K_ALT,
            K_CTRL,
            K_LEFT_SHIFT,
            K_RIGHT_SHIFT
        };

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
                    },
                    _ => {},
                }

                None
            },
            _ if k.pressed => {
                Some(Key::from_event(k))
            },
            _ => None,
        }
    }
}
