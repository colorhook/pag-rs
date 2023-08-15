use cxx::let_cxx_string;

use crate::AlphaType;
use crate::ColorType;
use crate::Matrix;
use crate::PAGError;
use crate::PAGScaleMode;
use crate::ID;

define_pag_class!(PAGImage);

impl PAGImage {
    pub fn from_path<T: ToString>(path: T) -> Result<Self, PAGError> {
        let_cxx_string!(file = path.to_string());
        let ptr = ffi::pag::PAGImage::FromPath(&file);
        if ptr.is_null() {
            Err(PAGError::ImageError)
        } else {
            Ok(Self::from_ptr(ptr))
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PAGError> {
        let ptr = bytes.as_ptr() as *const autocxx::c_void;
        let size = bytes.len();
        let ptr = unsafe { ffi::pag::PAGImage::FromBytes(ptr, size) };
        if ptr.is_null() {
            Err(PAGError::ImageError)
        } else {
            Ok(Self::from_ptr(ptr))
        }
    }

    pub fn from_pixels(
        pixels: *const autocxx::c_void,
        width: i32,
        height: i32,
        row_bytes: usize,
        color_type: ColorType,
        alpha_type: AlphaType,
    ) -> Result<Self, PAGError> {
        let width = autocxx::c_int(width);
        let height = autocxx::c_int(height);
        let ptr = unsafe {
            ffi::pag::PAGImage::FromPixels(pixels, width, height, row_bytes, color_type, alpha_type)
        };
        if ptr.is_null() {
            Err(PAGError::ImageError)
        } else {
            Ok(Self::from_ptr(ptr))
        }
    }

    pub fn unique_id(&self) -> ID {
        self.pin_mut().uniqueID()
    }

    pub fn width(&self) -> i32 {
        self.pin_mut().width().into()
    }

    pub fn height(&self) -> i32 {
        self.pin_mut().height().into()
    }

    pub fn scale_mode(&self) -> PAGScaleMode {
        let sm: i32 = self.pin_mut().scaleMode().into();
        PAGScaleMode::from(sm as u8)
    }

    pub fn set_scale_mode(&self, value: PAGScaleMode) {
        self.pin_mut().setScaleMode(autocxx::c_int(value as i32));
    }

    pub fn matrix(&self) -> Matrix {
        self.pin_mut().matrix()
    }

    pub fn set_matrix(&self, matrix: &Matrix) {
        self.pin_mut().setMatrix(matrix);
    }
}
