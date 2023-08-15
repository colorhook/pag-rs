use crate::PAGImage;
use crate::PAGVideoRange;

define_pag_sub_class!(PAGImageLayer, PAGLayer);
define_pag_type_from!(PAGImageLayer, PAGLayer);
unsafe impl Send for PAGImageLayer {}
unsafe impl Sync for PAGImageLayer {}

impl PAGImageLayer {
    pub fn make(width: i32, height: i32, duration: i64) -> Self {
        let width = autocxx::c_int(width);
        let height = autocxx::c_int(height);
        let ptr = ffi::pag::PAGImageLayer::Make(width, height, duration);
        Self::from_ptr(ptr)
    }

    pub fn content_duration(&self) -> i64 {
        self.pin_mut().contentDuration()
    }

    pub fn get_video_ranges(&self) -> Vec<PAGVideoRange> {
        let ranges = self.pin_mut().getVideoRanges();
        ranges
            .iter()
            .map(|item| PAGVideoRange::from(item))
            .collect()
    }

    /**
     * @deprecated in C++
     */
    pub fn replace_image(&self, image: Option<PAGImage>) {
        let ptr = image.map_or(cxx::SharedPtr::null(), |v| v.ptr);
        self.pin_mut().replaceImage(ptr);
    }

    /**
     * 传递 None 会重置为默认 image
     */
    pub fn set_image(&self, image: Option<PAGImage>) {
        let ptr = image.map_or(cxx::SharedPtr::null(), |v| v.ptr);
        self.pin_mut().setImage(ptr);
    }

    pub fn layer_time_to_content(&self, time: i64) -> i64 {
        self.pin_mut().layerTimeToContent(time)
    }

    pub fn content_time_to_layer(&self, time: i64) -> i64 {
        self.pin_mut().contentTimeToLayer(time)
    }

    pub fn image_bytes(&self) -> &[u8] {
        let pointer = self.pin_mut().imageBytes();
        let raw: &ffi::pag::ByteData = unsafe { std::mem::transmute(pointer) };
        let ptr = raw.data() as *const u8;
        let size = raw.length();
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }
}
