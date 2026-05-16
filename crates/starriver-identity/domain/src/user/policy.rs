#[derive(Debug, Clone, Copy)]
pub struct BadPasswordPolicy {
    pub window_minutes: u64,
    pub max_attempts: usize,
}

impl BadPasswordPolicy {
    pub fn new(window_minutes: u64, max_attempts: usize) -> Self {
        Self {
            window_minutes,
            max_attempts,
        }
    }
}
