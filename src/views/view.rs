use std::{fmt::Debug, io};

pub trait View: Debug {
    /// Draw the frame of the UI.
    fn draw(&mut self, frame: &mut ratatui::Frame);

    /// Handle events from the terminal.
    ///
    /// Returns `true` when the application should exit.
    fn handle_events(&mut self) -> io::Result<bool>;
}
