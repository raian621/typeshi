use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::style::Stylize;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    core::{engine::Engine, tokens::TokenDiff},
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
        let mut engine = Engine::default();
        engine.generate_tokens(20);
        Self {
            _navigator: navigator,
            engine,
        }
    }
}

impl Widget for &mut Typing {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(" Press Ctrl + C to exit ");
        let token_diff = self.engine.token_diff();
        let mut token_display_diff = vec![];
        token_display_diff.resize(token_diff.len() * 2 - 1, Default::default());
        for i in 0..token_diff.len() - 1 {
            token_display_diff[2 * i] = token_diff[i].clone();
            token_display_diff[2 * i + 1] = TokenDiff::new(" ", "", "");
        }

        let body_text = Text::from(Line::from(
            token_display_diff
                .into_iter()
                .flat_map(|diff| {
                    vec![
                        Span::raw(diff.common),
                        Span::raw(diff.incorrect).red(),
                        Span::raw(diff.missing).gray(),
                    ]
                })
                .collect::<Vec<_>>(),
        ));

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
