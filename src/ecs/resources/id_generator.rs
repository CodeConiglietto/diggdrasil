#[derive(Default)]
pub struct IdGeneratorResource {
    next: u64,
}

impl IdGeneratorResource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&mut self) -> u64 {
        let value = self.next;
        self.next += 1;
        value
    }
}
