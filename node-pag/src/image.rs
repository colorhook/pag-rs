use libpag::*;
use neon::prelude::*;

pub fn pag_image_new(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uri = cx.argument::<JsString>(0)?;
    let uri = String::from(uri.value(&mut cx));

    let image = PAGImage::from_path(uri).or_else(|err| cx.throw_error(err.to_string()))?;
    let uid = image.unique_id();
    global_pag_registry().add_image(image);
    Ok(cx.number(uid))
}

pub fn pag_image_width(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let image: PAGImage = global_pag_registry()
        .get_image(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(image.width()))
}

pub fn pag_image_height(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let image: PAGImage = global_pag_registry()
        .get_image(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(image.height()))
}

pub fn pag_image_scale_mode(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let image: PAGImage = global_pag_registry()
        .get_image(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(image.scale_mode() as u8))
}

pub fn pag_image_set_scale_mode(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let scale_mode = cx.argument::<JsNumber>(1)?;
    let scale_mode = scale_mode.value(&mut cx) as u8;
    let image: PAGImage = global_pag_registry()
        .get_image(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    image.set_scale_mode(PAGScaleMode::from(scale_mode));
    Ok(cx.undefined())
}

pub fn pag_image_dispose(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    global_pag_registry().remove_image(uid);
    Ok(cx.undefined())
}
