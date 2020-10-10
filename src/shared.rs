use crate::Settings;

use std::cell::RefCell;
use std::ops::Drop;
use std::sync::{Arc, atomic::{AtomicUsize, AtomicU8, Ordering::Relaxed}};



#[derive(Default)]
pub(crate) struct Shared {
    pub(crate) settings:   Settings,
    pub(crate) start_pos:  AtomicUsize,
    pub(crate) start_ch:   AtomicU8,
    pub(crate) pos:        AtomicUsize,
}

impl Shared {
    pub(crate) fn new(settings: &Settings) -> Self {
        Self {
            settings:   *settings,
            start_pos:  Default::default(),
            start_ch:   Default::default(),
            pos:        Default::default(),
        }
    }
}

thread_local! { static SHARED : RefCell<Option<Arc<Shared>>> = RefCell::new(None); }
pub(crate) fn settings()       -> Option<Settings> { SHARED.with(|s| s.borrow().as_ref().map(|s| s.settings)) }
pub(crate) fn start()          -> Option<(usize, char)> { SHARED.with(|s| s.borrow().as_ref().map(|s| (s.start_pos.load(Relaxed), s.start_ch.load(Relaxed) as char))) }
pub(crate) fn end()            -> Option<usize> { SHARED.with(|s| s.borrow().as_ref().map(|s| s.pos.load(Relaxed))) }



pub(crate) struct SharedStack(Option<Arc<Shared>>);

impl SharedStack {
    pub fn push(shared: Arc<Shared>) -> Self {
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
