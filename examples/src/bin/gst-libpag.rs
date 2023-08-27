// This example shows how to use the appsrc element.
// It operates the following pipeline:

// {appsrc} - {videoconvert} - {autovideosink}

// The application itself provides the video-data for the pipeline, by providing
// it in the callback of the appsrc element. Videoconvert makes sure that the
// format the application provides can be displayed by the autovideosink
// at the end of the pipeline.
// The application provides data of the following format:
// Video / BGRx (4 bytes) / 2 fps
use anyhow::Error;
use derive_more::{Display, Error};
use gst::prelude::*;
use libpag::*;
use std::path::PathBuf;

#[path = "../examples-common.rs"]
mod examples_common;

pub trait VideoStream {
    fn video_info(&self) -> &gst_video::VideoInfo;
    fn poll_frame(&mut self) -> Option<gst::Buffer>;
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

const WIDTH: usize = 750;
const HEIGHT: usize = 1334;

#[allow(dead_code)]
struct PagSource {
    video_info: gst_video::VideoInfo,
    current_frame: i32,
    total_frames: i32,
    fps: f32,
    surface: PAGSurface,
    player: PAGPlayer,
}

unsafe impl Send for PagSource {}
unsafe impl Sync for PagSource {}

impl PagSource {
    pub fn new(width: i32, height: i32, file: &str) -> Self {
        let surface = PAGSurface::make_offscreen(width, height).unwrap();
        let pag_file = PAGFile::from_file(file).unwrap();
        let player = PAGPlayer::new();

        let fps = pag_file.frame_rate();
        let total_frames = (pag_file.duration() as f32) * pag_file.frame_rate() / 1000000.0;
        let total_frames = total_frames as i32;
        println!(
            "fps = {}, total_frames = {}",
            pag_file.frame_rate(),
            total_frames
        );

        player.set_surface(&surface);
        player.set_composition(pag_file);

        let video_info = gst_video::VideoInfo::builder(
            gst_video::VideoFormat::Rgba,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        )
        .fps(gst::Fraction::approximate_f32(fps).unwrap())
        .build()
        .expect("Failed to create video info");

        Self {
            video_info,
            current_frame: 0,
            fps,
            total_frames,
            surface,
            player,
        }
    }
}

impl VideoStream for PagSource {
    fn video_info(&self) -> &gst_video::VideoInfo {
        &self.video_info
    }
    fn poll_frame(&mut self) -> Option<gst::Buffer> {
        if self.current_frame >= self.total_frames {
            return None;
        }
        self.player.flush();

        let video_info = self.video_info();
        let mut buffer = gst::Buffer::with_size(video_info.size()).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let frame = self.current_frame as u64;
            buffer.set_pts(frame * 40 * gst::ClockTime::MSECOND);

            let mut vframe =
                gst_video::VideoFrameRef::from_buffer_ref_writable(buffer, &video_info).unwrap();

            // let stride = vframe.plane_stride()[0] as usize;
            let video_ptr = vframe.plane_data_mut(0).unwrap();
            self.surface.read_rgba(video_ptr);
        }

        self.player.next_frame();
        self.current_frame += 1;
        Some(buffer)
    }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    let pipeline = gst::Pipeline::default();
    let mut app_source = PagSource::new(
        WIDTH.try_into().unwrap(),
        HEIGHT.try_into().unwrap(),
        assets_dir.join("MT2.pag").to_str().unwrap(),
    );

    let appsrc = gst_app::AppSrc::builder()
        .caps(&app_source.video_info().to_caps().unwrap())
        .format(gst::Format::Time)
        .build();

    let videoconvert = gst::ElementFactory::make("videoconvert").build()?;

    let sink = gst::ElementFactory::make("autovideosink").build()?;
    pipeline.add_many([appsrc.upcast_ref(), &videoconvert, &sink])?;
    gst::Element::link_many([appsrc.upcast_ref(), &videoconvert, &sink])?;

    // let x264enc = gst::ElementFactory::make("x264enc").build()?;
    // let mp4mux = gst::ElementFactory::make("mp4mux").build()?;

    // let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // let sink = gst::ElementFactory::make("filesink")
    //     .property("location", dir.join("skia.mp4").to_str())
    //     .build()?;

    // pipeline.add_many([appsrc.upcast_ref(), &videoconvert, &x264enc, &mp4mux, &sink])?;
    // gst::Element::link_many([appsrc.upcast_ref(), &videoconvert, &x264enc, &mp4mux, &sink])?;

    appsrc.set_callbacks(
        // Since our appsrc element operates in pull mode (it asks us to provide data),
        // we add a handler for the need-data callback and provide new data from there.
        // In our case, we told gstreamer that we do 2 frames per second. While the
        // buffers of all elements of the pipeline are still empty, this will be called
        // a couple of times until all of them are filled. After this initial period,
        // this handler will be called (on average) twice per second.
        gst_app::AppSrcCallbacks::builder()
            .need_data(move |appsrc, _| match app_source.poll_frame() {
                Some(buffer) => {
                    let s = appsrc.push_buffer(buffer);
                    println!("poll result: {:?}", s);
                }
                None => {
                    let _ = appsrc.end_of_stream();
                }
            })
            .build(),
    );

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

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn example_main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
