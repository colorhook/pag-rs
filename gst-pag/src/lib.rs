#![allow(clippy::non_send_fields_in_send_ty, unused_doc_comments)]

use gst::glib;

mod message;
use message::*;

mod pag_app_src;
use pag_app_src::PAGAppSrc;

mod pag_audio_src;
use pag_audio_src::PAGAudioSrc;

mod export_session;
pub use export_session::*;

fn plugin_init(_plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    // pagaudiosrc::register(plugin)?;
    Ok(())
}

gst::plugin_define!(
    // plugin name,
    pagsrc,
    // plugin description
    env!("CARGO_PKG_DESCRIPTION"),
    // plugin entry point function,
    plugin_init,
    // version number of this plugin
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    // license of the plugin
    "MIT",
    // source package name
    env!("CARGO_PKG_NAME"),
    // binary package name
    env!("CARGO_PKG_NAME"),
    // origin where it comes from
    env!("CARGO_PKG_REPOSITORY"),
    // and the date/time of release
    env!("BUILD_REL_DATE")
);
