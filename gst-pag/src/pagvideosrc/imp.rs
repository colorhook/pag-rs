use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst_app:prelude::*;

use byte_slice_cast::*;

use std::ops::Rem;
use std::sync::Mutex;
use std::u32;

use num_traits::cast::NumCast;
use num_traits::float::Float;

use once_cell::sync::Lazy;

use std::path::PathBuf;
use libpag::*;

// This module contains the private implementation details of our element

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rust-pagvideosrc",
        gst::DebugColorFlags::empty(),
        Some("Rust PAG Video Source"),
    )
});

#[derive(Debug)]
struct Settings {
    file: PAGFile
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

impl Default for Settings {
    fn default() -> Self {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let assets_dir = dir.join("../third_party/libpag/assets");
        let pag_file = PAGFile::from_file(assets_dir.join("AudioMarker.pag").to_str().unwrap());
        Settings {
            file: pag_file
        }
    }
}


// Struct containing all the element data
#[derive(Default)]
pub struct PAGVideoSrc {
    settings: Mutex<Settings>,
}


// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
#[glib::object_subclass]
impl ObjectSubclass for PAGVideoSrc {
    const NAME: &'static str = "GstPAGVideoSrc";
    type Type = super::PAGVideoSrc;
    type ParentType = gst_app::AppSrc;
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for PAGVideoSrc {
    // Metadata for the properties
    // fn properties() -> &'static [glib::ParamSpec] {
    //     static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
    //     });

    //     PROPERTIES.as_ref()
    // }

    // Called right after construction of a new instance
    fn constructed(&self) {
        // Call the parent class' ::constructed() implementation first
        self.parent_constructed();
    }

    // Called whenever a value of a property is changed. It can be called
    // at any time from any thread.
    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }

    // Called whenever a value of a property is read. It can be called
    // at any time from any thread.
    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }
}

impl GstObjectImpl for PAGVideoSrc {}

// Implementation of gst::Element virtual methods
impl ElementImpl for PAGVideoSrc {
    // Set the element specific metadata. This information is what
    // is visible from gst-inspect-1.0 and can also be programatically
    // retrieved from the gst::Registry after initial registration
    // without having to load the plugin in memory.
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "PAG Video Source",
                "Source/Video",
                "Creates a PAG Video source",
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
impl AppSrcImpl for PAGVideoSrc {

}