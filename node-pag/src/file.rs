use neon::prelude::*;
use libpag::*;
use crate::JsTextDocument;

pub fn pag_file_new(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uri = cx.argument::<JsString>(0)?;
    let uri = String::from(uri.value(&mut cx));
    let file = PAGFile::from_file(uri).or_else(|err| cx.throw_error(err.to_string()))?;
    let uid = file.unique_id();
    global_pag_registry().add(file);
    Ok(cx.number(uid))
}

pub fn pag_file_max_supported_tag_level(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let level = PAGFile::max_supported_tag_level();
    Ok(cx.number(level))
}

pub fn pag_file_tag_level(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(file.tag_level()))
}

pub fn pag_file_num_texts(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(file.num_texts()))
}

pub fn pag_file_num_images(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(file.num_images()))
}

pub fn pag_file_num_videos(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(file.num_videos()))
}

pub fn pag_file_path(mut cx: FunctionContext) -> JsResult<JsString> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.string(file.path()))
}

pub fn pag_file_get_text_data(mut cx: FunctionContext) -> JsResult<JsValue> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let index = cx.argument::<JsNumber>(1)?;
    let index = index.value(&mut cx) as i32;
    let text_data = file.get_text_data(index);
    let res = match text_data {
        Some(doc) => JsTextDocument::from_text_document(&doc)
            .to_js_object(&mut cx)?
            .upcast::<JsValue>(),
        None => cx.null().upcast::<JsValue>(),
    };
    Ok(res)
}

pub fn pag_file_replace_text(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let index = cx.argument::<JsNumber>(1)?;
    let index = index.value(&mut cx) as i32;

    let text_data = cx.argument::<JsValue>(2)?;

    match text_data.downcast::<JsObject, _>(&mut cx) {
        Ok(text_data) => {
            if let Some(doc) = file.get_text_data(index) {
                JsTextDocument::apply_text_document(&mut cx, &text_data, &doc);
                file.replace_text(index, Some(doc));
            }
        }
        _ => {
            file.replace_text(index, None);
        }
    };
    Ok(cx.undefined())
}

pub fn pag_file_replace_image(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let key = cx.argument::<JsValue>(1)?;
    let image = cx.argument::<JsValue>(2)?;
    let image: Option<PAGImage> = match image.downcast::<JsNumber, _>(&mut cx) {
        Ok(uid) => global_pag_registry().get_image(uid.value(&mut cx) as u32),
        _ => None,
    };

    if let Ok(index) = key.downcast::<JsNumber, _>(&mut cx) {
        let index = index.value(&mut cx) as i32;
        file.replace_image(index, image);
    } else if let Ok(name) = key.downcast::<JsString, _>(&mut cx) {
        let name = name.value(&mut cx);
        file.replace_image_by_name(name, image);
    }
    Ok(cx.undefined())
}

pub fn pag_file_get_editable_indices(mut cx: FunctionContext) -> JsResult<JsArray> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let key = cx.argument::<JsString>(1)?;
    let key = key.value(&mut cx);
    let layer_type: LayerType = key.into();
    let indices = file.get_editable_indices(layer_type);

    let res = JsArray::new(&mut cx, indices.len() as u32);
    for (i, s) in indices.iter().enumerate() {
        let v = cx.number(*s);
        res.set(&mut cx, i as u32, v)?;
    }
    Ok(res)
}

pub fn pag_file_time_stretch_mode(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;

    Ok(cx.number(file.time_stretch_mode() as u8))
}

pub fn pag_file_set_time_stretch_mode(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let file = global_pag_registry()
        .get_file(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let mode = cx.argument::<JsNumber>(1)?;
    let mode = mode.value(&mut cx) as u8;

    file.set_time_stretch_mode(PAGTimeStretchMode::from(mode));
    Ok(cx.undefined())
}
