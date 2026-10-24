use crate::screen::{Screen, settings, timer};
use iced::{
    Element, Font, Subscription, Task, Theme, font::{Family, Stretch}, widget::{container, svg}
};
use preferences::Preferences;

mod icon;
mod preferences;
mod screen;
mod audio;

pub const IOSEVKA: &[u8] =
    include_bytes!("../assets/fonts/Iosevka-Extended.ttf");

pub const REGULAR: Font = Font {
    family: Family::Name("Iosevka"),
    stretch: Stretch::Expanded,
    ..Font::DEFAULT
};

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tomat::new, Tomat::update, Tomat::view)
        .subscription(Tomat::subscription)
        .theme(Theme::CatppuccinMocha)
        .font(IOSEVKA)
        .default_font(REGULAR)
        .run()
}

struct Tomat {
    screen: Screen,
    timer_state: Option<timer::Timer>,
    preferences: Preferences,
}

#[derive(Clone, Debug)]
enum Message {
    Timer(timer::Message),
    Settings(settings::Message),
}

impl Tomat {
    fn new() -> Self {
        let mut preferences = Preferences::new();
        preferences.load();

        Self {
            screen: Screen::Timer(timer::Timer::new(&preferences)),
            timer_state: None,
            preferences,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Timer(message) => {
                if let Screen::Timer(state) = &mut self.screen {
                    use timer::Action;

                    match state.update(message, &self.preferences) {
                        Action::OpenSettings => {
                            let settings = settings::Settings::new(
                                self.preferences.clone(),
                            );

                            if let Screen::Timer(state) = std::mem::replace(
                                &mut self.screen,
                                Screen::Settings(settings),
                            ) {
                                self.timer_state = Some(state);
                            }

                            Task::none()
                        }
                        Action::Task(task) => task.map(Message::Timer),
                        Action::None => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Settings(message) => {
                if let Screen::Settings(state) = &mut self.screen {
                    use settings::Action;

                    match state.update(message) {
                        Action::Back => {
                            if let Some(timer_state) = self.timer_state.take() {
                                self.screen = Screen::Timer(timer_state);
                            }

                            Task::none()
                        }
                        Action::Apply(preferences) => {
                            self.preferences = preferences;
                            self.preferences.store();

                            if let Some(timer_state) = &mut self.timer_state {
                                timer_state
                                    .refresh_time_remaining(&self.preferences);
                            }

                            Task::none()
                        }
                        Action::None => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match &self.screen {
            Screen::Timer(state) => state.subscription().map(Message::Timer),
            Screen::Settings(_) => Subscription::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = match &self.screen {
            Screen::Timer(state) => {
                state.view(&self.preferences).map(Message::Timer)
            }
            Screen::Settings(state) => state.view().map(Message::Settings),
        };

        container(content).padding(24).into()
    }
}
