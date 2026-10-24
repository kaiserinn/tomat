use crate::screen::timer::Phase;
use std::{fs, time::Duration};

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

    fn config_path() -> std::path::PathBuf {
        std::env::home_dir()
            .unwrap()
            .join(".config/tomat/tomat.toml")
    }

    pub fn store(&self) {
        let serialized = format!(
            "\
pomodoro_duration = {}
break_duration = {}
long_break_duration = {}
pomodoro_count = {}
auto_start_enabled = {}
auto_start_delay = {}
alert_timeout = {}
alert_repeat = {}\
            ",
            self.pomodoro_duration.as_secs(),
            self.break_duration.as_secs(),
            self.long_break_duration.as_secs(),
            self.pomodoro_count,
            self.auto_start.enabled,
            self.auto_start.delay.as_secs(),
            self.alert.timeout.as_secs(),
            self.alert.repeat,
        );

        let path = Self::config_path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).unwrap();
        }
        std::fs::write(path, serialized).unwrap();
    }

    pub fn load(&mut self) {
        let path = Self::config_path();
        if !(fs::exists(&path).unwrap_or(false)) {
            return;
        }

        let content = fs::read_to_string(path).unwrap();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            let (key, value) = line.split_once("=").unwrap();
            let key = key.trim();
            let value = value.split("#").next().unwrap_or("").trim();

            match key {
                "pomodoro_duration" => {
                    self.pomodoro_duration =
                        Duration::from_secs(value.parse().unwrap())
                }
                "break_duration" => {
                    self.break_duration =
                        Duration::from_secs(value.parse().unwrap());
                }
                "long_break_duration" => {
                    self.long_break_duration =
                        Duration::from_secs(value.parse().unwrap());
                }
                "pomodoro_count" => {
                    self.pomodoro_count = value.parse().unwrap();
                }
                "auto_start_enabled" => {
                    self.auto_start.enabled = value.parse().unwrap();
                }
                "auto_start_delay" => {
                    self.auto_start.delay =
                        Duration::from_secs(value.parse().unwrap());
                }
                "alert_timeout" => {
                    self.alert.timeout =
                        Duration::from_secs(value.parse().unwrap());
                }
                "alert_repeat" => {
                    self.alert.repeat = value.parse().unwrap();
                }
                _ => (),
            }
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
