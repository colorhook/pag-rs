use neon::prelude::*;

mod text_document;
use text_document::JsTextDocument;

mod image;
use image::*;

mod layer;
use layer::*;

mod composition;
use composition::*;

mod file;
use file::*;

mod registry;
use registry::*;

mod export_session;
use export_session::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("pag_image_new", pag_image_new)?;
    cx.export_function("pag_image_width", pag_image_width)?;
    cx.export_function("pag_image_height", pag_image_height)?;
    cx.export_function("pag_image_scale_mode", pag_image_scale_mode)?;
    cx.export_function("pag_image_set_scale_mode", pag_image_set_scale_mode)?;
    cx.export_function("pag_image_dispose", pag_image_dispose)?;

    cx.export_function("pag_layer_unique_id", pag_layer_unique_id)?;
    cx.export_function("pag_layer_layer_type", pag_layer_layer_type)?;
    cx.export_function("pag_layer_layer_name", pag_layer_layer_name)?;
    cx.export_function("pag_layer_alpha", pag_layer_alpha)?;
    cx.export_function("pag_layer_set_alpha", pag_layer_set_alpha)?;
    cx.export_function("pag_layer_visible", pag_layer_visible)?;
    cx.export_function("pag_layer_set_visible", pag_layer_set_visible)?;
    cx.export_function("pag_layer_editable_index", pag_layer_editable_index)?;
    cx.export_function("pag_layer_frame_rate", pag_layer_frame_rate)?;
    cx.export_function("pag_layer_duration", pag_layer_duration)?;
    cx.export_function("pag_layer_start_time", pag_layer_start_time)?;
    cx.export_function("pag_layer_set_start_time", pag_layer_set_start_time)?;
    cx.export_function("pag_layer_current_time", pag_layer_current_time)?;
    cx.export_function("pag_layer_set_current_time", pag_layer_set_current_time)?;
    cx.export_function("pag_layer_get_progress", pag_layer_get_progress)?;
    cx.export_function("pag_layer_set_progress", pag_layer_set_progress)?;
    cx.export_function("pag_layer_pre_frame", pag_layer_pre_frame)?;
    cx.export_function("pag_layer_next_frame", pag_layer_next_frame)?;
    cx.export_function("pag_layer_is_pag_file", pag_layer_is_pag_file)?;
    cx.export_function("pag_layer_dispose", pag_layer_dispose)?;

    cx.export_function("pag_composition_make", pag_composition_make)?;
    cx.export_function("pag_composition_width", pag_composition_width)?;
    cx.export_function("pag_composition_height", pag_composition_height)?;
    cx.export_function(
        "pag_composition_set_content_size",
        pag_composition_set_content_size,
    )?;
    cx.export_function("pag_composition_num_children", pag_composition_num_children)?;
    cx.export_function(
        "pag_composition_audio_start_time",
        pag_composition_audio_start_time,
    )?;

    cx.export_function("pag_file_new", pag_file_new)?;
    cx.export_function(
        "pag_file_max_supported_tag_level",
        pag_file_max_supported_tag_level,
    )?;
    cx.export_function("pag_file_tag_level", pag_file_tag_level)?;
    cx.export_function("pag_file_num_texts", pag_file_num_texts)?;
    cx.export_function("pag_file_num_images", pag_file_num_images)?;
    cx.export_function("pag_file_num_videos", pag_file_num_videos)?;
    cx.export_function("pag_file_path", pag_file_path)?;
    cx.export_function("pag_file_get_text_data", pag_file_get_text_data)?;
    cx.export_function("pag_file_replace_text", pag_file_replace_text)?;
    cx.export_function("pag_file_replace_image", pag_file_replace_image)?;

    cx.export_function(
        "pag_file_get_editable_indices",
        pag_file_get_editable_indices,
    )?;
    cx.export_function("pag_file_time_stretch_mode", pag_file_time_stretch_mode)?;
    cx.export_function(
        "pag_file_set_time_stretch_mode",
        pag_file_set_time_stretch_mode,
    )?;

    cx.export_function("pag_registry_reset", pag_registry_reset)?;

    cx.export_function(
        "pag_export_session_start_async",
        pag_export_session_start_async,
    )?;
    Ok(())
}
