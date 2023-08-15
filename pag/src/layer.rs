use crate::LayerType;
use crate::Matrix;
use crate::PAGComposition;
use crate::PAGMarker;
use crate::PAGRect;
use crate::ID;

define_pag_class!(PAGLayer);

impl PAGLayer {
    pub fn unique_id(&self) -> ID {
        self.pin_mut().uniqueID()
    }

    pub fn layer_type(&self) -> LayerType {
        self.pin_mut().layerType()
    }

    pub fn layer_name(&self) -> String {
        self.pin_mut().layerName().to_string()
    }

    pub fn matrix(&self) -> Matrix {
        self.pin_mut().matrix()
    }

    pub fn set_matrix(&self, value: &Matrix) {
        self.pin_mut().setMatrix(value);
    }

    pub fn reset_matrix(&self) {
        self.pin_mut().resetMatrix();
    }

    pub fn get_total_matrix(&self) -> Matrix {
        self.pin_mut().getTotalMatrix()
    }

    pub fn alpha(&self) -> f32 {
        self.pin_mut().alpha().into()
    }

    pub fn set_alpha(&self, value: f32) {
        self.pin_mut().setAlpha(value);
    }

    pub fn visible(&self) -> bool {
        self.pin_mut().visible()
    }

    pub fn set_visible(&self, value: bool) {
        self.pin_mut().setVisible(value);
    }

    pub fn editable_index(&self) -> i32 {
        self.pin_mut().editableIndex().into()
    }

    pub fn parent(&self) -> Option<PAGComposition> {
        let ptr = self.pin_mut().parent();
        if ptr.is_null() {
            None
        } else {
            Some(PAGComposition::from_ptr(ptr))
        }
    }

    pub fn markers(&self) -> Vec<PAGMarker> {
        let markers = ffi::pag::cxx_PAGLayer_getMarkers(self.pin_mut());
        markers
            .into_iter()
            .map(|i| {
                let marker: &PAGMarker = unsafe { std::mem::transmute(i) };
                marker.clone()
            })
            .collect()
    }

    pub fn local_time_to_global(&self, time: i64) -> i64 {
        self.pin_mut().localTimeToGlobal(time)
    }

    pub fn global_to_local_time(&self, time: i64) -> i64 {
        self.pin_mut().globalToLocalTime(time)
    }

    pub fn duration(&self) -> i64 {
        self.pin_mut().duration()
    }

    pub fn frame_rate(&self) -> f32 {
        self.pin_mut().frameRate()
    }

    pub fn start_time(&self) -> i64 {
        self.pin_mut().startTime()
    }

    pub fn set_start_time(&self, value: i64) {
        self.pin_mut().setStartTime(value);
    }

    pub fn current_time(&self) -> i64 {
        self.pin_mut().currentTime()
    }

    pub fn set_current_time(&self, value: i64) {
        self.pin_mut().setCurrentTime(value);
    }

    pub fn get_progress(&self) -> f64 {
        self.pin_mut().getProgress()
    }

    pub fn set_progress(&self, value: f64) {
        self.pin_mut().setProgress(value);
    }

    pub fn pre_frame(&self) {
        self.pin_mut().preFrame();
    }

    pub fn next_frame(&self) {
        self.pin_mut().nextFrame();
    }

    pub fn get_bounds(&self) -> PAGRect {
        ffi::pag::cxx_PAGLayer_getBounds(self.pin_mut())
    }

    pub fn track_matte_layer(&self) -> Option<PAGLayer> {
        let ptr = self.pin_mut().trackMatteLayer();
        if ptr.is_null() {
            None
        } else {
            Some(PAGLayer::from_ptr(ptr))
        }
    }

    pub fn excluded_from_timeline(&self) -> bool {
        self.pin_mut().excludedFromTimeline()
    }

    pub fn set_excluded_from_timeline(&self, value: bool) {
        self.pin_mut().setExcludedFromTimeline(value);
    }

    // virtual method
    pub fn is_pag_file(&self) -> bool {
        return self.pin_mut().isPAGFile();
    }
}
