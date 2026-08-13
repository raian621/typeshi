use std::fmt;

pub struct Navigator {
    _change_view_fn: Box<dyn Fn(String)>,
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
            _change_view_fn: Box::new(change_view_fn),
        }
    }
}
