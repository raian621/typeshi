use std::io;
use ratatui::{
    self,
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Debug, Default)]
pub struct Menu {
    should_exit: bool,
}

impl Menu {
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(" Press Ctrl + Q to exit ");
        let block = Block::new().title_top(instructions);
        let body = Line::from("typeshi");

        Paragraph::new(body)
            .block(block)
            .wrap(Wrap { trim: true })
            .centered()
            .render(area, buf);
    }
}
