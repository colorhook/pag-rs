use crate::Opacity;
use crate::PAGColor;

define_pag_sub_class!(PAGSolidLayer, PAGLayer);
define_pag_type_from!(PAGSolidLayer, PAGLayer);

impl PAGSolidLayer {
    pub fn make(duration: i64, width: i32, height: i32, color: PAGColor, opacity: Opacity) -> Self {
        let ptr = ffi::pag::PAGSolidLayer::Make(duration, width, height, color, opacity);
        Self::from_ptr(ptr)
    }

    pub fn solid_color(&self) -> PAGColor {
        self.pin_mut().solidColor()
    }

    pub fn set_solid_color(&self, color: &PAGColor) {
        self.pin_mut().setSolidColor(color);
    }
}
