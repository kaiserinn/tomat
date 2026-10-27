use crate::{screen::timer::Phase, sound::{self, SoundId}};
use std::{fmt, fs, path::PathBuf, str::FromStr, time::Duration};

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

    pub fn config_path() -> Result<PathBuf, PreferencesError> {
        std::env::home_dir()
            .map(|path| path.join(".config/tomat"))
            .ok_or(PreferencesError::ConfigDirUnavailable)
    }

    pub fn store(&self) -> Result<(), PreferencesError> {
        let serialized = format!(
            "\
pomodoro_duration = {}
break_duration = {}
long_break_duration = {}
pomodoro_count = {}
auto_start_enabled = {}
auto_start_delay = {}
alert_timeout = {}
alert_repeat = {}
alert_sound = \"{}\"\
            ",
            self.pomodoro_duration.as_secs(),
            self.break_duration.as_secs(),
            self.long_break_duration.as_secs(),
            self.pomodoro_count,
            self.auto_start.enabled,
            self.auto_start.delay.as_secs(),
            self.alert.timeout.as_secs(),
            self.alert.repeat,
            self.alert.sound,
        );

        let config_dir = Self::config_path()?;
        fs::create_dir_all(&config_dir).map_err(|_| PreferencesError::Io {
            operation: "create config directory",
            path: config_dir.to_path_buf(),
        })?;
        std::fs::write(config_dir.join("tomat.toml"), serialized).map_err(
            |_| PreferencesError::Io {
                operation: "write config file",
                path: config_dir,
            },
        )
    }

    pub fn load(&mut self) -> Result<(), PreferencesError> {
        let path = Self::config_path()?.join("tomat.toml");

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(_) => {
                return Err(PreferencesError::Io {
                    operation: "read config file",
                    path: path.to_path_buf(),
                });
            }
        };

        for (i, raw_line) in content.lines().enumerate() {
            let line = i + 1;
            let raw_line = raw_line.trim();
            if raw_line.is_empty() || raw_line.starts_with("#") {
                continue;
            }
            let (key, value) = raw_line.split_once("=").ok_or(
                PreferencesError::InvalidLine {
                    line,
                    content: raw_line.to_string(),
                },
            )?;
            let key = key.trim();
            let value = value
                .split("#")
                .next()
                .unwrap_or(value)
                .trim()
                .trim_matches('"');

            match key {
                "pomodoro_duration" => {
                    let secs = Self::parse_value(line, key, value)?;
                    self.pomodoro_duration = Duration::from_secs(secs)
                }
                "break_duration" => {
                    let secs = Self::parse_value(line, key, value)?;
                    self.break_duration = Duration::from_secs(secs);
                }
                "long_break_duration" => {
                    let secs = Self::parse_value(line, key, value)?;
                    self.long_break_duration = Duration::from_secs(secs);
                }
                "pomodoro_count" => {
                    self.pomodoro_count = Self::parse_value(line, key, value)?;
                }
                "auto_start_enabled" => {
                    self.auto_start.enabled =
                        Self::parse_value(line, key, value)?;
                }
                "auto_start_delay" => {
                    let secs = Self::parse_value(line, key, value)?;
                    self.auto_start.delay = Duration::from_secs(secs);
                }
                "alert_timeout" => {
                    let secs = Self::parse_value(line, key, value)?;
                    self.alert.timeout = Duration::from_secs(secs);
                }
                "alert_repeat" => {
                    self.alert.repeat = Self::parse_value(line, key, value)?;
                }
                "alert_sound" => {
                    let (id_type, key) = value
                        .split_once(":")
                        .ok_or(PreferencesError::InvalidValue {
                            line,
                            key: key.to_string(),
                            value: value.to_string(),
                        })?;

                    self.alert.sound = if id_type == "custom" {
                        SoundId::Custom(key.to_string())
                    } else {
                        SoundId::Builtin(key.to_string())
                    }
                }

                // Add warning for unrecognized key?
                _ => (),
            }
        }

        Ok(())
    }

    fn parse_value<T: FromStr>(
        line: usize,
        key: &str,
        value: &str,
    ) -> Result<T, PreferencesError> {
        value.parse().map_err(|_| PreferencesError::InvalidValue {
            line,
            key: key.to_string(),
            value: value.to_string(),
        })
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
    pub sound: SoundId,
    pub timeout: Duration,
    pub repeat: bool,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            sound: sound::default_sound(),
            timeout: Duration::from_secs(15),
            repeat: true,
        }
    }
}

#[derive(Debug)]
pub enum PreferencesError {
    ConfigDirUnavailable,
    Io {
        operation: &'static str,
        path: PathBuf,
    },
    InvalidLine {
        line: usize,
        content: String,
    },
    InvalidValue {
        line: usize,
        key: String,
        value: String,
    },
}

impl fmt::Display for PreferencesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::ConfigDirUnavailable => {
                f.write_str("could not determine the home directory")
            }
            Self::Io {
                operation, path, ..
            } => {
                write!(f, "could not {operation} `{}`", path.display())
            }
            Self::InvalidLine { line, content } => {
                write!(
                    f,
                    "line {line}: expected `key = value`, got `{content}`"
                )
            }
            Self::InvalidValue {
                line, key, value, ..
            } => {
                write!(f, "line {line}: invalid value `{value}` for `{key}`")
            }
        }
    }
}
