use crate::views::{main_menu::MainMenu, navigator::Navigator, typing::Typing, view::View};
use ratatui::{self};

use std::{cell::RefCell, collections::HashMap, io, rc::Rc};

type ViewRef = Rc<RefCell<dyn View>>;

pub fn run_app() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

#[derive(Debug)]
pub struct App {
    _views: Rc<RefCell<HashMap<String, ViewRef>>>,
    default_view: ViewRef,
    current_view: Rc<RefCell<Option<ViewRef>>>,
}

impl App {
    pub fn new() -> Self {
        let views: Rc<RefCell<HashMap<String, ViewRef>>> = Rc::new(RefCell::new(HashMap::new()));
        let current_view: Rc<RefCell<Option<ViewRef>>> = Rc::new(RefCell::new(None));

        let navigator = Navigator::new({
            let views = Rc::downgrade(&views);
            let current_view = Rc::downgrade(&current_view);
            move |view_id: String| {
                let (Some(views), Some(current_view)) = (views.upgrade(), current_view.upgrade())
                else {
                    return;
                };
                if let Some(view) = views.borrow().get(&view_id) {
                    *current_view.borrow_mut() = Some(Rc::clone(view));
                }
            }
        });

        let default_view: ViewRef = Rc::new(RefCell::new(MainMenu::new(navigator.clone())));
        views
            .borrow_mut()
            .insert("menu".to_string(), default_view.clone());
        views.borrow_mut().insert(
            "typing".to_string(),
            Rc::new(RefCell::new(Typing::new(navigator.clone()))),
        );

        Self {
            _views: views,
            default_view,
            current_view,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        loop {
            let view_ref = {
                let current = self.current_view.borrow();
                current
                    .clone()
                    .unwrap_or_else(|| Rc::clone(&self.default_view))
            };
            let mut current_view = view_ref.borrow_mut();
            terminal.draw(|frame| current_view.draw(frame))?;
            if current_view.handle_events()? {
                break;
            }
        }
        Ok(())
    }
}
