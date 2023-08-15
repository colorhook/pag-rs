use autocxx::prelude::*;
use cxx::{let_cxx_string, UniquePtr};
use std::fmt;

pub struct PAGFont {
    pub ptr: UniquePtr<ffi::pag::PAGFont>,
}

impl fmt::Debug for PAGFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PAGFont")
            .field("font_family", &self.font_family())
            .field("font_style", &self.font_style())
            .finish()
    }
}
impl fmt::Display for PAGFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PAGFont {
    pub fn new(font_family: &str, font_style: &str) -> Self {
        let ptr = ffi::pag::PAGFont::new(font_family, font_style);
        let ptr = ptr.within_unique_ptr();
        Self { ptr }
    }

    pub fn font_family(&self) -> String {
        ffi::pag::cxx_PAGFont_get_fontFamily(&self.ptr).to_string()
    }

    pub fn font_style(&self) -> String {
        ffi::pag::cxx_PAGFont_get_fontStyle(&self.ptr).to_string()
    }

    pub fn register_font_from_path(
        font_path: &str,
        ttc_index: i32,
        font_family: &str,
        font_style: &str,
    ) -> Self {
        let_cxx_string! {font_path = font_path};
        let_cxx_string! {font_family = font_family};
        let_cxx_string! {font_style = font_style};
        let ptr = ffi::pag::PAGFont::RegisterFont(
            &font_path,
            autocxx::c_int(ttc_index),
            &font_family,
            &font_style,
        );
        let ptr = ptr.within_unique_ptr();
        Self { ptr }
    }

    pub fn register_font_from_data<'a>(
        data: &'a [u8],
        ttc_index: i32,
        font_family: &str,
        font_style: &str,
    ) -> Self {
        let ptr = data.as_ptr() as *const autocxx::c_void;
        let size = data.len();
        let_cxx_string! {font_family = font_family};
        let_cxx_string! {font_style = font_style};
        let ptr = unsafe {
            ffi::pag::PAGFont::RegisterFont1(
                ptr,
                size,
                autocxx::c_int(ttc_index),
                &font_family,
                &font_style,
            )
        };
        let ptr = ptr.within_unique_ptr();
        Self { ptr }
    }

    pub fn set_fallback_font_names(arr: Vec<impl Into<String>>) {
        let mut cxx_vector = ffi::pag::make_cxx_string_vector();
        for item in arr {
            ffi::pag::cxx_string_vector_push(cxx_vector.pin_mut(), item.into());
        }
        if let Some(ptr) = cxx_vector.as_ref() {
            ffi::pag::PAGFont::SetFallbackFontNames(ptr);
        }
    }
}
