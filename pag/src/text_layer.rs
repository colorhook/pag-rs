use std::string::ToString;

use autocxx::prelude::*;
use cxx::let_cxx_string;

use crate::PAGColor;
use crate::PAGFont;
use crate::TextDocument;

define_pag_sub_class!(PAGTextLayer, PAGLayer);
define_pag_type_from!(PAGTextLayer, PAGLayer);
define_pag_type_from!(PAGLayer, PAGTextLayer);

impl PAGTextLayer {
    pub fn make<T: ToString>(
        duration: i64,
        text: T,
        font_size: f32,
        font_family: T,
        font_style: T,
    ) -> Self {
        let ptr = ffi::pag::PAGTextLayer::Make(
            duration,
            &text.to_string(),
            font_size,
            &font_family.to_string(),
            &font_style.to_string(),
        );
        Self::from_ptr(ptr)
    }

    pub fn make_from_document(duration: i64, doc: TextDocument) -> Self {
        let ptr = ffi::pag::PAGTextLayer::Make1(duration, doc.ptr);
        Self::from_ptr(ptr)
    }

    pub fn fill_color(&self) -> PAGColor {
        self.pin_mut().fillColor()
    }

    pub fn set_fill_color(&self, color: &PAGColor) {
        self.pin_mut().setFillColor(color);
    }

    pub fn font(&self) -> PAGFont {
        let ptr = self.pin_mut().font().within_unique_ptr();
        PAGFont { ptr }
    }

    pub fn set_font(&self, font: &PAGFont) {
        if let Some(ptr) = font.ptr.as_ref() {
            self.pin_mut().setFont(ptr)
        }
    }

    pub fn font_size(&self) -> f32 {
        self.pin_mut().fontSize()
    }

    pub fn set_font_size(&self, size: f32) {
        self.pin_mut().setFontSize(size);
    }

    pub fn stroke_color(&self) -> PAGColor {
        self.pin_mut().strokeColor()
    }

    pub fn set_stroke_color(&self, color: &PAGColor) {
        self.pin_mut().setStrokeColor(color);
    }

    pub fn text(&self) -> String {
        let str = self.pin_mut().text();
        str.to_string()
    }

    pub fn set_text<T: ToString>(&self, text: T) {
        let_cxx_string! {text = text.to_string()};
        self.pin_mut().setText(&text);
    }

    pub fn reset(&self) {
        self.pin_mut().reset();
    }
}
