use neon::prelude::*;
use libpag::*;

pub fn pag_registry_reset(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    global_pag_registry().reset();
    Ok(cx.undefined())
}
