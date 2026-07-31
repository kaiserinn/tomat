use std::time::Duration;

use iced::{Length, Subscription, widget};

use crate::{icon, settings::Settings};

#[derive(Clone, Debug)]
pub struct Timer {
    pub remaining: u64,
    pub timer_state: TimerState,
    pub settings: Settings,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick,
    ToggleState,
    Reset,
    OpenSettings,
}

pub enum Action {
    None,
    OpenSettings,
}

impl Timer {
    pub fn new(settings: Settings) -> Self {
        Self {
            remaining: settings.pomodoro_duration,
            timer_state: TimerState::Stopped,
            settings,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Tick => {
                if self.remaining > 0 {
                    self.remaining -= 1;
                }

                Action::None
            }
            Message::ToggleState => {
                self.timer_state.toggle();

                Action::None
            },
            Message::Reset => {
                self.remaining = self.settings.pomodoro_duration;
                self.timer_state = TimerState::Stopped;

                Action::None
            }
            Message::OpenSettings => Action::OpenSettings,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.timer_state.is_playing() {
            iced::time::every(Duration::from_millis(1000))
                .map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let hours = self.remaining / 3600;
        let minutes = (self.remaining % 3600) / 60;
        let seconds = self.remaining % 60;

        let time =
            widget::text(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
                .size(120);

        let main_container =
            widget::container(time).center(Length::Fill);

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

        let settings = widget::button(icon::settings().size(20))
            .on_press(Message::OpenSettings)
            .style(widget::button::background);

        let row = widget::row![
            widget::container(settings)
                .width(Length::Fill),
            controls,
            widget::space().width(Length::Fill)
        ];

        widget::stack![
            main_container,
            widget::container(row)
                .center_x(Length::Fill)
                .align_bottom(Length::Fill),
        ]
        .into()
    }
}

#[derive(Clone, Debug)]
pub enum TimerState {
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
