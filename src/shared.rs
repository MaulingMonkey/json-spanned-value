use crate::Settings;

use std::cell::{Cell, RefCell};
use std::ops::Drop;
use std::rc::Rc;



#[derive(Default)]
pub(crate) struct Shared {
    pub(crate) settings:   Settings,
    pub(crate) start:      Cell<(usize, u8)>,
    pub(crate) pos:        Cell<usize>,
}

impl Shared {
    pub(crate) fn new(settings: &Settings) -> Self {
        Self {
            settings:   *settings,
            start:      Default::default(),
            pos:        Default::default(),
        }
    }
}

thread_local! { static SHARED : RefCell<Option<Rc<Shared>>> = RefCell::new(None); }
pub(crate) fn settings()       -> Option<Settings> { SHARED.with(|s| s.borrow().as_ref().map(|s| s.settings)) }
pub(crate) fn start()          -> Option<(usize, char)> { SHARED.with(|s| s.borrow().as_ref().map(|s| s.start.get())).map(|(s,c)| (s, c as char)) }
pub(crate) fn end()            -> Option<usize> { SHARED.with(|s| s.borrow().as_ref().map(|s| s.pos.get())) }



pub(crate) struct SharedStack(Option<Rc<Shared>>);

impl SharedStack {
    pub fn push(shared: Rc<Shared>) -> Self {
        SHARED.with(|s| {
            let mut s = s.borrow_mut();
            Self(std::mem::replace(&mut *s, Some(shared)))
        })
    }
}

impl Drop for SharedStack {
    fn drop(&mut self) {
        SHARED.with(|s| std::mem::swap(&mut *s.borrow_mut(), &mut self.0));
    }
}
