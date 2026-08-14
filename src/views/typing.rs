use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    core::engine::Engine,
    views::{navigator::Navigator, view::View},
};

#[derive(Debug)]
pub struct Typing {
    engine: Engine,
    _navigator: Navigator,
}

impl View for Typing {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press && self.handle_key_event(key_event) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Typing {
    fn handle_key_event(&mut self, event: KeyEvent) -> bool {
        match event.code {
            KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => true,
            KeyCode::Char(c) => {
                self.engine.push_char(c);
                false
            }
            KeyCode::Enter => {
                self.engine.push_char('\n');
                false
            }
            KeyCode::Tab => {
                self.engine.push_char('\t');
                false
            }
            KeyCode::Backspace => {
                self.engine.pop_char();
                false
            }
            _ => false,
        }
    }

    pub fn new(navigator: Navigator) -> Self {
        Self {
            _navigator: navigator,
            engine: Engine::default(),
        }
    }
}

impl Widget for &mut Typing {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(" Press Ctrl + C to exit ");
        let token_diff = self.engine.token_diff();
        let body_text = Text::from(
            token_diff
                .split("\n")
                .map(|line| Line::from(line.replace("\t", "  ")))
                .collect::<Vec<Line>>(),
        );

        let wpm = 0;
        let cpm = 0;
        let stats = Line::from(format!(" wpm = {wpm}, cpm = {cpm} "));
        let block = Block::bordered()
            .title_top(stats.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        Paragraph::new(body_text)
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
