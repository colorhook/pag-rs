use std::cell::RefCell;

use anyhow::Error;
use derive_more::{Display, Error};
use gst::element_error;
use gst::glib;
use gst::prelude::*;
use gst::{event::Seek, SeekFlags, SeekType};
use libpag::*;

use crate::*;

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

#[derive(Debug, Clone)]
pub struct PAGExportSessionConfig {
    pub width: i32,
    pub height: i32,
    pub frame_rate: f32,
    pub output: String,
}

#[derive(Debug)]
pub enum PAGExportStatus {
    Completed,
    Cancel,
    Failed,
}

// 用于录制 .pag 文件为视频的模块
pub struct PAGExportSession {
    composition: PAGComposition,
    config: PAGExportSessionConfig,
    render_mode: bool,
    main_loop: glib::MainLoop,
    pipeline: gst::Pipeline,
    // movies: Vec<gst::Pipeline, RefCell<bool>>,
}

impl PAGExportSession {
    // 根据 PAGFile 和视频录制路径创建 PAGExportSession
    // 导出视频的 size 和 frame_rate 与 PAGFile 一致
    // 如果需要设置视频大小和帧率，可以使用 new_with_config 方法
    pub fn new(file: impl Into<PAGComposition>, output: &str) -> Self {
        let composition = file.into();
        let width = composition.width();
        let height = composition.height();
        let frame_rate = composition.frame_rate();
        Self::new_with_config(
            composition,
            PAGExportSessionConfig {
                width,
                height,
                frame_rate,
                output: output.to_string(),
            },
        )
    }

    // 根据详细的配置创建 PAGExportSession
    pub fn new_with_config(
        file: impl Into<PAGComposition>,
        config: PAGExportSessionConfig,
    ) -> Self {
        let composition = file.into();
        let main_loop = glib::MainLoop::new(None, false);
        let pipeline = gst::Pipeline::default();

        Self {
            composition,
            config,
            render_mode: true,
            pipeline,
            main_loop,
            // movies: Default::default(),
        }
    }

    /// 开始录制视频
    pub fn start(
        &mut self,
        progress_handler: Option<Box<dyn Fn(f32) + Send>>,
    ) -> Result<(), Error> {
        self.config_pipeline()?;
        let pipeline = &self.pipeline;

        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");

        let bus = pipeline
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        let main_loop_clone = self.main_loop.clone();

        bus.connect_message(None, move |_, msg| match msg.view() {
            gst::MessageView::Eos(..) => {
                let main_loop = &main_loop_clone;
                main_loop.quit()
            }
            gst::MessageView::Application(application) => {
                if let Some(message) = ExportProgressMessage::parse(application) {
                    if let Some(handler) = &progress_handler {
                        handler(message.progress);
                    }
                }
            }
            gst::MessageView::Error(err) => {
                let main_loop = &main_loop_clone;
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                main_loop.quit();
            }
            _ => (),
        });
        bus.add_signal_watch();
        self.main_loop.run();
        bus.remove_signal_watch();
        pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");

        Ok(())
    }

