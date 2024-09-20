use std::sync::atomic::AtomicU8;

pub(crate) struct Database(pub AtomicU8);

pub fn init_db() -> Database {
    Database(AtomicU8::new(3))
}
