use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst_base::prelude::*;
use gst_base::subclass::base_src::CreateSuccess;
use gst_base::subclass::prelude::*;

use byte_slice_cast::*;

use std::ops::Rem;
use std::sync::Mutex;
use std::u32;

use num_traits::cast::NumCast;
use num_traits::float::Float;

use once_cell::sync::Lazy;


use libpag::*;
use std::path::PathBuf;

// This module contains the private implementation details of our element

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rust-pagaudiosrc",
        gst::DebugColorFlags::empty(),
        Some("Rust PAG Audio Source"),
    )
});

#[derive(Debug)]
struct Settings {
    pub file: Option<PAGFile>,
}


unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

impl Default for Settings {
    fn default() -> Self {
        Settings { file: None }
    }
}

impl Settings {
    pub fn file(&self) -> &PAGFile {
        self.file.as_ref().unwrap()
    }
}

// Struct containing all the element data
#[derive(Default)]
pub struct PAGAudioSrc {
    settings: Mutex<Settings>,
}

impl PAGAudioSrc {
    pub fn set_file(&self, file: PAGFile) {
        let mut settings = self.settings.lock().unwrap();
        settings.file = Some(file);
    }
}

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
#[glib::object_subclass]
impl ObjectSubclass for PAGAudioSrc {
    const NAME: &'static str = "GstPAGAudioSrc";
    type Type = super::PAGAudioSrc;
    type ParentType = gst_base::BaseSrc;
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for PAGAudioSrc {
    // Metadata for the properties
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                // glib::ParamSpecString::builder("file")
                // .nick("File")
                // .blurb("Path of the pag file to be used by query from global_pag_registry cache")
                // .mutable_ready()
                // .build()
            ]
        });

        PROPERTIES.as_ref()
    }

    // Called right after construction of a new instance
    fn constructed(&self) {
        // Call the parent class' ::constructed() implementation first
        self.parent_constructed();
    }

    // Called whenever a value of a property is changed. It can be called
    // at any time from any thread.
    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        // let mut settings = self.settings.lock().unwrap();
        // let s: String = value.get().unwrap();

        // match pspec.name() {
        //     "file" => {
        //         let path: String = value.get().unwrap();
        //         settings.file = global_pag_registry().get_file(&path).unwrap();
        //         println!("settings.file: {:?}", &settings.file);
        //     }
        //     _ => unimplemented!(),
        // };
    }

    // Called whenever a value of a property is read. It can be called
    // at any time from any thread.
    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }
}

impl GstObjectImpl for PAGAudioSrc {}

// Implementation of gst::Element virtual methods
impl ElementImpl for PAGAudioSrc {
    // Set the element specific metadata. This information is what
    // is visible from gst-inspect-1.0 and can also be programatically
    // retrieved from the gst::Registry after initial registration
    // without having to load the plugin in memory.
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "PAG Audio Source",
                "Source/Audio",
                "Creates a PAG Audio source",
                "lj",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    // Create and add pad templates for our sink and source pad. These
    // are later used for actually creating the pads and beforehand
    // already provide information to GStreamer about all possible
    // pads that could exist for this type.
    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            // On the src pad, we can produce F32/F64 with any sample rate
            // and any number of channels
            let caps = gst::Caps::new_any();
            // The src pad template must be named "src" for basesrc
            // and specific a pad that is always there
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &caps,
            )
            .unwrap();

            vec![src_pad_template]
        });

        PAD_TEMPLATES.as_ref()
    }

    // Called whenever the state of the element should be changed. This allows for
    // starting up the element, allocating/deallocating resources or shutting down
    // the element again.
    fn change_state(
        &self,
        transition: gst::StateChange,
    ) -> Result<gst::StateChangeSuccess, gst::StateChangeError> {
        // Call the parent class' implementation of ::change_state()
        self.parent_change_state(transition)
    }
}

// Implementation of gst_base::BaseSrc virtual methods
impl BaseSrcImpl for PAGAudioSrc {
    fn size(&self) -> Option<u64> {
        let settings = self.settings.lock().unwrap();
        let file = settings.file();
        let audio_bytes: &[u8] = file.audio_bytes().unwrap();
        println!("audio_bytes.len = {}", audio_bytes.len());
        Some(audio_bytes.len() as u64)
    }

    fn is_seekable(&self) -> bool {
        true
    }

    fn create(
        &self,
        offset: u64,
        _buffer: Option<&mut gst::BufferRef>,
        length: u32,
    ) -> Result<CreateSuccess, gst::FlowError> {
        let offset: usize = offset.try_into().unwrap();
        let length: usize = length.try_into().unwrap();
        let mut buffer = gst::Buffer::with_size(length).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();

            let settings = &self.settings.lock().unwrap();
            let file = settings.file();
            let audio_bytes: &[u8] = file.audio_bytes().unwrap();

            let end = offset + length;
            let slice = &audio_bytes[offset..end];
            // println!("offset={}, length={}, end={}, size={}", offset, length, end, audio_bytes.len());
            buffer.copy_from_slice(0, slice).unwrap();
        }
        Ok(CreateSuccess::NewBuffer(buffer))
    }
}
