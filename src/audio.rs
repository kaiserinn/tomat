use crate::{preferences::Preferences, sound};
use rodio::{Decoder, MixerDeviceSink, Player, Source};
use std::io::Cursor;

pub struct AudioManager {
    _sink: MixerDeviceSink,
    player: Player,
}

impl AudioManager {
    pub fn new() -> Self {
        let sink = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let player = Player::connect_new(sink.mixer());

        Self {
            _sink: sink,
            player,
        }
    }

    pub fn play(&mut self, preferences: &Preferences) {
        let Ok(data) = sound::resolve(&preferences.alert.sound) else {
            return;
        };

        let Ok(decoder) = Decoder::try_from(Cursor::new(data)) else {
            return;
        };

        if preferences.alert.repeat {
            self.player.append(
                decoder
                    .repeat_infinite()
                    .take_duration(preferences.alert.timeout),
            );
        } else {
            self.player.append(decoder);
        }
    }

    pub fn stop(&self) {
        self.player.stop();
    }

    pub fn is_empty(&self) -> bool {
        self.player.empty()
    }
}
