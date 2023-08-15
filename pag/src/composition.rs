use cxx::let_cxx_string;

use crate::PAGLayer;
use crate::PAGMarker;

define_pag_sub_class!(PAGComposition, PAGLayer);
define_pag_type_from!(PAGComposition, PAGLayer);
define_pag_type_from!(PAGLayer, PAGComposition);

unsafe impl Send for PAGComposition {}
unsafe impl Sync for PAGComposition {}

impl PAGComposition {
    pub fn make(width: i32, height: i32) -> Self {
        let width = autocxx::c_int(width);
        let height = autocxx::c_int(height);
        let ptr = ffi::pag::PAGComposition::Make(width, height);
        Self::from_ptr(ptr)
    }

    pub fn width(&self) -> i32 {
        self.pin_mut().width().into()
    }

    pub fn height(&self) -> i32 {
        self.pin_mut().height().into()
    }

    pub fn set_content_size(&self, width: i32, height: i32) {
        let width = autocxx::c_int(width);
        let height = autocxx::c_int(height);
        self.pin_mut().setContentSize(width, height);
    }

    pub fn num_children(&self) -> i32 {
        self.pin_mut().numChildren().into()
    }

    pub fn get_layer_at(&self, index: i32) -> Option<PAGLayer> {
        let ptr = self.pin_mut().getLayerAt(autocxx::c_int(index));
        if ptr.is_null() {
            None
        } else {
            Some(PAGLayer::from_ptr(ptr))
        }
    }

    pub fn get_layer_index(&self, layer: impl Into<PAGLayer>) -> i32 {
        self.pin_mut().getLayerIndex(layer.into().ptr).into()
    }

    pub fn set_layer_index(&self, layer: PAGLayer, index: i32) {
        self.pin_mut()
            .setLayerIndex(layer.ptr, autocxx::c_int(index));
    }

    pub fn add_layer(&self, layer: impl Into<PAGLayer>) -> bool {
        self.pin_mut().addLayer(layer.into().ptr)
    }

    pub fn add_layer_at(&self, layer: impl Into<PAGLayer>, index: i32) -> bool {
        self.pin_mut()
            .addLayerAt(layer.into().ptr, autocxx::c_int(index))
    }

    pub fn contains(&self, layer: impl Into<PAGLayer>) -> bool {
        self.pin_mut().contains(layer.into().ptr)
    }

    pub fn remove_layer(&self, layer: impl Into<PAGLayer>) -> Option<PAGLayer> {
        let ptr = self.pin_mut().removeLayer(layer.into().ptr);
        if ptr.is_null() {
            None
        } else {
            Some(PAGLayer::from_ptr(ptr))
        }
    }

    pub fn remove_layer_at(&self, index: i32) -> Option<PAGLayer> {
        let ptr = self.pin_mut().removeLayerAt(autocxx::c_int(index));
        if ptr.is_null() {
            None
        } else {
            Some(PAGLayer::from_ptr(ptr))
        }
    }

    pub fn remove_all_layers(&self) {
        self.pin_mut().removeAllLayers();
    }

    pub fn swap_layer(&self, layer1: impl Into<PAGLayer>, layer2: impl Into<PAGLayer>) {
        self.pin_mut()
            .swapLayer(layer1.into().ptr, layer2.into().ptr);
    }

    pub fn swap_layer_at(&self, index1: i32, index2: i32) {
        self.pin_mut()
            .swapLayerAt(autocxx::c_int(index1), autocxx::c_int(index2));
    }

    pub fn audio_bytes(&self) -> Option<&[u8]> {
        let pointer = self.pin_mut().audioBytes();
        if pointer.is_null() {
            None
        } else {
            let raw: &ffi::pag::ByteData = unsafe { std::mem::transmute(pointer) };
            let ptr = raw.data() as *const u8;
            let size = raw.length();
            let data = unsafe { std::slice::from_raw_parts(ptr, size) };
            Some(data)
        }
    }

    pub fn audio_markers(&self) -> Vec<PAGMarker> {
        let markers = ffi::pag::cxx_PAGComposition_getAudioMarkers(self.pin_mut());
        markers
            .into_iter()
            .map(|i| {
                let marker: &PAGMarker = unsafe { std::mem::transmute(i) };
                marker.clone()
            })
            .collect()
    }

    pub fn audio_start_time(&self) -> i64 {
        self.pin_mut().audioStartTime().into()
    }

    pub fn get_layers_by_name(&self, name: impl Into<String>) -> Vec<PAGLayer> {
        let_cxx_string! {name = name.into()}
        let indices = ffi::pag::cxx_PAGComposition_getLayersByName(self.pin_mut(), &name);
        indices
            .iter()
            .filter_map(|i| self.get_layer_at(*i))
            .collect()
    }

    pub fn get_layers_under_point(&self, x: f32, y: f32) -> Vec<PAGLayer> {
        let indices = ffi::pag::cxx_PAGComposition_getLayersUnderPoint(self.pin_mut(), x, y);
        indices
            .iter()
            .filter_map(|i| self.get_layer_at(*i))
            .collect()
    }
}
