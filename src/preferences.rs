use crate::screen::timer::Phase;

#[derive(Clone, Debug)]
pub struct Preferences {
    pub pomodoro_duration: u64,
    pub break_duration: u64,
    pub long_break_duration: u64,
    pub pomodoro_count: u32,
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            pomodoro_duration: 25 * 60,
            break_duration: 5 * 60,
            long_break_duration: 20 * 60,
            pomodoro_count: 4,
        }
    }

    pub fn duration_for(&self, phase: Phase) -> u64 {
        match phase {
            Phase::Focus => self.pomodoro_duration,
            Phase::Break => self.break_duration,
            Phase::LongBreak => self.long_break_duration,
        }
    }
}
