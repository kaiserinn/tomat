use crate::screen::{Screen, settings::Action, timer};
use iced::{Subscription, widget};

mod icon;
mod screen;
mod settings;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tomat::new, Tomat::update, Tomat::view)
        .subscription(Tomat::subscription)
        .run()
}

struct Tomat {
    screen: Screen,
    timer_state: Option<timer::Timer>,
    settings: settings::Settings,
    started: std::time::Instant,
}

#[derive(Clone, Debug)]
enum Message {
    StartupMeasured,
    Timer(timer::Message),
    Settings(screen::settings::Message),
}

impl Tomat {
    fn new() -> Self {
        let settings = settings::Settings::new();

        Self {
            screen: Screen::Timer(timer::Timer::new(settings.clone())),
            timer_state: None,
            settings,
            started: std::time::Instant::now(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StartupMeasured => {
                println!("Startup time (event): {:?}", self.started.elapsed());
            }
            Message::Timer(message) => {
                if let Screen::Timer(state) = &mut self.screen {
                    match state.update(message) {
                        timer::Action::OpenSettings => {
                            if let Screen::Timer(state) = &self.screen {
                                self.timer_state = Some(state.clone());
                            }
                            self.screen = Screen::Settings(
                                screen::Settings::new(self.settings.clone()),
                            )
                        }
                        timer::Action::None => (),
                    }
                }
            }
            Message::Settings(message) => {
                if let Screen::Settings(state) = &mut self.screen {
                    match state.update(message) {
                        Action::Back => {
                            let state = if let Some(mut state) = self.timer_state.clone() {
                                state.settings = self.settings.clone();
                                state.remaining = state.settings.pomodoro_duration;
                                state
                            } else {
                                timer::Timer::new(self.settings.clone())
                            };
                            self.screen = Screen::Timer(state);
                        }
                        Action::Apply(settings) => {
                            self.settings = settings
                        },
                        Action::None => (),
                    }
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let startup = iced::window::open_events().map(|_| Message::StartupMeasured);
        let subscription = match &self.screen {
            Screen::Timer(state) => state.subscription().map(Message::Timer),
            Screen::Settings(_) => Subscription::none(),
        };

        Subscription::batch(vec![startup, subscription])
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let content = match &self.screen {
            Screen::Timer(state) => state.view().map(Message::Timer),
            Screen::Settings(state) => state.view().map(Message::Settings),
        };

        widget::container(widget::container(content).padding(24)).into()
    }
}
