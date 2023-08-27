#![allow(dead_code)]
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

#[path = "../examples-common.rs"]
mod examples_common;

#[path = "../canvas.rs"]
mod canvas;

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

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

struct AppSource {
    width: i32,
    height: i32,
    data: Vec<u8>,
    video_info: gst_video::VideoInfo,
    current_frame: i32,
    total_frames: i32,
    canvas: canvas::Canvas,
}

unsafe impl Send for AppSource {}
unsafe impl Sync for AppSource {}

use skia_safe::{paint, Color, Font, IPoint, ISize, ImageInfo, Paint, Rect, TextBlob, Typeface};

impl AppSource {
    pub fn new(width: i32, height: i32, total_frames: i32) -> Self {
        let canvas = canvas::Canvas::new(width, height);
        let data: Vec<u8> = vec![0; 320 * 240 * 4];
        let video_info = gst_video::VideoInfo::builder(
            gst_video::VideoFormat::Rgba,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        )
        .fps(gst::Fraction::new(25, 1))
        .build()
        .expect("Failed to create video info");
        Self {
            width,
            height,
            video_info,
            data,
            current_frame: 0,
            total_frames,
            canvas,
        }
    }
    pub fn draw(&mut self) {
        let canvas = self.canvas.canvas();
        canvas.clear(Color::WHITE);
        let paint = Paint::default();

        let text = TextBlob::from_str(
            format!("frame - {}", self.current_frame + 1),
            &Font::from_typeface(&Typeface::default(), 24.0),
        )
        .unwrap();
        canvas.draw_text_blob(&text, (50, 125), &paint);

        let text = TextBlob::from_str(
            "Hello, Skia!",
            &Font::from_typeface(&Typeface::default(), 24.0),
        )
        .unwrap();

        let fill_paint = &mut Paint::default();
        fill_paint.set_color(Color::from_argb(0xFF, 0xFF, 0x00, 0x00));
        canvas.draw_text_blob(&text, (50, 225), &fill_paint);

        let stroke_paint = &mut Paint::default();
        stroke_paint
            .set_style(paint::Style::Stroke)
            .set_stroke_width(3.0);

        canvas
            .draw_rect(Rect::from_point_and_size((10, 10), (60, 20)), fill_paint)
            .draw_rect(Rect::from_point_and_size((80, 10), (60, 20)), stroke_paint);
    }
}

impl VideoStream for AppSource {
    fn video_info(&self) -> &gst_video::VideoInfo {
        &self.video_info
    }
    fn poll_frame(&mut self) -> Option<gst::Buffer> {
        if self.current_frame >= self.total_frames {
            return None;
        }

        self.draw();

        let size = ISize::new(self.width, self.height);
        let dst_info = ImageInfo::new_n32_premul(size, None);

        let dst_row_bytes: usize = self.width.try_into().unwrap();
        let src_point = IPoint::default();
        // let copied = canvas.read_pixels(
        //     &dst_info,
        //     &mut self.data,
        //     dst_row_bytes * 4,
        //     src_point
        // );

        // let length: usize = (self.width * self.height * 4).try_into().unwrap();
        // let mut buffer = gst::Buffer::with_size(length).unwrap();
        // {
        //     let buffer = buffer.get_mut().unwrap();
        //     buffer.set_pts((self.current_frame as u64) * 500 * gst::ClockTime::MSECOND);
        //     // buffer.copy_from_slice(0, &data).unwrap();
        // }

        //   println!("create buffer");
        let video_info = self.video_info();
        let mut buffer = gst::Buffer::with_size(video_info.size()).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let frame = self.current_frame as u64;
            buffer.set_pts(frame * 40 * gst::ClockTime::MSECOND);

            // let r = if frame % 2 == 0 { 100 } else { 255 };
            // let g = if frame % 3 == 0 { 0 } else { 255 };
            // let b = if frame % 5 == 0 { 0 } else { 255 };

            let mut vframe =
                gst_video::VideoFrameRef::from_buffer_ref_writable(buffer, &video_info).unwrap();

            // let width = vframe.width() as usize;
            // let height = vframe.height() as usize;

            // let stride = vframe.plane_stride()[0] as usize;

            let video_ptr = vframe.plane_data_mut(0).unwrap();
            let canvas = self.canvas.canvas();
            let _ = canvas.read_pixels(&dst_info, video_ptr, dst_row_bytes * 4, src_point);

            // for line in vframe
            //   .plane_data_mut(0)
            //   .unwrap()
            //   .chunks_exact_mut(stride)
            //   .take(height)
            // {
            //   // Iterate over each pixel of 4 bytes in that line
            //   for pixel in line[..(4 * width)].chunks_exact_mut(4) {
            //     pixel[0] = b;
            //     pixel[1] = g;
            //     pixel[2] = r;
            //     pixel[3] = 255;
            //   }
            // }
        }

        self.current_frame += 1;
        Some(buffer)
    }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::default();
    let mut app_source = AppSource::new(WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap(), 200);

    let appsrc = gst_app::AppSrc::builder()
        .caps(&app_source.video_info().to_caps().unwrap())
        .format(gst::Format::Time)
        .build();

    let videoconvert = gst::ElementFactory::make("videoconvert").build()?;
    // let sink = gst::ElementFactory::make("autovideosink").build()?;

    let x264enc = gst::ElementFactory::make("x264enc").build()?;
    let mp4mux = gst::ElementFactory::make("mp4mux").build()?;

    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sink = gst::ElementFactory::make("filesink")
        .property("location", dir.join("skia.mp4").to_str())
        .build()?;

    pipeline.add_many([appsrc.upcast_ref(), &videoconvert, &x264enc, &mp4mux, &sink])?;
    gst::Element::link_many([appsrc.upcast_ref(), &videoconvert, &x264enc, &mp4mux, &sink])?;

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
                    let _ = appsrc.push_buffer(buffer);
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
