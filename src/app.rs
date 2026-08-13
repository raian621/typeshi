use crate::views::{navigator::Navigator, typing::Typing, view::View};
use ratatui::{self};

use std::{collections::HashMap, io};

pub fn run_app() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
pub struct App {
    should_exit: bool,
    views: HashMap<String, Box<dyn View>>,
}

impl App {
    pub fn register_views(&mut self) {
        let navigator = Navigator::new(|_| ());
        self.views
            .insert("typing".into(), Box::new(Typing::new(navigator)));
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        self.register_views();
        while !self.should_exit {
            let current_view = self.views.get_mut("typing".into()).unwrap();
            terminal.draw(|frame| current_view.draw(frame))?;
            if current_view.handle_events()? {
                self.should_exit = true;
            }
        }
        Ok(())
    }
}
