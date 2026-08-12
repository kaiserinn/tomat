use crate::preferences::Preferences;
use crate::{icon, icon::SvgExt};
use iced::{
    Alignment, Element, Length, padding,
    widget::{
        Row, button, center_x, column, container, row, stack, text, text_input,
        toggler,
    },
};
use std::time::Duration;

pub struct Settings {
    preferences: Preferences,
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
    PomodoroDurationChange(Option<u64>),
    BreakDurationChange(Option<u64>),
    LongBreakDurationChange(Option<u64>),
    PomodoroCountChange(Option<u64>),
    AutoStartEnabledChange(bool),
    AutoStartDelayChange(Option<u64>),
    Apply,
}

pub enum Action {
    None,
    Back,
    Apply(Preferences),
}

impl Settings {
    pub fn new(preferences: Preferences) -> Self {
        Self { preferences }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
            Message::Apply => Action::Apply(self.preferences.clone()),
            Message::PomodoroDurationChange(c) => {
                if let Some(c) = c {
                    self.preferences.pomodoro_duration =
                        Duration::from_secs(c * 60);
                }

                Action::None
            }
            Message::BreakDurationChange(c) => {
                if let Some(c) = c {
                    self.preferences.break_duration =
                        Duration::from_secs(c * 60);
                }

                Action::None
            }
            Message::LongBreakDurationChange(c) => {
                if let Some(c) = c {
                    self.preferences.long_break_duration =
                        Duration::from_secs(c * 60);
                }

                Action::None
            }
            Message::PomodoroCountChange(c) => {
                if let Some(c) = c {
                    self.preferences.pomodoro_count =
                        c.try_into().unwrap_or_default();
                }

                Action::None
            }
            Message::AutoStartEnabledChange(c) => {
                self.preferences.auto_start.enabled = c;

                Action::None
            }
            Message::AutoStartDelayChange(c) => {
                if let Some(c) = c {
                    self.preferences.auto_start.delay = Duration::from_secs(c);
                }

                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let back = button(
            row![icon!("chevron-left").width(20), text("Back")].spacing(4),
        )
        .padding(5)
        .on_press(Message::Back)
        .style(button::background);

        let main = {
            let pomodoro_duration = settings_item(
                "Pomodoro duration (minutes)",
                number_input(self.preferences.pomodoro_duration.as_secs() / 60)
                    .map(Message::PomodoroDurationChange),
            );

            let break_duration = settings_item(
                "Break duration (minutes)",
                number_input(self.preferences.break_duration.as_secs() / 60)
                    .map(Message::BreakDurationChange),
            );

            let long_break_duration = settings_item(
                "Long break duration (minutes)",
                number_input(
                    self.preferences.long_break_duration.as_secs() / 60,
                )
                .map(Message::LongBreakDurationChange),
            );

            let pomodoro_count = settings_item(
                "Pomodoro count per cycle",
                number_input(self.preferences.pomodoro_count.into())
                    .map(Message::PomodoroCountChange),
            );

            let auto_start = settings_item(
                "Auto start next phase",
                toggler(self.preferences.auto_start.enabled)
                    .size(24)
                    .on_toggle(Message::AutoStartEnabledChange),
            );

            let auto_start_delay = if self.preferences.auto_start.enabled {
                Some(
                    settings_item(
                        "Auto start delay (seconds)",
                        number_input(
                            self.preferences.auto_start.delay.as_secs(),
                        )
                        .map(Message::AutoStartDelayChange),
                    )
                    .padding(padding::left(40)),
                )
            } else {
                None
            };

            column![
                pomodoro_duration,
                break_duration,
                long_break_duration,
                pomodoro_count,
                auto_start,
                auto_start_delay,
            ]
            .spacing(16)
            .width(Length::Fill.max(800))
            .align_x(Alignment::Center)
        };

        let column = column![back, center_x(main)].spacing(16);

        let stack = stack![
            column,
            container(button("Apply").on_press(Message::Apply))
                .align_bottom(Length::Fill)
                .align_right(Length::Fill)
        ]
        .height(Length::Fill);

        container(stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn number_input<'a>(value: u64) -> Element<'a, Option<u64>> {
    row![
        button(icon!("minus").size(20))
            .on_press(Some(value.saturating_sub(1)))
            .style(button::subtle),
        text_input("", value.to_string())
            .width(80)
            .align_x(Alignment::Center)
            .on_input(move |c| {
                if c.is_empty() {
                    None
                } else if let Ok(v) = c.parse() {
                    Some(v)
                } else {
                    Some(value)
                }
            }),
        button(icon!("plus").size(20))
            .on_press(Some(value.saturating_add(1)))
            .style(button::subtle),
    ]
    .spacing(4)
    .into()
}

fn settings_item<'a>(
    label: &'a str,
    widget: impl Into<iced::Element<'a, Message>>,
) -> Row<'a, Message> {
    row![text(label).size(18).width(Length::Fill), container(widget)]
        .spacing(16)
        .align_y(Alignment::Center)
}
