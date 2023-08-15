use neon::prelude::*;
use libpag::*;

pub fn pag_composition_make(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let width = cx.argument::<JsNumber>(0)?;
    let width = width.value(&mut cx) as i32;
    let height = cx.argument::<JsNumber>(1)?;
    let height = height.value(&mut cx) as i32;

    let composition = PAGComposition::make(width, height);
    let uid = composition.unique_id();
    global_pag_registry().add(composition);
    Ok(cx.number(uid))
}

pub fn pag_composition_width(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let composition = global_pag_registry()
        .get_composition(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(composition.width()))
}

pub fn pag_composition_height(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let composition = global_pag_registry()
        .get_composition(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(composition.height()))
}

pub fn pag_composition_set_content_size(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let width = cx.argument::<JsNumber>(1)?;
    let width = width.value(&mut cx) as i32;
    let height = cx.argument::<JsNumber>(2)?;
    let height = height.value(&mut cx) as i32;

    let composition = global_pag_registry()
        .get_composition(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    composition.set_content_size(width, height);
    Ok(cx.undefined())
}

pub fn pag_composition_num_children(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let composition = global_pag_registry()
        .get_composition(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(composition.num_children()))
}

pub fn pag_composition_audio_start_time(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let composition = global_pag_registry()
        .get_composition(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(composition.audio_start_time() as f32))
}
