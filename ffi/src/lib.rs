use std::fmt;

use autocxx::prelude::*;

#[cxx::bridge(namespace = "pag")]
pub mod shared {
    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub struct PAGMarker {
        pub start_time: i64,
        pub duration: i64,
        pub comment: String,
    }
}

// 包含 std::string 的类型无法使用 generate_pod! 生成 POD 类型
include_cpp! {
    #include "binding.h"
    safety!(unsafe_ffi)
    generate_pod!("pag::PAGRect")
    generate!("pag::make_cxx_int_vector")
    generate!("pag::make_cxx_string_vector")
    generate!("pag::cxx_int_vector_push")
    generate!("pag::cxx_string_vector_push")
    generate!("pag::cxx_PAGLayer_getMarkers")
    generate!("pag::cxx_PAGComposition_getAudioMarkers")
    generate!("pag::cxx_PAGComposition_getLayersByName")
    generate!("pag::cxx_PAGComposition_getLayersUnderPoint")
    generate!("pag::cxx_PAGLayer_getBounds")
    generate!("pag::cxx_PAGPlayer_getBounds")
    generate!("pag::cxx_PAGFile_getEditableIndices")
    generate!("pag::cxx_PAGFont_get_fontFamily")
    generate!("pag::cxx_PAGFont_get_fontStyle")
    generate!("pag::cxx_TextDocument_get_applyFill")
    generate!("pag::cxx_TextDocument_set_applyFill")
    generate!("pag::cxx_TextDocument_get_applyStroke")
    generate!("pag::cxx_TextDocument_set_applyStroke")
    generate!("pag::cxx_TextDocument_get_baselineShift")
    generate!("pag::cxx_TextDocument_get_boxText")
    generate!("pag::cxx_TextDocument_get_firstBaseLine")
    generate!("pag::cxx_TextDocument_get_fauxBold")
    generate!("pag::cxx_TextDocument_set_fauxBold")
    generate!("pag::cxx_TextDocument_get_fauxItalic")
    generate!("pag::cxx_TextDocument_set_fauxItalic")
    generate!("pag::cxx_TextDocument_get_fontSize")
    generate!("pag::cxx_TextDocument_set_fontSize")
    generate!("pag::cxx_TextDocument_get_fillColor")
    generate!("pag::cxx_TextDocument_set_fillColor")
    generate!("pag::cxx_TextDocument_get_strokeColor")
    generate!("pag::cxx_TextDocument_set_strokeColor")
    generate!("pag::cxx_TextDocument_get_fontFamily")
    generate!("pag::cxx_TextDocument_set_fontFamily")
    generate!("pag::cxx_TextDocument_get_fontStyle")
    generate!("pag::cxx_TextDocument_set_fontStyle")
    generate!("pag::cxx_TextDocument_get_strokeOverFill")
    generate!("pag::cxx_TextDocument_get_strokeWidth")
    generate!("pag::cxx_TextDocument_set_strokeWidth")
    generate!("pag::cxx_TextDocument_get_text")
    generate!("pag::cxx_TextDocument_set_text")
    generate!("pag::cxx_TextDocument_get_justification")
    generate!("pag::cxx_TextDocument_set_justification")
    generate!("pag::cxx_TextDocument_get_leading")
    generate!("pag::cxx_TextDocument_set_leading")
    generate!("pag::cxx_TextDocument_get_tracking")
    generate!("pag::cxx_TextDocument_set_tracking")
    generate!("pag::cxx_TextDocument_get_backgroundColor")
    generate!("pag::cxx_TextDocument_set_backgroundColor")
    generate!("pag::cxx_TextDocument_get_backgroundAlpha")
    generate!("pag::cxx_TextDocument_set_backgroundAlpha")
    generate!("pag::cxx_TextDocument_get_direction")
    generate!("pag::cxx_TextDocument_set_direction")
    generate!("pag::PAG")
    generate_pod!("pag::Color")
    generate_pod!("pag::Point")
    generate_pod!("pag::Matrix")
    generate_pod!("pag::PAGVideoRange")
    generate!("pag::ByteData")
    generate!("pag::TextDocument")
    generate!("pag::PAGFont")
    generate!("pag::PAGLayer")
    generate!("pag::PAGComposition")
    generate!("pag::PAGFile")
    generate!("pag::PAGPlayer")
    generate!("pag::PAGSurface")
    generate!("pag::PAGImage")
    generate!("pag::PAGImageLayer")
    generate!("pag::PAGSolidLayer")
    generate!("pag::PAGShapeLayer")
    generate!("pag::PAGTextLayer")
}

