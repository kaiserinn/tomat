use iced::widget::{
    Svg, svg,
    svg::{Handle, Style},
};

pub struct Icon<'a> {
    pub svg: Svg<'a, iced::Theme>,
}

impl<'a> Icon<'a> {
    fn new(handle: impl Into<Handle>) -> Self {
        Self {
            svg: svg(handle).style(move |theme: &iced::Theme, _| Style {
                color: Some(theme.palette().text),
            }),
        }
    }

    #[must_use]
    pub fn size(mut self, size: impl Into<iced::Length>) -> Self {
        let size = size.into();
        self.svg = self.svg.width(size);
        self.svg = self.svg.height(size);
        self
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<iced::Length>) -> Self {
        self.svg = self.svg.width(width);
        self
    }

    #[must_use]
    pub fn height(mut self, height: impl Into<iced::Length>) -> Self {
        self.svg = self.svg.height(height);
        self
    }

    #[must_use]
    pub fn color(mut self, color: iced::Color) -> Self {
        self.svg = self.svg.style(move |_, _| Style { color: Some(color) });
        self
    }
}

impl<'a, Message: 'a> From<Icon<'a>> for iced::Element<'a, Message> {
    fn from(icon: Icon<'a>) -> Self {
        icon.svg.into()
    }
}

pub fn play<'a>() -> Icon<'a> {
    Icon::new(Handle::from_memory(include_bytes!("../assets/play.svg")))
}

pub fn pause<'a>() -> Icon<'a> {
    Icon::new(Handle::from_memory(include_bytes!("../assets/pause.svg")))
}

pub fn stop<'a>() -> Icon<'a> {
    Icon::new(Handle::from_memory(include_bytes!("../assets/stop.svg")))
}
