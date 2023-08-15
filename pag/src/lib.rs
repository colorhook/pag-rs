#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

mod error;
pub use error::PAGError;

mod types;
pub use types::*;

mod video_range;
pub use video_range::*;

mod byte_data;
pub use byte_data::*;

mod text_document;
pub use text_document::*;

mod font;
pub use font::*;

mod layer;
pub use layer::*;

mod solid_layer;
pub use solid_layer::*;

mod shape_layer;
pub use shape_layer::*;

mod image_layer;
pub use image_layer::*;

mod text_layer;
pub use text_layer::*;

mod file;
pub use file::*;

mod composition;
pub use composition::*;

mod surface;
pub use surface::*;

mod image;
pub use image::*;

mod player;
pub use player::*;

mod pag_registry;
pub use pag_registry::global_pag_registry;

mod movie;
pub use movie::*;

pub fn sdk_version() -> String {
    let str = ffi::pag::PAG::SDKVersion();
    str.to_string()
}