use ffi::pag::ColorType;
impl fmt::Debug for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ColorType::ALPHA_8 => "ALPHA_8",
            ColorType::RGBA_8888 => "RGBA_8888",
            ColorType::BGRA_8888 => "BGRA_8888",
            ColorType::RGB_565 => "RGB_565",
            ColorType::Gray_8 => "Gray_8",
            ColorType::RGBA_F16 => "RGBA_F16",
            ColorType::RGBA_1010102 => "RGBA_1010102",
            _ => "Unknown",
        };
        write!(f, "{:?}", name)
    }
}
impl fmt::Display for ffi::pag::ColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use ffi::pag::AlphaType;
impl fmt::Debug for AlphaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            AlphaType::Opaque => "Opaque",
            AlphaType::Premultiplied => "Premultiplied",
            AlphaType::Unpremultiplied => "Unpremultiplied",
            _ => "Unknown",
        };
        write!(f, "{:?}", name)
    }
}
impl fmt::Display for AlphaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use ffi::pag::LayerType;
impl Copy for LayerType {}

impl fmt::Debug for LayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            LayerType::Null => "Null",
            LayerType::Solid => "Solid",
            LayerType::Text => "Text",
            LayerType::Shape => "Shape",
            LayerType::Image => "Image",
            LayerType::PreCompose => "PreCompose",
            LayerType::Camera => "Camera",
            _ => "Unknown",
        };
        write!(f, "{:?}", name)
    }
}
impl fmt::Display for LayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<String> for LayerType {
    fn from(str: String) -> Self {
        match str.as_ref() {
            "Null" => LayerType::Null,
            "Solid" => LayerType::Solid,
            "Text" => LayerType::Text,
            "Shape" => LayerType::Shape,
            "Image" => LayerType::Image,
            "PreCompose" => LayerType::PreCompose,
            "Camera" => LayerType::Camera,
            _ => panic!("invalid layer type"),
        }
    }
}

use ffi::pag::Color;
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Color")
            .field("red", &self.red)
            .field("green", &self.green)
            .field("blue", &self.blue)
            .finish()
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Color {
    pub const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    pub const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
    };
    pub const RED: Color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    pub const GREEN: Color = Color {
        red: 0,
        green: 255,
        blue: 0,
    };
    pub const BLUE: Color = Color {
        red: 0,
        green: 0,
        blue: 255,
    };

    pub fn from_rgb(r: u8, b: u8, g: u8) -> Self {
        Self {
            red: r,
            blue: b,
            green: g,
        }
    }
    // 将 Color 转换成十六进制字符串
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    // 将十六进制字符串转换成 Color
    pub fn from_hex(s: &str) -> Option<Color> {
        if !s.starts_with('#') {
            return None;
        }
        let s = &s[1..];
        let (r, g, b) = match s.len() {
            3 => {
                let r = u8::from_str_radix(&s[0..1], 16).ok().map(|x| x * 17)?;
                let g = u8::from_str_radix(&s[1..2], 16).ok().map(|x| x * 17)?;
                let b = u8::from_str_radix(&s[2..3], 16).ok().map(|x| x * 17)?;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                (r, g, b)
            }
            _ => return None,
        };
        Some(Color::from_rgb(r, g, b))
    }
}

use ffi::pag::PAGRect;
impl fmt::Debug for PAGRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PAGRect")
            .field("left", &self.left)
            .field("top", &self.top)
            .field("right", &self.right)
            .field("bottom", &self.bottom)
            .finish()
    }
}
impl fmt::Display for PAGRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub use ffi::pag;
