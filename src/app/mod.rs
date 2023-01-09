use std::io;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

pub struct App {
    pub help_visible: bool,
}

impl App {
    pub fn new() -> App {
        App {
            help_visible: false,
        }
    }

    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }
}
