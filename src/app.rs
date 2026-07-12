use crate::core::engine::Engine;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    self,
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use std::io;

pub fn run_app() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
pub struct App {
    should_exit: bool,
    engine: Engine,
}

impl App {
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_exit = true;
            }
            KeyCode::Char(c) => self.engine.push_char(c),
            KeyCode::Enter => self.engine.push_char('\n'),
            KeyCode::Backspace => self.engine.pop_char(),
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from("Press Ctrl + Q to exit");
        let token_diff = self.engine.token_diff();
        let body_text = Text::from(
            token_diff
                .split("\n")
                .map(Line::from)
                .collect::<Vec<Line>>(),
        );
        let block = Block::bordered()
            .title(instructions)
            .border_set(border::THICK);
        Paragraph::new(body_text)
            .block(block)
            .centered()
            .render(area, buf);
    }
}
