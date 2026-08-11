use std::time::Duration;
use crate::screen::timer::Phase;

#[derive(Clone, Debug)]
pub struct Preferences {
    pub pomodoro_duration: Duration,
    pub break_duration: Duration,
    pub long_break_duration: Duration,
    pub pomodoro_count: u32,
    pub auto_start: AutoStart,
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            pomodoro_duration: Duration::from_secs(25 * 60),
            break_duration: Duration::from_secs(5 * 60),
            long_break_duration: Duration::from_secs(20 * 60),
            pomodoro_count: 4,
            auto_start: AutoStart::new(false, 3),
        }
    }

    pub fn duration_for(&self, phase: Phase) -> Duration {
        match phase {
            Phase::Focus => self.pomodoro_duration,
            Phase::Break => self.break_duration,
            Phase::LongBreak => self.long_break_duration,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AutoStart {
    pub enabled: bool,
    pub delay: Duration,
}

impl AutoStart {
    pub fn new(enabled: bool, delay_in_secs: u64) -> Self {
        Self {
            enabled,
            delay: Duration::from_secs(delay_in_secs)
        }
    }
}
