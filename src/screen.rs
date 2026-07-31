pub mod timer;
pub mod settings;

pub use timer::Timer;
pub use settings::Settings;

pub enum Screen {
    Timer(Timer),
    Settings(Settings),
}
