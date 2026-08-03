pub mod timer;
pub mod settings;

use timer::Timer;
use settings::Settings;

pub enum Screen {
    Timer(Timer),
    Settings(Settings),
}
