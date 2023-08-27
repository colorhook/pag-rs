use autocxx::prelude::*;
use cxx::UniquePtr;
use std::fmt;
use std::pin::Pin;

use crate::Matrix;
use crate::PAGComposition;
use crate::PAGLayer;
use crate::PAGRect;
use crate::PAGScaleMode;
use crate::PAGSurface;

pub struct PAGPlayer {
    pub ptr: UniquePtr<ffi::pag::PAGPlayer>,
}

impl fmt::Debug for PAGPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PAGPlayer").finish()
    }
}
impl fmt::Display for PAGPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PAGPlayer {
    pub fn new() -> Self {
        let ptr = ffi::pag::PAGPlayer::new().within_unique_ptr();
        // let raw =
        //     std::ops::Deref::deref(&ptr) as *const ffi::pag::PAGPlayer as *mut ffi::pag::PAGPlayer;
        // std::mem::forget(raw);
        Self { ptr }
    }

    fn pin_mut(&self) -> Pin<&mut ffi::pag::PAGPlayer> {
        let raw = std::ops::Deref::deref(&self.ptr) as *const ffi::pag::PAGPlayer
            as *mut ffi::pag::PAGPlayer;
        unsafe { Pin::new_unchecked(&mut *raw) }
    }

    pub fn get_composition(&self) -> PAGComposition {
        let ptr = self.pin_mut().getComposition();
        PAGComposition::from_ptr(ptr)
    }

    pub fn set_composition(&self, value: impl Into<PAGComposition>) {
        self.pin_mut().setComposition(value.into().ptr);
    }

    pub fn get_surface(&self) -> PAGSurface {
        let ptr = self.pin_mut().getSurface();
        PAGSurface { ptr }
    }

    pub fn set_surface(&self, value: &PAGSurface) {
        self.pin_mut().setSurface(value.ptr.clone());
    }

    pub fn video_enabled(&self) -> bool {
        self.pin_mut().videoEnabled()
    }

    pub fn set_video_enabled(&self, value: bool) {
        self.pin_mut().setVideoEnabled(value);
    }

    pub fn cache_enabled(&self) -> bool {
        self.pin_mut().cacheEnabled()
    }

    pub fn set_cache_enabled(&self, value: bool) {
        self.pin_mut().setCacheEnabled(value);
    }

    pub fn cache_scale(&self) -> f32 {
        self.pin_mut().cacheScale()
    }

    pub fn set_cache_scale(&self, value: f32) {
        self.pin_mut().setCacheScale(value);
    }

    pub fn max_frame_rate(&self) -> f32 {
        self.pin_mut().maxFrameRate()
    }

    pub fn set_max_frame_rate(&self, value: f32) {
        self.pin_mut().setMaxFrameRate(value);
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

    pub fn duration(&self) -> i64 {
        self.pin_mut().duration()
    }

    pub fn next_frame(&self) {
        self.pin_mut().nextFrame();
    }

    pub fn pre_frame(&self) {
        self.pin_mut().preFrame();
    }

    pub fn get_progress(&self) -> f64 {
        self.pin_mut().getProgress()
    }

    pub fn set_progress(&self, value: f64) {
        self.pin_mut().setProgress(value)
    }

    pub fn auto_clear(&self) -> bool {
        self.pin_mut().autoClear()
    }

    pub fn set_auto_clear(&self, value: bool) {
        self.pin_mut().setAutoClear(value);
    }

    pub fn prepare(&self) {
        self.pin_mut().prepare();
    }

    /*
    pub fn wait() {}
    */

    pub fn flush(&self) -> bool {
        self.pin_mut().flush()
    }

    pub fn get_bounds(&self, layer: &PAGLayer) -> PAGRect {
        ffi::pag::cxx_PAGPlayer_getBounds(self.pin_mut(), layer.ptr.clone())
    }

    pub fn rendering_time(&self) -> i64 {
        self.pin_mut().renderingTime()
    }

    pub fn image_decoding_time(&self) -> i64 {
        self.pin_mut().imageDecodingTime()
    }

    pub fn presenting_time(&self) -> i64 {
        self.pin_mut().presentingTime()
    }

    pub fn graphics_memory(&self) -> i64 {
        self.pin_mut().graphicsMemory()
    }
}
