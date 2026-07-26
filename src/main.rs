use iced::{Length, Subscription, Task, widget};
use std::time::Duration;

mod icon;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tomat::new, Tomat::update, Tomat::view)
        .subscription(Tomat::subscription)
        .run()
}

struct Tomat {
    remaining: u64,
    timer_state: TimerState,
    settings: Settings,
}

#[derive(Clone, Debug)]
enum Message {
    Tick,
    ToggleState,
    Reset,
}

impl Tomat {
    fn new() -> Self {
        let settings = Settings::new();

        Self {
            remaining: settings.pomodoro_duration,
            timer_state: TimerState::Stopped,
            settings,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Tick => {
                if self.remaining > 0 {
                    self.remaining -= 1;
                }
                Task::none()
            }
            Message::ToggleState => {
                self.timer_state.toggle();

                Task::none()
            }
            Message::Reset => {
                self.remaining = self.settings.pomodoro_duration;
                self.timer_state = TimerState::Stopped;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.timer_state.is_playing() {
            iced::time::every(Duration::from_millis(1000))
                .map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let hours = self.remaining / 3600;
        let minutes = (self.remaining % 3600) / 60;
        let seconds = self.remaining % 60;

        let time =
            widget::text(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
                .size(120);

        let main_container =
            widget::container(time).padding(24).center(Length::Fill);

        let mut controls = widget::row![
            widget::button(if self.timer_state.is_playing() {
                icon::pause().size(20)
            } else {
                icon::play().size(20)
            })
            .on_press(Message::ToggleState),
        ]
        .spacing(8);

        if !self.timer_state.is_stopped() {
            controls = controls.push(
                widget::button(icon::stop().size(20)).on_press(Message::Reset),
            )
        }

        let pin = widget::pin(controls)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let pin_container = widget::container(pin)
            .padding(24)
            .center_x(Length::Fill)
            .align_bottom(Length::Fill);

        widget::stack![main_container, pin_container].into()
    }
}

struct Settings {
    pomodoro_duration: u64,
}

impl Settings {
    fn new() -> Self {
        Self {
            pomodoro_duration: 25 * 60,
        }
    }
}

enum TimerState {
    Playing,
    Paused,
    Stopped,
}

impl TimerState {
    fn toggle(&mut self) {
        *self = match self {
            TimerState::Playing => TimerState::Paused,
            TimerState::Paused | TimerState::Stopped => TimerState::Playing,
        };
    }

    fn is_playing(&self) -> bool {
        matches!(self, TimerState::Playing)
    }

    fn is_stopped(&self) -> bool {
        matches!(self, TimerState::Stopped)
    }
}
