use std::sync::Arc;

use neon::prelude::*;
use gst_pag::*;
use libpag::*;

pub fn pag_export_session_start_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    gst::init().or_else(|err| cx.throw_error(err.to_string()))?;
    let uid = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let composition = global_pag_registry()
        .get_composition(uid)
        .ok_or("Resource had disposed")
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let options: Handle<JsObject> = cx.argument::<JsObject>(1)?;
    let output: Handle<JsString> = options.get(&mut cx, "output")?;
    let output: String = output.value(&mut cx);


    let prop = options.get_opt::<JsNumber, _, _>(&mut cx, "width");
    let width: i32 = match prop {
        Ok(Some(width)) => width.value(&mut cx) as i32,
        _ => composition.width(),
    };
    let prop = options.get_opt::<JsNumber, _, _>(&mut cx, "height");
    let height: i32 = match prop {
        Ok(Some(height)) => height.value(&mut cx) as i32,
        _ => composition.height(),
    };
    let prop = options.get_opt::<JsNumber, _, _>(&mut cx, "frame_rate");
    let frame_rate: f32 = match prop {
        Ok(Some(frame_rate)) => frame_rate.value(&mut cx) as f32,
        _ => composition.frame_rate(),
    };

    let config = PAGExportSessionConfig {
        output,
        width,
        height,
        frame_rate,
    };

    let progress_callback = cx.argument::<JsFunction>(2)?.root(&mut cx);
    let complete_callback = cx.argument::<JsFunction>(3)?.root(&mut cx);
    let channel = cx.channel();
    let channel_progress = cx.channel();
    let progress_arc = Arc::new(progress_callback);

    std::thread::spawn(move || {
        let mut session = PAGExportSession::new_with_config(composition, config);

        let handler = Box::new(move |progress| {
            let progress_callback = Arc::clone(&progress_arc);
            channel_progress.send(move |mut cx| {
                let progress_callback = progress_callback.to_inner(&mut cx);
                let this = cx.undefined();
                let args = vec![cx.number(progress).upcast::<JsValue>()];
                let _ = progress_callback.call(&mut cx, this, args);
                Ok(())
            });
        });
        let err = session.start(Some(handler)).err();

        channel.send(move |mut cx| {
            let complete_callback = complete_callback.into_inner(&mut cx);
            let this = cx.undefined();
            let mut args = vec![];
            if let Some(err) = err {
                args.push(cx.error(format!("{:?}", err))?.upcast());
            }
            complete_callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
}
