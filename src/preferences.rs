use crate::screen::timer::Phase;
use std::time::Duration;

const DEFAULT_ALERT: &[u8] = include_bytes!("../assets/chime.wav");

#[derive(Clone, Debug)]
pub struct Preferences {
    pub pomodoro_duration: Duration,
    pub break_duration: Duration,
    pub long_break_duration: Duration,
    pub pomodoro_count: u32,
    pub auto_start: AutoStartConfig,
    pub alert: AlertConfig,
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            pomodoro_duration: Duration::from_secs(25 * 60),
            break_duration: Duration::from_secs(5 * 60),
            long_break_duration: Duration::from_secs(20 * 60),
            pomodoro_count: 4,
            auto_start: AutoStartConfig::new(false, 3),
            alert: AlertConfig::default(),
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
pub struct AutoStartConfig {
    pub enabled: bool,
    pub delay: Duration,
}

impl AutoStartConfig {
    pub fn new(enabled: bool, delay_in_secs: u64) -> Self {
        Self {
            enabled,
            delay: Duration::from_secs(delay_in_secs),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlertConfig {
    pub audio: Vec<u8>,
    pub timeout: Duration,
    pub repeat: bool,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            audio: DEFAULT_ALERT.into(),
            timeout: Duration::from_secs(15),
            repeat: true,
        }
    }
}
