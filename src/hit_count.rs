use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

pub struct HitCount {
    pub count: Arc<AtomicUsize>,
}
