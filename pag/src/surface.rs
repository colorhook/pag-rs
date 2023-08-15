pub use autocxx::c_void;

use crate::PAGError;
use crate::{AlphaType, ColorType};

define_pag_class!(PAGSurface);

impl PAGSurface {
    pub fn make_offscreen(width: i32, height: i32) -> Result<Self, PAGError> {
        let width = autocxx::c_int(width);
        let height = autocxx::c_int(height);
        let ptr = ffi::pag::PAGSurface::MakeOffscreen(width, height);
        if ptr.is_null() {
            Err(PAGError::SurfaceError)
        } else {
            Ok(Self::from_ptr(ptr))
        }
    }

    pub fn width(&self) -> i32 {
        self.pin_mut().width().into()
    }

    pub fn height(&self) -> i32 {
        self.pin_mut().height().into()
    }

    pub fn update_size(&self) {
        self.pin_mut().updateSize();
    }

    pub fn free_cache(&self) {
        self.pin_mut().freeCache();
    }

    pub fn clear_all(&self) {
        self.pin_mut().clearAll();
    }

    pub fn read_pixels(
        &self,
        color_type: ColorType,
        alpha_type: AlphaType,
        dst_pixels: *mut c_void,
        dst_row_bytes: usize,
    ) {
        unsafe {
            self.pin_mut()
                .readPixels(color_type, alpha_type, dst_pixels, dst_row_bytes);
        }
    }

    // 扩展方法
    pub fn read_rgba(&self, data: &mut [u8]) {
        let size = (self.width() * self.height() * 4) as usize;
        assert!(data.len() == size, "data size not matched");
        let ptr = data.as_ptr() as *mut c_void;
        let row_bytes = (self.width() * 4) as usize;
        self.read_pixels(
            ColorType::RGBA_8888,
            AlphaType::Unpremultiplied,
            ptr,
            row_bytes,
        );
    }
}
