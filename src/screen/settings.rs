use iced::Alignment;
use iced::{Length, widget};

use crate::icon;
use crate::settings;

pub struct Settings {
    settings: settings::Settings,
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
    PomodoroDurationChange(String),
    ShortBreakDurationChange(String),
    LongBreakDurationChange(String),
    LongBreakIntervalChange(String),
    Apply,
}

pub enum Action {
    None,
    Back,
    Apply(settings::Settings),
}

impl Settings {
    pub fn new(settings: settings::Settings) -> Self {
        Self { settings }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
            Message::Apply => Action::Apply(self.settings.clone()),
            Message::PomodoroDurationChange(c) => {
                self.settings.pomodoro_duration =
                    c.parse::<u64>().unwrap() * 60;

                Action::None
            }
            Message::ShortBreakDurationChange(c) => {
                self.settings.break_duration =
                    c.parse::<u64>().unwrap() * 60;

                Action::None
            }
            Message::LongBreakDurationChange(c) => {
                self.settings.long_break_duration =
                    c.parse::<u64>().unwrap() * 60;

                Action::None
            }
            Message::LongBreakIntervalChange(c) => {
                self.settings.pomodoro_count = c.parse().unwrap();

                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let back = widget::button(widget::row![
            icon::caret_left().size(20),
            widget::text("Back")
        ])
        .on_press(Message::Back)
        .style(widget::button::background);

        let pomodoro_duration = settings_item(
            "Pomodoro duration (minutes)",
            widget::text_input(
                "",
                &(self.settings.pomodoro_duration / 60).to_string(),
            )
            .on_input(|c| {
                if c.is_empty() || c.parse::<u64>().is_ok() {
                    Message::PomodoroDurationChange(c)
                } else {
                    Message::PomodoroDurationChange(
                        (self.settings.pomodoro_duration / 60).to_string(),
                    )
                }
            }),
        );

        let short_break_duration = settings_item(
            "Short break duration (minutes)",
            widget::text_input(
                "",
                &(self.settings.break_duration / 60).to_string(),
            )
            .on_input(|c| {
                if c.is_empty() || c.parse::<u64>().is_ok() {
                    Message::ShortBreakDurationChange(c)
                } else {
                    Message::ShortBreakDurationChange(
                        (self.settings.break_duration / 60).to_string(),
                    )
                }
            }),
        );

        let long_break_duration = settings_item(
            "Long break duration (minutes)",
            widget::text_input(
                "",
                &(self.settings.long_break_duration / 60).to_string(),
            )
            .on_input(|c| {
                if c.is_empty() || c.parse::<u64>().is_ok() {
                    Message::LongBreakDurationChange(c)
                } else {
                    Message::LongBreakDurationChange(
                        (self.settings.long_break_duration / 60).to_string(),
                    )
                }
            }),
        );

        let long_break_interval = settings_item(
            "Long break interval",
            widget::text_input(
                "",
                &self.settings.pomodoro_count.to_string(),
            )
            .on_input(|c| {
                if c.is_empty() || c.parse::<u64>().is_ok() {
                    Message::LongBreakIntervalChange(c)
                } else {
                    Message::LongBreakIntervalChange(
                        (self.settings.pomodoro_count / 60).to_string(),
                    )
                }
            }),
        );

        let column = widget::column![
            back,
            pomodoro_duration,
            short_break_duration,
            long_break_duration,
            long_break_interval,
        ]
        .spacing(16);

        let stack = widget::stack![
            column,
            widget::container(widget::button("Apply").on_press(Message::Apply))
                .align_bottom(Length::Fill)
                .align_right(Length::Fill)
        ]
        .height(Length::Fill);

        widget::container(stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn settings_item<'a>(
    label: &'a str,
    widget: impl Into<iced::Element<'a, Message>>,
) -> iced::Element<'a, Message> {
    widget::row![
        widget::text(label).width(Length::FillPortion(1)),
        widget::container(widget).width(Length::FillPortion(3)),
    ]
    .spacing(16)
    .align_y(Alignment::Center)
    .into()
}
