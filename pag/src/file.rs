use cxx::let_cxx_string;

use crate::LayerType;
use crate::PAGComposition;
use crate::PAGError;
use crate::PAGImage;
use crate::PAGLayer;
use crate::PAGTimeStretchMode;
use crate::TextDocument;

define_pag_sub_class!(PAGFile, PAGComposition);
define_pag_type_from!(PAGFile, PAGComposition);
define_pag_type_from!(PAGComposition, PAGFile);
define_pag_type_from!(PAGFile, PAGLayer);
define_pag_type_from!(PAGLayer, PAGFile);

unsafe impl Send for PAGFile {}
unsafe impl Sync for PAGFile {}

impl PAGFile {
    pub fn max_supported_tag_level() -> u16 {
        ffi::pag::PAGFile::MaxSupportedTagLevel()
    }

    pub fn from_file(path: impl AsRef<str>) -> Result<Self, PAGError> {
        let filepath = path.as_ref();
        let_cxx_string!(file = filepath);
        let_cxx_string!(password = "");
        let ptr = ffi::pag::PAGFile::Load1(&file, &password);
        if ptr.is_null() {
            Err(PAGError::FileError)
        } else {
            Ok(Self::from_ptr(ptr))
        }
    }

    pub fn tag_level(&self) -> u16 {
        self.pin_mut().tagLevel()
    }

    pub fn num_texts(&self) -> i32 {
        self.pin_mut().numTexts().into()
    }

    pub fn num_images(&self) -> i32 {
        self.pin_mut().numImages().into()
    }

    pub fn num_videos(&self) -> i32 {
        self.pin_mut().numVideos().into()
    }

    pub fn path(&self) -> String {
        self.pin_mut().path().to_string()
    }

    pub fn get_text_data(&self, index: i32) -> Option<TextDocument> {
        let ptr = self.pin_mut().getTextData(autocxx::c_int(index));
        if ptr.is_null() {
            None
        } else {
            Some(TextDocument::from_ptr(ptr))
        }
    }

    pub fn replace_text(&self, index: i32, text: Option<TextDocument>) {
        let ptr = text.map_or(cxx::SharedPtr::null(), |v| v.ptr);
        self.pin_mut().replaceText(autocxx::c_int(index), ptr);
    }

    pub fn replace_image(&self, index: i32, image: Option<PAGImage>) {
        let ptr = image.map_or(cxx::SharedPtr::null(), |v| v.ptr);
        self.pin_mut().replaceImage(index.into(), ptr);
    }

    pub fn replace_image_by_name(&self, name: impl Into<String>, image: Option<PAGImage>) {
        let ptr = image.map_or(cxx::SharedPtr::null(), |v| v.ptr);
        let_cxx_string!(name = name.into());
        self.pin_mut().replaceImageByName(&name, ptr);
    }

    fn get_editable_layers(
        composition: &PAGComposition,
        index: i32,
        layer_type: LayerType,
    ) -> Vec<PAGLayer> {
        let num_children = composition.num_children();
        let mut vec = Vec::new();
        for i in 0..num_children {
            if let Some(layer) = composition.get_layer_at(i) {
                if layer.editable_index() == index && layer.layer_type() == layer_type {
                    vec.push(layer);
                } else if layer.layer_type() == LayerType::PreCompose {
                    vec.extend(PAGFile::get_editable_layers(
                        &layer.into(),
                        index,
                        layer_type,
                    ))
                }
            }
        }
        vec
    }
    /**
     * 因绑定问题，在 Rust 层基于 C++ 逻辑重新实现
     */
    pub fn get_layers_by_editable_index(&self, index: i32, layer_type: LayerType) -> Vec<PAGLayer> {
        PAGFile::get_editable_layers(&self, index, layer_type)
    }

    pub fn get_editable_indices(&self, layer_type: LayerType) -> Vec<i32> {
        let indices = ffi::pag::cxx_PAGFile_getEditableIndices(self.pin_mut(), layer_type);
        indices.into_iter().map(|i| (*i).into()).collect()
    }

    pub fn time_stretch_mode(&self) -> PAGTimeStretchMode {
        PAGTimeStretchMode::from(self.pin_mut().timeStretchMode())
    }

    pub fn set_time_stretch_mode(&self, mode: PAGTimeStretchMode) {
        self.pin_mut().setTimeStretchMode(mode as u8)
    }

    pub fn set_duration(&self, duration: i64) {
        self.pin_mut().setDuration(duration);
    }

    pub fn is_pag_file(&self) -> bool {
        return self.pin_mut().isPAGFile();
    }
}
