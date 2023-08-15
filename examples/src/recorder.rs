use gst::prelude::*;
use gst_pbutils::prelude::*;
use anyhow::Error;
use derive_more::{Display, Error};

pub trait VideoStream {
  fn video_info(&self) -> &gst_video::VideoInfo;
  fn poll_frame(&mut self) -> Option<gst::Buffer>;
}


#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
  src: String,
  error: String,
  debug: Option<String>,
  source: glib::Error,
}

pub struct Recorder {
  pipeline: gst::Pipeline,
}

impl Recorder {

  pub fn new(mut video_stream: Box<dyn VideoStream + Send>) -> Self {
    let pipeline = Self::create_pipeline(video_stream);
    let pipeline = pipeline.unwrap();
    Self {
      pipeline
    }
  }

  fn create_pipeline(mut video_stream: Box<dyn VideoStream + Send>) -> Result<gst::Pipeline, Error> {
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("appsrc", None).map_err(|_| MissingElement("appsrc"))?;
    let videoconvert = gst::ElementFactory::make("videoconvert", None)
      .map_err(|_| MissingElement("videoconvert"))?;
    let scale = gst::ElementFactory::make("videoscale", None)
      .map_err(|_| MissingElement("videoscale"))?;


    let x264enc = gst::ElementFactory::make("x264enc", None)
      .map_err(|_| MissingElement("x264enc"))?;

    let mp4mux = gst::ElementFactory::make("mp4mux", None)
      .map_err(|_| MissingElement("mp4mux"))?;

    let sink = gst::ElementFactory::make("filesink", None)
      .map_err(|_| MissingElement("filesink"))?;

    let output_file = "test.mp4";
    sink.set_property("location", &output_file)
      .expect("setting location property failed");

    pipeline.add_many(&[&src, &videoconvert, &scale, &x264enc, &mp4mux, &sink])?;
    gst::Element::link_many(&[&src, &videoconvert, &scale, &x264enc, &mp4mux, &sink])?;

    let appsrc = src
      .dynamic_cast::<gst_app::AppSrc>()
      .expect("Source element is expected to be an appsrc!");

    let video_info = video_stream.video_info();
    appsrc.set_caps(Some(&video_info.to_caps().unwrap()));
    appsrc.set_property_format(gst::Format::Time);

    appsrc.set_callbacks(
      gst_app::AppSrcCallbacks::builder()
        .need_data(move |appsrc, _| {
          let stream = video_stream.as_mut();
          match stream.poll_frame() {
            Some(buffer) => {
              let s = appsrc.push_buffer(buffer);
              println!("poll result: {:?}", s);
            },
            None => {
              appsrc.end_of_stream();
            }
          }
        })
        .build(),
    );

    Ok(pipeline)
  }

  pub fn run(&self) -> Result<(), Error> {
    let pipeline = &self.pipeline;
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
      .get_bus()
      .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
      use gst::MessageView;

      match msg.view() {
        MessageView::Eos(..) => break,
        MessageView::Error(err) => {
          pipeline.set_state(gst::State::Null)?;
          return Err(ErrorMessage {
            src: msg
              .get_src()
              .map(|s| String::from(s.get_path_string()))
              .unwrap_or_else(|| String::from("None")),
            error: err.get_error().to_string(),
            debug: err.get_debug(),
            source: err.get_error(),
          }
            .into());
        }
        _ => (),
      }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
  }
}