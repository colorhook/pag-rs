use std::convert::From;

pub type ID = u32;
pub type Opacity = u8;
pub type AlphaType = ffi::pag::AlphaType;
pub type ColorType = ffi::pag::ColorType;
pub type LayerType = ffi::pag::LayerType;
pub type Matrix = ffi::pag::Matrix;
pub type PAGColor = ffi::pag::Color;
pub type PAGRect = ffi::pag::PAGRect;
pub type PAGMarker = ffi::shared::PAGMarker;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PAGScaleMode {
    None = 0,
    Stretch,
    LetterBox,
    Zoom,
}

impl From<u8> for PAGScaleMode {
    fn from(v: u8) -> Self {
        match v {
            0 => PAGScaleMode::None,
            1 => PAGScaleMode::Stretch,
            2 => PAGScaleMode::LetterBox,
            3 => PAGScaleMode::Zoom,
            _ => panic!("invalid PAGScaleMode"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PAGTimeStretchMode {
    None = 0,
    Stretch,
    Repeat,
    RepeatInverted,
}

impl From<u8> for PAGTimeStretchMode {
    fn from(v: u8) -> Self {
        match v {
            0 => PAGTimeStretchMode::None,
            1 => PAGTimeStretchMode::Stretch,
            2 => PAGTimeStretchMode::Repeat,
            3 => PAGTimeStretchMode::RepeatInverted,
            _ => panic!("invalid PAGTimeStretchMode"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum ParagraphJustification {
    LeftJustify = 0,
    CenterJustify,
    RightJustify,
    FullJustifyLastLineLeft,
    FullJustifyLastLineRight,
    FullJustifyLastLineCenter,
    FullJustifyLastLineFull,
}

impl From<u8> for ParagraphJustification {
    fn from(v: u8) -> Self {
        match v {
            0 => ParagraphJustification::LeftJustify,
            1 => ParagraphJustification::CenterJustify,
            2 => ParagraphJustification::RightJustify,
            3 => ParagraphJustification::FullJustifyLastLineLeft,
            4 => ParagraphJustification::FullJustifyLastLineRight,
            5 => ParagraphJustification::FullJustifyLastLineCenter,
            6 => ParagraphJustification::FullJustifyLastLineFull,
            _ => panic!("invalid ParagraphJustification"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum TextDirection {
    Default = 0,
    Horizontal,
    Vertical,
}

impl From<u8> for TextDirection {
    fn from(v: u8) -> Self {
        match v {
            0 => TextDirection::Default,
            1 => TextDirection::Horizontal,
            2 => TextDirection::Vertical,
            _ => panic!("invalid TextDirection"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum BlendMode {
    Normal = 0,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    Add,
}

impl From<u8> for BlendMode {
    fn from(v: u8) -> Self {
        match v {
            0 => BlendMode::Normal,
            1 => BlendMode::Multiply,
            2 => BlendMode::Screen,
            3 => BlendMode::Overlay,
            4 => BlendMode::Darken,
            5 => BlendMode::Lighten,
            6 => BlendMode::ColorDodge,
            7 => BlendMode::ColorBurn,
            8 => BlendMode::HardLight,
            9 => BlendMode::SoftLight,
            10 => BlendMode::Difference,
            11 => BlendMode::Exclusion,
            12 => BlendMode::Hue,
            13 => BlendMode::Saturation,
            14 => BlendMode::Color,
            15 => BlendMode::Luminosity,
            16 => BlendMode::Add,
            _ => panic!("invalid BlendMode"),
        }
    }
}
