#[derive(Clone, Debug)]
pub struct Settings {
    pub pomodoro_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub long_break_interval: u32,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            pomodoro_duration: 25 * 60,
            short_break_duration: 5 * 60,
            long_break_duration: 20 * 60,
            long_break_interval: 4,
        }
    }
}
