#[derive(Debug, Clone, Copy)]
pub struct BadPasswordPolicy {
    pub window_minutes: u16,
    pub max_attempts: u8,
    pub lockout_minutes: u16,
}

impl BadPasswordPolicy {
    pub fn new(window_minutes: u16, max_attempts: u8, lockout_minutes: u16) -> Self {
        Self {
            window_minutes,
            max_attempts,
            lockout_minutes,
        }
    }
}
