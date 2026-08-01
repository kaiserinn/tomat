use std::time::Duration;

use iced::{Alignment, Length, Subscription, Task, padding, widget};

use crate::{icon, settings::Settings};

#[derive(Clone, Debug)]
pub struct Timer {
    pub settings: Settings,
    pub time_remaining: u64,
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
    pub fn new(settings: Settings) -> Self {
        Self {
            time_remaining: settings.pomodoro_duration,
            status: TimerStatus::Idle,
            settings,
            phase: Phase::Focus,
            session_count: 0,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
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
                    && self.session_count < self.settings.pomodoro_count
                {
                    self.session_count += 1;
                } else if self.phase.is_long_break() {
                    self.session_count = 0;
                }

                self.phase = match self.phase {
                    Phase::Focus
                        if self.session_count
                            == self.settings.pomodoro_count =>
                    {
                        Phase::LongBreak
                    }
                    Phase::Focus => Phase::Break,
                    Phase::Break | Phase::LongBreak => Phase::Focus,
                };

                self.time_remaining = self.settings.duration_for(self.phase);
                self.status = TimerStatus::Idle;

                Action::None
            }
            Message::ToggleState => {
                self.status.toggle();

                Action::None
            }
            Message::Reset => {
                self.time_remaining = self.settings.pomodoro_duration;
                self.status = TimerStatus::Idle;

                Action::None
            }
            Message::OpenSettings => Action::OpenSettings,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.status.is_playing() {
            iced::time::every(Duration::from_millis(1000))
                .map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let hours = self.time_remaining / 3600;
        let minutes = (self.time_remaining % 3600) / 60;
        let seconds = self.time_remaining % 60;

        let time =
            widget::text(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
                .size(140);
        let phase = widget::text(self.phase.to_string()).size(20);

        let session_count: iced::Element<_> = if self.settings.pomodoro_count
            > 10
        {
            widget::text(format!(
                "{}/{}",
                self.session_count, self.settings.pomodoro_count
            ))
            .size(16)
            .into()
        } else {
            let mut row = widget::Row::new().spacing(16);
            for _ in 0..self.session_count {
                row = row.push(icon::circle_filled().size(16));
            }
            for _ in 0..(self.settings.pomodoro_count - self.session_count) {
                row = row.push(icon::circle_outline().size(16));
            }
            row.into()
        };

        let main_container = widget::container(
            widget::column![phase, time, session_count]
                .align_x(Alignment::Center),
        )
        .center(Length::Fill);

        let mut controls = widget::row![
            widget::button(if self.status.is_playing() {
                icon::pause().size(20)
            } else {
                icon::play().size(20)
            })
            .on_press(Message::ToggleState),
        ]
        .spacing(8);

        if !self.status.is_idle() {
            controls = controls.push(
                widget::button(icon::stop().size(20)).on_press(Message::Reset),
            )
        }

        controls = controls.push(
            widget::button(icon::skip().size(20)).on_press(Message::NextPhase),
        );

        let settings = widget::button(icon::settings().size(20))
            .on_press(Message::OpenSettings)
            .style(widget::button::background);

        let row = widget::row![
            widget::container(settings).width(Length::Fill),
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

    fn is_playing(&self) -> bool {
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
