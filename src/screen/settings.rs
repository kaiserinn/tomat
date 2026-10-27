use crate::preferences::Preferences;
use crate::sound::{self, SoundId};
use crate::{icon, icon::SvgExt};
use iced::widget::pick_list;
use iced::{
    Alignment, Element, Length, padding,
    widget::{
        Row, button, center_x, column, container, row, stack, text, text_input,
        toggler,
    },
};
use iced::{Task, window};
use rfd::FileHandle;
use std::fs;
use std::time::Duration;

pub struct Settings {
    preferences: Preferences,
    sounds: Vec<SoundId>,
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
    AlertSoundChange(SoundId),
    AlertSoundDelete,
    AlertSoundChosen(Option<FileHandle>),
    AlertRepeatChange(bool),
    AlertTimeoutChange(Option<u64>),
    Browse,
    Apply,
}

pub enum Action {
    None,
    Back,
    Apply(Preferences),
    Task(Task<Message>),
}

impl Settings {
    pub fn new(preferences: Preferences) -> Self {
        Self {
            preferences,
            sounds: sound::entries().unwrap(),
        }
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
            Message::Browse => Action::Task(pick_audio_file()),
            Message::AlertSoundChange(choice) => {
                self.preferences.alert.sound = choice;

                Action::None
            }
            Message::AlertSoundDelete => {
                let SoundId::Custom(name) = &self.preferences.alert.sound
                else {
                    return Action::None;
                };

                // TODO: Alert
                if let Ok(path) = sound::sound_dir() {
                    _ = fs::remove_file(path.join(name));
                } else {
                    return Action::None;
                }

                self.sounds
                    .retain(|sound| sound != &self.preferences.alert.sound);
                self.preferences.alert.sound = sound::default_sound();

                Action::None
            }
            Message::AlertSoundChosen(file) => {
                if let Some(file) = file {
                    // TODO: handle error
                    let dest = sound::sound_dir().unwrap();
                    fs::create_dir_all(&dest).unwrap();
                    fs::copy(file.path(), dest.join(file.file_name())).unwrap();

                    let id = SoundId::Custom(file.file_name());
                    self.preferences.alert.sound = id.clone();
                    self.sounds.push(id);
                }

                Action::None
            }
            Message::AlertRepeatChange(c) => {
                self.preferences.alert.repeat = c;

                Action::None
            }
            Message::AlertTimeoutChange(c) => {
                if let Some(c) = c {
                    self.preferences.alert.timeout = Duration::from_secs(c);
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

            let alert_sound = {
                let is_custom =
                    matches!(self.preferences.alert.sound, SoundId::Custom(_));

                settings_item(
                    "Alert sound",
                    row![
                        is_custom.then_some(
                            button(icon!("x").size(16))
                                .padding(5)
                                .style(button::subtle)
                                .on_press(Message::AlertSoundDelete)
                        ),
                        pick_list(
                            Some(&self.preferences.alert.sound),
                            sound::entries().unwrap(),
                            |s| match s {
                                SoundId::Builtin(s) => s.clone(),
                                SoundId::Custom(s) => s.clone(),
                            },
                        )
                        .on_select(Message::AlertSoundChange),
                        button("browse").on_press(Message::Browse),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                )
            };

            let alert_repeat = settings_item(
                "Repeat alert sound",
                toggler(self.preferences.alert.repeat)
                    .size(24)
                    .on_toggle(Message::AlertRepeatChange),
            );

            let alert_timeout = if self.preferences.alert.repeat {
                Some(
                    settings_item(
                        "Alert timeout (seconds)",
                        number_input(self.preferences.alert.timeout.as_secs())
                            .map(Message::AlertTimeoutChange),
                    )
                    .padding(padding::left(40)),
                )
            } else {
                None
            };

            let alert_category = settings_category(
                "Alert",
                column![alert_sound, alert_repeat, alert_timeout].spacing(16),
            );

            column![
                pomodoro_duration,
                break_duration,
                long_break_duration,
                pomodoro_count,
                auto_start,
                auto_start_delay,
                alert_category,
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

fn pick_audio_file() -> Task<Message> {
    window::oldest()
        .and_then(|id| {
            window::run(id, |window| {
                rfd::AsyncFileDialog::new().set_parent(&window).pick_file()
            })
        })
        .then(Task::future)
        .map(Message::AlertSoundChosen)
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

fn settings_category<'a>(
    title: &'a str,
    items: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let items: Element<'a, Message> = items.into();

    column![text(title).size(16).style(text::secondary), items]
        .spacing(16)
        .padding([32, 0])
        .into()
}
