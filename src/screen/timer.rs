use crate::{icon, icon::SvgExt, preferences::Preferences};
use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    Subscription, Task,
    widget::{button, column, container, row, space::horizontal, stack, text},
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct Timer {
    time_remaining: Duration,
    status: TimerStatus,
    phase: Phase,
    session_count: u32,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tick(Instant),
    ToggleTimer,
    Reset,
    OpenSettings,
    NextPhase,
    Run(Preferences),
}

pub enum Action {
    None,
    OpenSettings,
    Task(Task<Message>),
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
            Message::Tick(now) => {
                if let TimerStatus::Running(target_time) = self.status {
                    self.time_remaining =
                        target_time.saturating_duration_since(now);
                }

                if self.time_remaining.is_zero() {
                    return Action::Task(Task::done(Message::NextPhase));
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

                if preferences.auto_start.enabled {
                    let preferences = preferences.clone();
                    let delay = preferences.auto_start.delay;

                    if !delay.is_zero() {
                        return Action::Task(Task::perform(
                            sleep(delay),
                            move |_| Message::Run(preferences),
                        ));
                    }

                    self.run(&preferences);
                }

                Action::None
            }
            Message::ToggleTimer => {
                match self.status {
                    TimerStatus::Idle | TimerStatus::Paused => {
                        self.run(preferences)
                    }
                    TimerStatus::Running(_) => self.pause(),
                };

                Action::None
            }
            Message::Reset => {
                self.time_remaining = preferences.duration_for(self.phase);
                self.status = TimerStatus::Idle;

                Action::None
            }
            Message::Run(preferences) => {
                self.run(&preferences);

                Action::None
            }
            Message::OpenSettings => {
                if self.status.is_running() {
                    self.pause();
                }

                Action::OpenSettings
            }
        }
    }

    pub fn pause(&mut self) {
        if let TimerStatus::Running(target_time) = self.status {
            self.time_remaining =
                target_time.saturating_duration_since(Instant::now());
            self.status = TimerStatus::Paused;
        }
    }

    pub fn run(&mut self, preferences: &Preferences) {
        if self.status.is_idle() {
            let duration = preferences.duration_for(self.phase);

            self.status = TimerStatus::Running(Instant::now() + duration)
        } else if self.status.is_paused() {
            self.status =
                TimerStatus::Running(Instant::now() + self.time_remaining)
        }
    }

    pub fn refresh_time_remaining(&mut self, preferences: &Preferences) {
        if self.status.is_idle() {
            self.time_remaining = preferences.duration_for(self.phase);
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.status.is_running() {
            iced::time::every(Duration::from_millis(500))
                .map(|_| Message::Tick(Instant::now()))
        } else {
            Subscription::none()
        }
    }

    pub fn view(&self, preferences: &Preferences) -> Element<'_, Message> {
        let remaining = self.time_remaining.as_secs_f32().ceil() as u32;

        let hours = remaining / 3600;
        let minutes = (remaining % 3600) / 60;
        let seconds = remaining % 60;

        let time = text(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
            .size(140);
        let phase = text(self.phase.to_string()).size(20);

        let session_count = if preferences.pomodoro_count > 10 {
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
            button(
                if self.status.is_running() {
                    icon!("pause")
                } else {
                    icon!("play")
                }
                .size(20)
                .style(icon::primary)
            )
            .on_press(Message::ToggleTimer),
            (!self.status.is_idle())
                .then(|| button(icon!("stop").size(20).style(icon::primary))
                    .on_press(Message::Reset)),
            button(icon!("skip").size(20).style(icon::primary))
                .on_press(Message::NextPhase)
        ]
        .spacing(8);

        let settings = button(icon!("settings").size(20))
            .on_press(Message::OpenSettings)
            .style(button::background);

        let footer =
            row![container(settings).width(Fill), controls, horizontal(),];

        stack![main, container(footer).center_x(Fill).align_bottom(Fill),]
            .into()
    }
}

fn session_dots<'a>(count: u32, total: u32) -> Element<'a, Message> {
    let dots = (0..total).map(|i| {
        (if i < count {
            icon!("circle-filled")
        } else {
            icon!("circle-outline")
        })
        .size(16)
        .into()
    });

    row(dots).spacing(16).into()
}

#[derive(Clone, Debug)]
enum TimerStatus {
    Running(Instant),
    Paused,
    Idle,
}

impl TimerStatus {
    fn is_running(&self) -> bool {
        matches!(self, TimerStatus::Running(_))
    }

    fn is_paused(&self) -> bool {
        matches!(self, TimerStatus::Paused)
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
