use crate::{icon, icon::SvgExt};
use iced::{
    Alignment, Background, Color, Element, Length, Theme,
    widget::{button, column, container, row, svg, text},
};

pub struct Alerts {
    alerts: Vec<Alert>,
}

#[derive(Clone)]
pub enum Alert {
    Error(String),
    Warning(String),
}

#[derive(Clone, Debug)]
pub enum Message {
    Dismiss(usize),
    ClearAll,
}

impl Alert {
    fn message(&self) -> &str {
        match self {
            Alert::Error(message) | Alert::Warning(message) => message,
        }
    }

    fn text_style(&self) -> fn(&Theme) -> text::Style {
        match self {
            Alert::Error(_) => text::danger,
            Alert::Warning(_) => text::warning,
        }
    }

    fn color(&self, theme: &Theme) -> Color {
        let seed = theme.seed();

        match self {
            Alert::Error(_) => seed.danger,
            Alert::Warning(_) => seed.warning,
        }
    }

    fn view(&self, index: usize) -> Element<'_, Message> {
        let dismiss =
            icon!("x").size(12).style(|theme: &Theme, _| svg::Style {
                color: Some(self.color(theme)),
            });

        row![
            text(self.message())
                .size(12)
                .width(Length::Fill)
                .style(self.text_style()),
            button(dismiss)
                .style(button::text)
                .on_press(Message::Dismiss(index)),
        ]
        .align_y(Alignment::Center)
        .into()
    }
}

impl Alerts {
    pub fn new() -> Self {
        Self { alerts: Vec::new() }
    }

    pub fn push(&mut self, alert: Alert) {
        self.alerts.push(alert);
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Dismiss(index) => {
                self.alerts.remove(index);
            }
            Message::ClearAll => self.alerts.clear(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let summary = row![
            text(format!("Encountered {} alerts", self.alerts.len()))
                .size(12)
                .width(Length::Fill)
                .style(text::danger),
            button(text("clear all").size(12).style(text::danger))
                .style(button::text)
                .on_press(Message::ClearAll),
        ]
        .align_y(Alignment::Center);

        container(
            column(
                self.alerts
                    .iter()
                    .enumerate()
                    .map(|(index, alert)| alert.view(index)),
            )
            .push((self.alerts.len() > 1).then_some(summary))
            .padding([4, 8]),
        )
        .style(|theme: &Theme| container::Style {
            background: Some(Background::Color(
                theme.palette().background.weakest.color,
            )),
            ..Default::default()
        })
        .into()
    }
}
