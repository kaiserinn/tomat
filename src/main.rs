use crate::screen::{Screen, settings, timer};
use iced::{Subscription, Task, widget};
use preferences::Preferences;

mod icon;
mod preferences;
mod screen;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tomat::new, Tomat::update, Tomat::view)
        .subscription(Tomat::subscription)
        .run()
}

struct Tomat {
    screen: Screen,
    timer_state: Option<timer::Timer>,
    preferences: Preferences,
    started: std::time::Instant,
}

#[derive(Clone, Debug)]
enum Message {
    StartupMeasured,
    Timer(timer::Message),
    Settings(settings::Message),
}

impl Tomat {
    fn new() -> Self {
        let preferences = Preferences::new();

        Self {
            screen: Screen::Timer(timer::Timer::new(preferences.clone())),
            timer_state: None,
            preferences,
            started: std::time::Instant::now(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartupMeasured => {
                println!("Startup time (event): {:?}", self.started.elapsed());

                Task::none()
            }
            Message::Timer(message) => {
                if let Screen::Timer(state) = &mut self.screen {
                    use timer::Action;

                    match state.update(message) {
                        Action::OpenSettings => {
                            if let Screen::Timer(state) = &self.screen {
                                self.timer_state = Some(state.clone());
                            }
                            self.screen = Screen::Settings(
                                settings::Settings::new(self.preferences.clone()),
                            );

                            Task::none()
                        }
                        Action::NextPhase(task) => {
                            task.map(Message::Timer)
                        }
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
                            let state = if let Some(mut state) =
                                self.timer_state.clone()
                            {
                                state.preferences = self.preferences.clone();
                                state.time_remaining =
                                    state.preferences.pomodoro_duration;
                                state
                            } else {
                                timer::Timer::new(self.preferences.clone())
                            };
                            self.screen = Screen::Timer(state);

                            Task::none()
                        }
                        Action::Apply(settings) => {
                            self.preferences = settings;

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
        let startup =
            iced::window::open_events().map(|_| Message::StartupMeasured);
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

        widget::container(content).padding(24).into()
    }
}
