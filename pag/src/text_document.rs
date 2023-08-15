use crate::PAGColor;
use crate::ParagraphJustification;
use crate::TextDirection;

define_pag_class!(TextDocument);

impl TextDocument {
    // apply_fill
    pub fn apply_fill(&self) -> bool {
        ffi::pag::cxx_TextDocument_get_applyFill(self.pin_mut())
    }
    pub fn set_apply_fill(&self, value: bool) {
        ffi::pag::cxx_TextDocument_set_applyFill(self.pin_mut(), value);
    }

    // aplly_stroke
    pub fn apply_stroke(&self) -> bool {
        ffi::pag::cxx_TextDocument_get_applyStroke(self.pin_mut())
    }
    pub fn set_apply_stroke(&self, value: bool) {
        ffi::pag::cxx_TextDocument_set_applyStroke(self.pin_mut(), value);
    }

    // baseline_shift
    pub fn baseline_shift(&self) -> f32 {
        ffi::pag::cxx_TextDocument_get_baselineShift(self.pin_mut())
    }

    // box_text
    pub fn box_text(&self) -> bool {
        ffi::pag::cxx_TextDocument_get_boxText(self.pin_mut())
    }

    // first_baseline
    pub fn first_baseline(&self) -> f32 {
        ffi::pag::cxx_TextDocument_get_firstBaseLine(self.pin_mut())
    }

    // faux_bold
    pub fn faux_bold(&self) -> bool {
        ffi::pag::cxx_TextDocument_get_fauxBold(self.pin_mut())
    }
    pub fn set_faux_bold(&self, value: bool) {
        ffi::pag::cxx_TextDocument_set_fauxBold(self.pin_mut(), value);
    }

    // faux_italic
    pub fn faux_italic(&self) -> bool {
        ffi::pag::cxx_TextDocument_get_fauxItalic(self.pin_mut())
    }
    pub fn set_faux_italic(&self, value: bool) {
        ffi::pag::cxx_TextDocument_set_fauxItalic(self.pin_mut(), value);
    }

    // font_size
    pub fn font_size(&self) -> f32 {
        ffi::pag::cxx_TextDocument_get_fontSize(self.pin_mut())
    }
    pub fn set_font_size(&self, value: f32) {
        ffi::pag::cxx_TextDocument_set_fontSize(self.pin_mut(), value);
    }

    // fill_color
    pub fn fill_color(&self) -> PAGColor {
        ffi::pag::cxx_TextDocument_get_fillColor(self.pin_mut())
    }
    pub fn set_fill_color(&self, value: &PAGColor) {
        ffi::pag::cxx_TextDocument_set_fillColor(self.pin_mut(), value);
    }

    // stroke_color
    pub fn stroke_color(&self) -> PAGColor {
        ffi::pag::cxx_TextDocument_get_strokeColor(self.pin_mut())
    }
    pub fn set_stroke_color(&self, value: &PAGColor) {
        ffi::pag::cxx_TextDocument_set_strokeColor(self.pin_mut(), value);
    }

    // font_family
    pub fn font_family(&self) -> String {
        ffi::pag::cxx_TextDocument_get_fontFamily(self.pin_mut()).to_string()
    }
    pub fn set_font_family<T: ToString>(&self, value: T) {
        let value = value.to_string();
        ffi::pag::cxx_TextDocument_set_fontFamily(self.pin_mut(), value);
    }

    // font_style
    pub fn font_style(&self) -> String {
        ffi::pag::cxx_TextDocument_get_fontStyle(self.pin_mut()).to_string()
    }
    pub fn set_font_style<T: ToString>(&self, value: T) {
        let value = value.to_string();
        ffi::pag::cxx_TextDocument_set_fontStyle(self.pin_mut(), value);
    }

    // stroke_over_fill
    pub fn stroke_over_fill(&self) -> bool {
        ffi::pag::cxx_TextDocument_get_strokeOverFill(self.pin_mut())
    }

    // stroke_width
    pub fn stroke_width(&self) -> f32 {
        ffi::pag::cxx_TextDocument_get_strokeWidth(self.pin_mut())
    }
    pub fn set_stroke_width(&self, value: f32) {
        ffi::pag::cxx_TextDocument_set_strokeWidth(self.pin_mut(), value);
    }

    // text
    pub fn text(&self) -> String {
        ffi::pag::cxx_TextDocument_get_text(self.pin_mut()).to_string()
    }
    pub fn set_text<T: ToString>(&self, value: T) {
        let value = value.to_string();
        ffi::pag::cxx_TextDocument_set_text(self.pin_mut(), value);
    }

    // justification
    pub fn justification(&self) -> ParagraphJustification {
        let value = ffi::pag::cxx_TextDocument_get_justification(self.pin_mut());
        ParagraphJustification::from(value)
    }
    pub fn set_justification(&self, value: ParagraphJustification) {
        ffi::pag::cxx_TextDocument_set_justification(self.pin_mut(), value as u8);
    }

    // leading
    pub fn leading(&self) -> f32 {
        ffi::pag::cxx_TextDocument_get_leading(self.pin_mut())
    }
    pub fn set_leading(&self, value: f32) {
        ffi::pag::cxx_TextDocument_set_leading(self.pin_mut(), value);
    }

    // tracking
    pub fn tracking(&self) -> f32 {
        ffi::pag::cxx_TextDocument_get_tracking(self.pin_mut())
    }
    pub fn set_tracking(&self, value: f32) {
        ffi::pag::cxx_TextDocument_set_tracking(self.pin_mut(), value);
    }

    // background_color
    pub fn background_color(&self) -> PAGColor {
        ffi::pag::cxx_TextDocument_get_backgroundColor(self.pin_mut())
    }
    pub fn set_background_color(&self, value: &PAGColor) {
        ffi::pag::cxx_TextDocument_set_backgroundColor(self.pin_mut(), value);
    }

    // background_alpha
    pub fn background_alpha(&self) -> u8 {
        ffi::pag::cxx_TextDocument_get_backgroundAlpha(self.pin_mut())
    }
    pub fn set_background_alpha(&self, value: u8) {
        ffi::pag::cxx_TextDocument_set_backgroundAlpha(self.pin_mut(), value);
    }

    // direction
    pub fn direction(&self) -> TextDirection {
        let value = ffi::pag::cxx_TextDocument_get_direction(self.pin_mut());
        TextDirection::from(value)
    }
    pub fn set_direction(&self, value: TextDirection) {
        ffi::pag::cxx_TextDocument_set_direction(self.pin_mut(), value as u8);
    }
}
