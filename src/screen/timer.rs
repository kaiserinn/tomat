use crate::{
    audio::AudioManager, icon, icon::SvgExt, preferences::Preferences,
};
use iced::{
    Alignment::Center,
    Element, Event,
    Length::Fill,
    Subscription, Task, event,
    widget::{button, column, container, row, space::horizontal, stack, text},
    window,
};
use notify_rust::{NotificationHandle, Timeout};
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct Timer {
    time_remaining: Duration,
    status: TimerStatus,
    phase: Phase,
    session_count: u32,
    audio_manager: AudioManager,
    notification_handle: Option<NotificationHandle>,
    window_focused: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    TimerTick(Instant),
    AlertTick,
    ToggleTimer,
    Reset,
    OpenSettings,
    NextPhase,
    Run(Preferences),
    StopAlert,
    WindowFocused(bool),
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
            audio_manager: AudioManager::new(),
            notification_handle: None,
            window_focused: true,
        }
    }

    pub fn update(
        &mut self,
        message: Message,
        preferences: &Preferences,
    ) -> Action {
        match message {
            Message::TimerTick(now) => {
                if let TimerStatus::Running(target_time) = self.status {
                    self.time_remaining =
                        target_time.saturating_duration_since(now);
                }

                if self.time_remaining.is_zero() {
                    self.notify();
                    self.status = TimerStatus::Alert;
                    self.audio_manager.play(preferences);
                }

                Action::None
            }
            Message::AlertTick => {
                if self.audio_manager.is_empty() {
                    // TODO: Might want to encapsulate the next phase logic
                    //       in a function instead
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
                    TimerStatus::Alert => (),
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
            Message::StopAlert => {
                self.audio_manager.stop();

                if let Some(handle) = self.notification_handle.take() {
                    handle.close();
                }

                Action::None
            }
            Message::WindowFocused(focused) => {
                self.window_focused = focused;

                Action::None
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let tick_sub = if self.status.is_running() {
            iced::time::every(Duration::from_millis(500))
                .map(|_| Message::TimerTick(Instant::now()))
        } else if self.status.is_alert() {
            iced::time::every(Duration::from_millis(50))
                .map(|_| Message::AlertTick)
        } else {
            Subscription::none()
        };

        let window_focus_sub = event::listen_with(|event, _, _| match event {
            Event::Window(window::Event::Focused) => {
                Some(Message::WindowFocused(true))
            }
            Event::Window(window::Event::Unfocused) => {
                Some(Message::WindowFocused(false))
            }
            _ => None,
        });

        Subscription::batch([tick_sub, window_focus_sub])
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

    fn notify(&mut self) {
        if !self.window_focused {
            self.notification_handle = Some(
                notify_rust::Notification::new()
                    .summary("Timer Stopped")
                    .timeout(Timeout::Never)
                    .action("click", "Click")
                    .show()
                    .unwrap(),
            );
        }
    }

    pub fn refresh_time_remaining(&mut self, preferences: &Preferences) {
        if self.status.is_idle() {
            self.time_remaining = preferences.duration_for(self.phase);
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

        let controls = if !self.status.is_alert() {
            row![
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
                (!self.status.is_idle()).then(|| button(
                    icon!("stop").size(20).style(icon::primary)
                )
                .on_press(Message::Reset)),
                button(icon!("skip").size(20).style(icon::primary))
                    .on_press(Message::NextPhase),
            ]
            .spacing(8)
        } else {
            row![
                button(icon!("stop").size(20).style(icon::primary))
                    .on_press(Message::StopAlert)
            ]
        };

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
    Alert,
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

    fn is_alert(&self) -> bool {
        matches!(self, TimerStatus::Alert)
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
