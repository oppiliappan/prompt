use ansi_term::Style;
use std::fmt;

pub struct Module {
    action: fn() -> Option<String>,
    padding: (usize, usize),
    style: Style,
}

impl Module {
    pub fn new() -> Module {
        return Module {
            action: || None,
            padding: (0, 0),
            style: Style::default(),
        };
    }
    pub fn action(mut self, action: fn() -> Option<String>) -> Self {
        self.action = action;
        self
    }
    pub fn padding(mut self, left: usize, right: usize) -> Self {
        self.padding = (left, right);
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = (self.action)();
        let start = 1 as char;
        let end = 2 as char;
        match content {
            Some(c) => write!(
                f,
                "{l}{content}{r}",
                l = " ".repeat(self.padding.0),
                r = " ".repeat(self.padding.1),
                content = self.style.paint(c),
            ),
            None => write!(f, ""),
        }
    }
}
