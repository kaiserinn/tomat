use crate::{icon, preferences::Preferences};
use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    Subscription, Task,
    widget::{button, column, container, row, space::horizontal, stack, text},
};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Timer {
    time_remaining: u64,
    status: TimerStatus,
    phase: Phase,
    session_count: u32,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick,
    ToggleState,
    Reset,
    OpenSettings,
    NextPhase,
}

pub enum Action {
    None,
    OpenSettings,
    NextPhase(Task<Message>),
}

impl Timer {
    pub fn new(preferences: &Preferences) -> Self {
        Self {
            time_remaining: preferences.pomodoro_duration,
            status: TimerStatus::Idle,
            phase: Phase::Focus,
            session_count: 0,
        }
    }

    pub fn update(
        &mut self,
        message: Message,
        preferences: &Preferences,
    ) -> Action {
        match message {
            Message::Tick => {
                if self.time_remaining > 0 {
                    self.time_remaining -= 1;
                }

                if self.time_remaining == 0 {
                    return Action::NextPhase(Task::done(Message::NextPhase));
                }

                Action::None
            }
            Message::NextPhase => {
                if self.phase.is_focus()
                    && self.session_count < preferences.pomodoro_count
                {
                    self.session_count += 1;
                } else if self.phase.is_long_break() {
                    self.session_count = 0;
                }

                self.phase = match self.phase {
                    Phase::Focus
                        if self.session_count == preferences.pomodoro_count =>
                    {
                        Phase::LongBreak
                    }
                    Phase::Focus => Phase::Break,
                    Phase::Break | Phase::LongBreak => Phase::Focus,
                };

                self.time_remaining = preferences.duration_for(self.phase);
                self.status = TimerStatus::Idle;

                Action::None
            }
            Message::ToggleState => {
                self.status.toggle();

                Action::None
            }
            Message::Reset => {
                self.time_remaining = preferences.duration_for(self.phase);
                self.status = TimerStatus::Idle;

                Action::None
            }
            Message::OpenSettings => Action::OpenSettings,
        }
    }

    pub fn refresh_time_remaining(&mut self, preferences: &Preferences) {
        if self.status.is_idle() {
            self.time_remaining = preferences.duration_for(self.phase);
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.status.is_running() {
            iced::time::every(Duration::from_millis(1000))
                .map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    pub fn view(&self, preferences: &Preferences) -> Element<'_, Message> {
        let hours = self.time_remaining / 3600;
        let minutes = (self.time_remaining % 3600) / 60;
        let seconds = self.time_remaining % 60;

        let time = text(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
            .size(140);
        let phase = text(self.phase.to_string()).size(20);

        let session_count: Element<_> = if preferences.pomodoro_count > 10 {
            text(format!(
                "{}/{}",
                self.session_count, preferences.pomodoro_count
            ))
            .size(16)
            .into()
        } else {
            session_dots(self.session_count, preferences.pomodoro_count)
        };

        let main =
            container(column![phase, time, session_count].align_x(Center))
                .center(Fill);

        let controls = row![
                button(if self.status.is_running() {
                    icon::pause().size(20)
                } else {
                    icon::play().size(20)
                }).on_press(Message::ToggleState),

                (!self.status.is_idle())
                    .then(|| button(icon::stop().size(20))
                        .on_press(Message::Reset)),

                button(icon::skip().size(20)).on_press(Message::NextPhase)
            ]
        .spacing(8);

        let settings = button(icon::settings().size(20))
            .on_press(Message::OpenSettings)
            .style(button::background);

        let footer =
            row![container(settings).width(Fill), controls, horizontal()];

        stack![main, container(footer).center_x(Fill).align_bottom(Fill),]
            .into()
    }
}

fn session_dots<'a>(count: u32, total: u32) -> Element<'a, Message> {
    let dots = (0..total).map(|i| {
        (if i < count {
            icon::circle_filled()
        } else {
            icon::circle_outline()
        })
        .size(16)
        .into()
    });

    row(dots).spacing(16).into()
}

#[derive(Clone, Debug)]
enum TimerStatus {
    Running,
    Paused,
    Idle,
}

impl TimerStatus {
    fn toggle(&mut self) {
        *self = match self {
            TimerStatus::Running => TimerStatus::Paused,
            TimerStatus::Paused | TimerStatus::Idle => TimerStatus::Running,
        };
    }

    fn is_running(&self) -> bool {
        matches!(self, TimerStatus::Running)
    }

    fn is_idle(&self) -> bool {
        matches!(self, TimerStatus::Idle)
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Phase {
    Focus,
    Break,
    LongBreak,
}

impl Phase {
    fn is_focus(&self) -> bool {
        matches!(self, Self::Focus)
    }

    fn is_long_break(&self) -> bool {
        matches!(self, Self::LongBreak)
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Phase::Focus => write!(f, "Focus"),
            Phase::Break => write!(f, "Break"),
            Phase::LongBreak => write!(f, "Long Break"),
        }
    }
}
