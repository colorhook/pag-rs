use neon::prelude::*;
use libpag::*;

pub fn pag_layer_unique_id(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.unique_id()))
}

pub fn pag_layer_layer_type(mut cx: FunctionContext) -> JsResult<JsString> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    let layer_type = format!("{}", layer.layer_type());
    Ok(cx.string(layer_type))
}

pub fn pag_layer_layer_name(mut cx: FunctionContext) -> JsResult<JsString> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.string(layer.layer_name()))
}

pub fn pag_layer_alpha(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.alpha()))
}

pub fn pag_layer_set_alpha(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let alpha = cx.argument::<JsNumber>(1)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    layer.set_alpha(alpha.value(&mut cx) as f32);
    Ok(cx.undefined())
}

pub fn pag_layer_visible(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.boolean(layer.visible()))
}

pub fn pag_layer_set_visible(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let bool = cx.argument::<JsBoolean>(1)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    layer.set_visible(bool.value(&mut cx));
    Ok(cx.undefined())
}

pub fn pag_layer_editable_index(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.editable_index()))
}

pub fn pag_layer_frame_rate(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.frame_rate()))
}

pub fn pag_layer_duration(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.duration() as f64))
}

pub fn pag_layer_start_time(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.start_time() as f64))
}

pub fn pag_layer_set_start_time(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let num = cx.argument::<JsNumber>(1)?;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    layer.set_start_time(num.value(&mut cx) as i64);
    Ok(cx.undefined())
}

pub fn pag_layer_current_time(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.current_time() as f64))
}

pub fn pag_layer_set_current_time(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let num = cx.argument::<JsNumber>(1)?;
    let num = num.value(&mut cx) as i64;

    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    layer.set_current_time(num);
    Ok(cx.undefined())
}

pub fn pag_layer_get_progress(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.number(layer.get_progress()))
}

pub fn pag_layer_set_progress(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let num = cx.argument::<JsNumber>(1)?;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    layer.set_progress(num.value(&mut cx));
    Ok(cx.undefined())
}

pub fn pag_layer_pre_frame(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    layer.pre_frame();
    Ok(cx.undefined())
}

pub fn pag_layer_next_frame(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    layer.next_frame();
    Ok(cx.undefined())
}

pub fn pag_layer_is_pag_file(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let layer: PAGLayer = global_pag_registry()
        .get(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;
    Ok(cx.boolean(layer.is_pag_file()))
}

pub fn pag_layer_dispose(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let uid = cx.argument::<JsNumber>(0)?;
    let uid = uid.value(&mut cx) as u32;
    let removed = global_pag_registry().remove(uid);
    Ok(cx.boolean(removed))
}
