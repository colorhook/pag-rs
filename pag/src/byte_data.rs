use cxx::{let_cxx_string, UniquePtr};
use std::fmt;
use std::slice;

pub struct ByteData {
    pub ptr: UniquePtr<ffi::pag::ByteData>,
}

impl fmt::Debug for ByteData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ByteData")
            .field("len", &self.data().len())
            .finish()
    }
}
impl fmt::Display for ByteData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ByteData {
    pub fn make(len: usize) -> Self {
        let ptr = ffi::pag::ByteData::Make(len);
        Self { ptr }
    }
    pub fn from_path<T: ToString>(path: T) -> Self {
        let_cxx_string!(file = path.to_string());
        let ptr = ffi::pag::ByteData::FromPath(&file);
        Self { ptr }
    }
    pub fn make_widthout_copy(data: &mut [u8]) -> Self {
        let ptr = data.as_mut_ptr() as *mut autocxx::c_void;
        let size = data.len();
        let ptr = unsafe { ffi::pag::ByteData::MakeWithoutCopy(ptr, size) };
        Self { ptr }
    }
    pub fn make_copy(data: &[u8]) -> Self {
        let ptr = data.as_ptr() as *const autocxx::c_void;
        let size = data.len();
        let ptr = unsafe { ffi::pag::ByteData::MakeCopy(ptr, size) };
        Self { ptr }
    }
    pub fn data(&self) -> &[u8] {
        let ptr = self.ptr.data() as *const u8;
        let size = self.ptr.length();
        unsafe { slice::from_raw_parts(ptr, size) }
    }
}
