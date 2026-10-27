use std::{borrow::Cow, fmt::Display, fs, path::PathBuf};
use crate::preferences::Preferences;

#[derive(Clone, PartialEq, Debug)]
pub enum SoundId {
    Builtin(String),
    Custom(String),
}

impl Display for SoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Builtin(s) => write!(f, "builtin:{s}"),
            Self::Custom(s) => write!(f, "custom:{s}"),
        }
    }
}

pub const BUILTIN_SOUNDS: &[BuiltinSound] = &[BuiltinSound {
    key: "wind-chime",
    bytes: include_bytes!("../assets/chime.wav"),
}];
pub const DEFAULT_BUILTIN: &str = "wind-chime";

pub fn builtin(key: &str) -> Option<&BuiltinSound> {
    BUILTIN_SOUNDS.iter().find(|v| v.key == key)
}

pub struct BuiltinSound {
    key: &'static str,
    bytes: &'static [u8],
}

#[derive(Debug)]
pub enum SoundError {
    SoundDirUnavailable,
    SoundNotFound,
}

pub fn sound_dir() -> Result<PathBuf, SoundError> {
    Preferences::config_path()
        .map(|path| path.join("sounds"))
        .map_err(|_| SoundError::SoundDirUnavailable)
}

pub fn default_sound() -> SoundId {
    SoundId::Builtin(builtin(DEFAULT_BUILTIN).unwrap().key.to_string())
}

pub fn entries() -> Result<Vec<SoundId>, SoundError> {
    let mut sounds: Vec<SoundId> = BUILTIN_SOUNDS
        .iter()
        .map(|v| SoundId::Builtin(v.key.to_string()))
        .collect();

    for entry in fs::read_dir(sound_dir()?).unwrap() {
        let entry = entry.unwrap().file_name().into_string().unwrap();
        sounds.push(SoundId::Custom(entry.clone()));
    }

    Ok(sounds)
}

pub fn resolve(id: &SoundId) -> Result<Cow<'static, [u8]>, SoundError> {
    match id {
        SoundId::Builtin(key) => builtin(key)
            .map(|v| Cow::Borrowed(v.bytes))
            .ok_or(SoundError::SoundNotFound),
        SoundId::Custom(name) => {
            let Ok(sound_dir) = sound_dir() else {
                return builtin(DEFAULT_BUILTIN)
                    .map(|v| Cow::Borrowed(v.bytes))
                    .ok_or(SoundError::SoundNotFound);
            };

            match fs::read(sound_dir.join(name)) {
                Ok(bytes) => Ok(Cow::Owned(bytes)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    builtin(DEFAULT_BUILTIN)
                        .map(|v| Cow::Borrowed(v.bytes))
                        .ok_or(SoundError::SoundNotFound)
                }
                Err(e) => panic!("error while resolving sound: {e}"),
            }
        }
    }
}
