use std::{fmt, rc::Rc};

#[derive(Clone)]
pub struct Navigator {
    change_view_fn: Rc<dyn Fn(String)>,
}

impl fmt::Debug for Navigator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Navigator").finish_non_exhaustive()
    }
}

impl Navigator {
    pub fn new<F>(change_view_fn: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        Self {
            change_view_fn: Rc::new(change_view_fn),
        }
    }

    pub fn go_to(&self, view_id: &str) {
        (self.change_view_fn)(view_id.to_string());
    }
}