    // 配置录制 Pipeline
    pub fn config_pipeline(&mut self) -> Result<(), Error> {
        let pipeline = &self.pipeline;
        // 1. add video source
        let appsrc = self.create_video_source();
        let videoconvert = gst::ElementFactory::make("videoconvert").build()?;
        if self.render_mode {
            // x264enc 这个编码器在 macOS 中无法使用 QuickTime Player 播放
            // @TODO 需要兼容 Linux
            let use_hw = false;
            let name = if use_hw {
                "vtenc_h264_hw"
            } else {
                "vtenc_h264"
            };
            // let name = if use_hw { "nvh264enc" } else { "x264enc" };
            let x264enc = gst::ElementFactory::make(name).build()?;
            let mp4mux = gst::ElementFactory::make("mp4mux").name("mux").build()?;
            let sink = gst::ElementFactory::make("filesink")
                .name("filesink")
                .property("async", true)
                .property("location", &self.config.output)
                .build()?;
            self.pipeline.add_many([
                appsrc.upcast_ref(),
                &videoconvert,
                &x264enc,
                &mp4mux,
                &sink,
            ])?;
            gst::Element::link_many([
                appsrc.upcast_ref(),
                &videoconvert,
                &x264enc,
                &mp4mux,
                &sink,
            ])?;
        } else {
            let sink = gst::ElementFactory::make("osxvideosink").build()?;
            let queue = gst::ElementFactory::make("queue").build()?;
            pipeline.add_many([appsrc.upcast_ref(), &queue, &videoconvert, &sink])?;
            gst::Element::link_many([appsrc.upcast_ref(), &queue, &videoconvert, &sink])?;
        }
        // 2. add audio source if needed
        let audiosrc = self.create_audio_source();
        if let Some(audiosrc) = audiosrc {
            if self.render_mode {
                let mux = pipeline.by_name("mux").unwrap();
                let demux = gst::ElementFactory::make("qtdemux").build()?;

                pipeline.add_many([audiosrc.upcast_ref(), &demux])?;
                gst::Element::link_many([audiosrc.upcast_ref(), &demux])?;

                // let pipeline_weak = pipeline.downgrade();
                demux.connect_pad_added(move |_, src_pad| {
                    // let pipeline = match pipeline_weak.upgrade() {
                    //     Some(pipeline) => pipeline,
                    //     None => return,
                    // };

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
        }
        // 3. check movie replace and add movie source
        // self.add_movies();
        Ok(())
    }

    fn create_video_source(&self) -> gst_app::AppSrc {
        let config = &self.config;
        let composition = &self.composition;
        let surface = PAGSurface::make_offscreen(config.width, config.height).unwrap();
        let player = PAGPlayer::new();
        let total_frames = (composition.duration() as f32) * composition.frame_rate() / 1000000.0;
        let total_frames = total_frames as i32;
        let fps = composition.frame_rate();
        player.set_surface(&surface);
        player.set_composition(composition.clone());

        let video_info = gst_video::VideoInfo::builder(
            gst_video::VideoFormat::Rgba,
            config.width.try_into().unwrap(),
            config.height.try_into().unwrap(),
        )
        .fps(gst::Fraction::approximate_f32(fps).unwrap())
        .build()
        .expect("Failed to create video info");

        let pipeline = self.pipeline.clone();
        let total_frames2 = total_frames;

        let app_src = PAGAppSrc {
            video_info,
            current_frame: 0,
            total_frames,
            surface,
            player,
        };
        app_src.to_app_src()
    }

    fn create_audio_source(&self) -> Option<PAGAudioSrc> {
        let file: PAGComposition = self.composition.clone();
        let layer: PAGLayer = file.clone().into();
        global_pag_registry().add(layer);
        if file.audio_bytes().is_none() {
            return None;
        }
        Some(PAGAudioSrc::new(file.into()))
    }

    #[allow(dead_code)]
    fn add_movies(&mut self) {
        let file: PAGFile = self.composition.clone().into();
        let indices = file.get_editable_indices(LayerType::Image);
        for index in indices {
            let layers = file.get_layers_by_editable_index(index, LayerType::Image);
            if let Some(layer) = layers.get(0) {
                let layer: PAGImageLayer = layer.clone().into();
                let movie = global_pag_registry().get_movie(layer.unique_id());
                if let Some(movie) = movie {
                    self.add_movie(movie, &layer);
                }
            }
        }
    }

    fn add_movie(&mut self, movie: PAGMovie, layer: &PAGImageLayer) {
        let bin = gst::Pipeline::default();
        let uridecodebin = gst::ElementFactory::make("uridecodebin")
            .name("source")
            .property("uri", &format!("file:///{}", &movie.uri))
            .build()
            .expect("Could not create uridecodebin element.");
        let videoconvert = gst::ElementFactory::make("videoconvert")
            .build()
            .expect("Could not create uridecodebin element.");
        let appsink = gst_app::AppSink::builder()
            .name("appsink")
            .caps(
                &gst_video::VideoCapsBuilder::new()
                    .format(gst_video::VideoFormat::Rgbx)
                    .build(),
            )
            .build();

        // appsink.set_property("sync", true);

        bin.add_many([&uridecodebin, &videoconvert, appsink.upcast_ref()])
            .unwrap();

        let convert = videoconvert.clone();
        let appsink_cloned = appsink.clone();
        uridecodebin.connect_pad_added(move |_, src_pad| {
            let elements = [&convert, appsink_cloned.upcast_ref()];
            gst::Element::link_many(&elements).unwrap();

            for e in elements {
                e.sync_state_with_parent().unwrap();
            }

            let sink_pad = convert.static_pad("sink").unwrap();
            src_pad
                .link(&sink_pad)
                .expect("Unable to link src pad to sink pad");
        });
        // self.pipeline.add(&bin);

        let layer = layer.clone();

        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                // Add a handler to the "new-sample" signal.
                .new_sample(move |appsink| {
                    // Pull the sample in question out of the appsink's buffer.
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                    let buffer = sample.buffer().ok_or_else(|| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to get buffer from appsink")
                        );

                        gst::FlowError::Error
                    })?;
                    // println!("pts = {:?}", buffer.pts());

                    let caps = sample.caps().expect("Sample without caps");
                    let info = gst_video::VideoInfo::from_caps(caps).expect("Failed to parse caps");

                    let frame = gst_video::VideoFrameRef::from_buffer_ref_readable(buffer, &info)
                        .map_err(|_| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to map buffer readable")
                        );

                        gst::FlowError::Error
                    })?;

                    let width = frame.width() as i32;
                    let height = frame.height() as i32;
                    let row_bytes = frame.plane_stride()[0].try_into().unwrap();
                    let video_ptr = frame.plane_data(0).unwrap().as_ptr() as *const c_void;

                    let pag_image = PAGImage::from_pixels(
                        video_ptr,
                        width,
                        height,
                        row_bytes,
                        ColorType::RGBA_8888,
                        AlphaType::Premultiplied,
                    )
                    .unwrap();

                    layer.set_image(Some(pag_image));

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        let bus = bin.bus().expect("Bin without bus. Shouldn't happen!");

        let seeked = RefCell::new(false);
        let offset = movie.offset;
        let rate = movie.rate;
        let bin_clone = bin.clone();

        bus.connect_message(None, move |_, msg| match msg.view() {
            gst::MessageView::AsyncDone(..) => {
                if *seeked.borrow() == false {
                    // AsyncDone means that the pipeline has started now and that we can seek
                    println!("Got AsyncDone message, seeking to...");

                    let position = gst::ClockTime::MSECOND * (offset * 1000.0) as u64;
                    println!("position = {:?}", position);
                    let seek_event = if rate > 0. {
                        Seek::new(
                            rate.into(),
                            SeekFlags::FLUSH | SeekFlags::ACCURATE,
                            SeekType::Set,
                            position,
                            SeekType::End,
                            gst::ClockTime::ZERO,
                        )
                    } else {
                        Seek::new(
                            rate.into(),
                            SeekFlags::FLUSH | SeekFlags::ACCURATE,
                            SeekType::Set,
                            position,
                            SeekType::Set,
                            position,
                        )
                    };
                    bin_clone.send_event(seek_event);

                    let _ = seeked.replace(true);
                } else {
                    println!("Got second AsyncDone message, seek finished");
                }
            }
            gst::MessageView::Eos(..) => {
                println!("bin bus received eos");
                // let main_loop = &main_loop_clone;
                // main_loop.quit()
            }
            gst::MessageView::Error(err) => {
                // let main_loop = &main_loop_clone;
                eprintln!(
                    "Bin Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                // main_loop.quit();
            }
            _ => (),
        });
        bus.add_signal_watch();

        println!("bus start playing");
        let _ = bin.set_state(gst::State::Paused);
        println!("bus after playing");
        // self.movies.push(bin);
    }

    // let pipeline = gst::parse_launch(&format!(
    //     "uridecodebin uri=file:///{} ! videoconvert ! appsink name=sink", &movie.uri
    // )).unwrap()
    // .downcast::<gst::Pipeline>()
    // .expect("Expected a gst::Pipeline");
    // // Get access to the appsink element.
    // let appsink = pipeline
    //     .by_name("sink")
    //     .expect("Sink element not found")
    //     .downcast::<gst_app::AppSink>()
    //     .expect("Sink element is expected to be an appsink!");

    /// 取消录制视频
    pub fn cancel(&self) {}
}
