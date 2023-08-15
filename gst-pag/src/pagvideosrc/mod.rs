use gst::glib;
use gst::prelude::*;

mod imp;

// The public Rust wrapper type for our element
glib::wrapper! {
    pub struct PAGVideoSrc(ObjectSubclass<imp::PAGVideoSrc>) @extends gst_app::AppSrc, gst::Element, gst::Object;
}

// Registers the type for our element, and then registers in GStreamer under
// the name "pagsrc" for being able to instantiate it via e.g.
// gst::ElementFactory::make().
pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "pagvideosrc",
        gst::Rank::None,
        PAGSrc::static_type(),
    )
}