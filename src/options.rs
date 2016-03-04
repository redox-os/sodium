pub struct Options {
    pub autoindent: bool,
    pub debug: bool,
    pub highlight: bool,
    pub line_marker: bool,
}

impl Options {
    /// Create new default options
    pub fn new() -> Self {
        Options {
            autoindent: true,
            debug: true, // TODO: Let this be `true` only in debug compilation cfg
            highlight: true,
            line_marker: true,
        }
    }

    /// Get the given option as a mutable reference
    pub fn get_mut(&mut self, name: &str) -> Option<&mut bool> {
        match name {
            "autoindent" | "ai" => Some(&mut self.autoindent),
            "debug" | "debug_mode" => Some(&mut self.debug),
            "hightlight" | "hl" => Some(&mut self.highlight),
            "line_marker" | "linemarker" | "linemark" | "lm" => Some(&mut self.line_marker),
            _ => None,
        }
    }

    /// Get a given option
    pub fn get(&self, name: &str) -> Option<bool> {
        match name {
            "autoindent" | "ai" => Some(self.autoindent),
            "debug" | "debug_mode" => Some(self.debug),
            "hightlight" | "hl" => Some(self.highlight),
            "line_marker" | "linemarker" | "linemark" | "lm" => Some(self.line_marker),
            _ => None,
        }
    }

    /// Set a given option (mark it as active)
    pub fn set(&mut self, name: &str) -> Result<(), ()> {
        match self.get_mut(name) {
            Some(x) => {
                *x = true;
                Ok(())
            }
            None => Err(()),
        }
    }
    /// Unset a given option (mark it as inactive)
    pub fn unset(&mut self, name: &str) -> Result<(), ()> {
        match self.get_mut(name) {
            Some(x) => {
                *x = false;
                Ok(())
            }
            None => Err(()),
        }
    }
    /// Toggle a given option
    pub fn toggle(&mut self, name: &str) -> Result<(), ()> {
        match self.get_mut(name) {
            Some(x) => {
                *x = !*x;
                Ok(())
            }
            None => Err(()),
        }
    }

}
