use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub struct Sphere {
    id: usize,
}

impl Sphere {
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        return Sphere {
            id
        };
    }
}