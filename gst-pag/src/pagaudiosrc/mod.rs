use gst::glib;
use gst::prelude::*;
use gst_base::subclass::prelude::*;
use libpag::PAGFile;

mod imp;

// The public Rust wrapper type for our element
glib::wrapper! {
    pub struct PAGAudioSrc(ObjectSubclass<imp::PAGAudioSrc>) @extends gst_base::BaseSrc, gst::Element, gst::Object;
}


impl PAGAudioSrc {
    pub fn new(file: PAGFile) -> Self {
        let src: Self = glib::Object::builder().build();
        let imp = src.imp();
        imp.set_file(file);
        src
    }
}

// Registers the type for our element, and then registers in GStreamer under
// the name "pagsrc" for being able to instantiate it via e.g.
// gst::ElementFactory::make().
pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "pagaudiosrc",
        gst::Rank::None,
        PAGAudioSrc::static_type(),
    )
}