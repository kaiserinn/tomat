use iced::{
    Length, Theme,
    widget::{
        Svg,
        svg::{Status, Style},
    },
};

pub trait SvgExt {
    fn size(self, size: impl Into<Length>) -> Self;
}

impl<'a> SvgExt for Svg<'a> {
    fn size(self, size: impl Into<Length>) -> Self {
        let size = size.into();
        self.width(size).height(size)
    }
}

#[macro_export]
macro_rules! icon {
    ( $path:literal ) => {
        icon!("/assets/icons/", $path)
    };
    ( $base:literal, $path:expr ) => {
        $crate::svg($crate::svg::Handle::from_memory(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            $base,
            $path,
            ".svg"
        ))))
        .style(|theme: &iced::Theme, _| $crate::svg::Style {
            color: Some(theme.palette().background.base.text),
        })
    };
}

pub fn primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();

    Style {
        color: Some(palette.primary.base.text),
    }
}
