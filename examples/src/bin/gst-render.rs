use anyhow::Error;
use derive_more::{Display, Error};
use gst::prelude::*;
use gst_pag::*;
use libpag::*;
use std::path::PathBuf;

/// GStreamer 1.21.1
/// https://bugzilla.gnome.org/show_bug.cgi?id=782379

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

// 是否是渲染到文件
// https://gist.github.com/hum4n0id/cda96fb07a34300cdb2c0e314c14df0a
const RENDER_MODE: bool = true;

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    let mut pag_source = PagSource::new(assets_dir.join("AudioMarker.pag").to_str().unwrap());
    let pipeline = gst::Pipeline::default();

    let audiosrc = pag_source.get_audio_element().unwrap();
    global_pag_registry().clear();

    let videosrc = pag_source.get_video_element();
    let videoconvert = gst::ElementFactory::make("videoconvert").build()?;

    if RENDER_MODE {
        // x264enc 这个编码器在 macOS 中无法使用 QuickTime Player 播放
        let use_hw = false;
        let name = if use_hw {
            "vtenc_h264_hw"
        } else {
            "vtenc_h264"
        };
        // let name = if use_hw { "nvh264enc" } else { "x264enc" };

        let x264enc = gst::ElementFactory::make(name).build()?;
        // let h264parse = gst::ElementFactory::make("h264parse").build()?;
        let mp4mux = gst::ElementFactory::make("mp4mux").name("mux").build()?;

        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let sink = gst::ElementFactory::make("filesink")
            .property("location", dir.join("pag.mp4").to_str())
            .build()?;
        pipeline.add_many([
            videosrc.upcast_ref(),
            &videoconvert,
            &x264enc,
            &mp4mux,
            &sink,
        ])?;
        gst::Element::link_many([
            videosrc.upcast_ref(),
            &videoconvert,
            &x264enc,
            &mp4mux,
            &sink,
        ])?;
    } else {
        let video_caps = gst_video::VideoCapsBuilder::new()
            .width(750)
            .height(1334)
            .build();
        let scale = gst::ElementFactory::make("videoscale").build()?;
        let filter = gst::ElementFactory::make("capsfilter")
            .property("caps", &video_caps)
            .build()?;
        let sink = gst::ElementFactory::make("osxvideosink").build()?;
        let queue = gst::ElementFactory::make("queue").build()?;
        pipeline.add_many([
            videosrc.upcast_ref(),
            &queue,
            &videoconvert,
            &scale,
            &filter,
            &sink,
        ])?;
        gst::Element::link_many([
            videosrc.upcast_ref(),
            &queue,
            &videoconvert,
            &scale,
            &filter,
            &sink,
        ])?;
    }

    if RENDER_MODE {
        let mux = pipeline.by_name("mux").unwrap();
        let demux = gst::ElementFactory::make("qtdemux").build()?;

        pipeline.add_many([audiosrc.upcast_ref(), &demux])?;
        gst::Element::link_many([audiosrc.upcast_ref(), &demux])?;

        let pipeline_weak = pipeline.downgrade();
        demux.connect_pad_added(move |_, src_pad| {
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return,
            };

            let elements = [&mux];
            for e in elements {
                e.sync_state_with_parent().unwrap();
            }

            let sink_pad = mux
                .request_pad_simple("audio_%u")
                .expect("Could not get audio pad from mp4mux");
            src_pad
                .link(&sink_pad)
                .expect("Unable to link src pad to sink pad");
        });
    } else {
        let decodebin = gst::ElementFactory::make("decodebin").build()?;
        let queue = gst::ElementFactory::make("queue").build()?;
        pipeline.add_many([audiosrc.upcast_ref(), &queue, &decodebin])?;
        gst::Element::link_many([audiosrc.upcast_ref(), &queue, &decodebin])?;

        let pipeline_weak = pipeline.downgrade();
        decodebin.connect_pad_added(move |_, src_pad| {
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return,
            };

            let audioconvert = gst::ElementFactory::make("audioconvert").build().unwrap();
            let audioresample = gst::ElementFactory::make("audioresample").build().unwrap();
            let audiosink = gst::ElementFactory::make("autoaudiosink").build().unwrap();

            let elements = [&audioconvert, &audioresample, &audiosink];
            pipeline.add_many(&elements).unwrap();
            gst::Element::link_many(&elements).unwrap();

            for e in elements {
                e.sync_state_with_parent().unwrap();
            }

            let sink_pad = audioconvert.static_pad("sink").unwrap();
            src_pad
                .link(&sink_pad)
                .expect("Unable to link src pad to sink pad");
        });
    }

    // let mpegaudioparse = gst::ElementFactory::make("mpegaudioparse").build()?;
    // let mpg123audiodec = gst::ElementFactory::make("mpg123audiodec").build()?;

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                    error: err.error(),
                    debug: err.debug(),
                }
                .into());
            }
            _ => (),
        }
    }

    // futures::executor::block_on(app_src_sink.close()).unwrap();

    // let t1 = std::thread::spawn(|| futures::executor::block_on(sleep()));
    // t1.join().unwrap();
    println!("set_state to NUll before");
    pipeline.set_state(gst::State::Null)?;
    println!("set_state to NUll after");

    Ok(())
}

fn example_main() {
    use std::time::Instant;
    let now = Instant::now();

    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    examples_common::run(example_main);
}
