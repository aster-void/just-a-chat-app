use std::sync::atomic::{AtomicU8, Ordering};

pub(crate) struct Database(AtomicU8);

impl Database {
    pub fn obtain(&self) -> u8 {
        self.0.load(Ordering::Relaxed)
    }
}

pub fn init_db(init: u8) -> Database {
    Database(AtomicU8::new(init))
}
