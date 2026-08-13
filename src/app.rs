use crate::views::{navigator::Navigator, typing::Typing, view::View};
use ratatui::{self};

use std::{cell::RefCell, collections::HashMap, io, rc::Rc};

pub fn run_app() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

#[derive(Debug)]
pub struct App {
    _views: HashMap<String, Rc<RefCell<dyn View>>>,
    default_view: Rc<RefCell<dyn View>>,
    current_view: Option<Rc<RefCell<dyn View>>>,
}

impl App {
    pub fn new() -> Self {
        let navigator = Navigator::new(|_| ());
        let default_view: Rc<RefCell<dyn View>> = Rc::new(RefCell::new(Typing::new(navigator)));
        Self {
            _views: HashMap::from([("typing".to_string(), default_view.clone())]),
            default_view,
            current_view: None,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        loop {
            let mut current_view = self
                .current_view
                .as_ref()
                .unwrap_or(&self.default_view)
                .borrow_mut();
            terminal.draw(|frame| current_view.draw(frame))?;
            if current_view.handle_events()? {
                break;
            }
        }
        Ok(())
    }
}
