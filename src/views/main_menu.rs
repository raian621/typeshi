use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    style::{Color, Modifier},
    widgets::{List, ListState},
};

use crate::views::{navigator::Navigator, view::View};

#[derive(Debug)]
pub struct MainMenu {
    navigator: Navigator,
    list_state: ListState,
    list_options: Vec<String>,
}

impl View for MainMenu {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let list = List::new(self.list_options.clone())
            .style(Color::White)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
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

impl MainMenu {
    fn handle_key_event(&mut self, event: KeyEvent) -> bool {
        match event.code {
            KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => true,
            KeyCode::Esc => true,
            KeyCode::Up => {
                self.list_state.select_previous();
                false
            }
            KeyCode::Down => {
                self.list_state.select_next();
                false
            }
            KeyCode::Enter => self.pick_selected_list_item(),
            _ => false,
        }
    }

    fn pick_selected_list_item(&self) -> bool {
        let selected = self.list_state.selected().unwrap_or(0);
        match selected {
            0 => {
                self.navigator.go_to("typing");
                false
            }
            _ => true,
        }
    }

    pub fn new(navigator: Navigator) -> Self {
        Self {
            navigator,
            list_state: ListState::default().with_selected(Some(0)),
            list_options: vec!["Typing Practice".to_string(), "Exit".to_string()],
        }
    }
}
