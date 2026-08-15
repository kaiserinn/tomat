use rodio::{Decoder, MixerDeviceSink, Player, Source};
use std::io::Cursor;

use crate::preferences::Preferences;

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
        let decoded =
            Decoder::try_from(Cursor::new(preferences.alert.audio.clone()))
                .unwrap();

        if preferences.alert.repeat {
            self.player.append(
                decoded
                    .repeat_infinite()
                    .take_duration(preferences.alert.timeout),
            );
        } else {
            self.player.append(decoded);
        }
    }

    pub fn stop(&self) {
        self.player.stop();
    }

    pub fn is_empty(&self) -> bool {
        self.player.empty()
    }
}
