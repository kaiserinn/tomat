use iced::{
    Font,
    font::{Family, Stretch, Style, Weight},
};

pub const IOSEVKA: &[u8] =
    include_bytes!("../assets/fonts/Iosevka-Extended.ttf");

pub const IOSEVKA_BOLD: &[u8] =
    include_bytes!("../assets/fonts/Iosevka-ExtendedBold.ttf");

pub const IOSEVKA_ITALIC: &[u8] =
    include_bytes!("../assets/fonts/Iosevka-ExtendedItalic.ttf");

pub const REGULAR: Font = Font {
    family: Family::Name("Iosevka"),
    stretch: Stretch::Expanded,
    ..Font::DEFAULT
};

pub const BOLD: Font = Font {
    family: Family::Name("Iosevka"),
    stretch: Stretch::Expanded,
    weight: Weight::Bold,
    ..Font::DEFAULT
};

pub const ITALIC: Font = Font {
    family: Family::Name("Iosevka"),
    stretch: Stretch::Expanded,
    style: Style::Italic,
    ..Font::DEFAULT
};
