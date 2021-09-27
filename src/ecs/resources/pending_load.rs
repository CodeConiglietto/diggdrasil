#[derive(Default)]
pub struct PendingLoadResource {
    pub ids: Vec<u64>,
}

impl PendingLoadResource {
    pub fn new() -> Self {
        Self::default()
    }
}
