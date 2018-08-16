use std::sync::atomic::AtomicUsize;

pub struct HitCount {
    pub count: AtomicUsize,
}
