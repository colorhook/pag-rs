use gst::prelude::*;
use libpag::{PAGPlayer, PAGSurface};

use crate::ExportProgressMessage;

pub struct PAGAppSrc {
    pub video_info: gst_video::VideoInfo,
    pub current_frame: i32,
    pub total_frames: i32,
    pub surface: PAGSurface,
    pub player: PAGPlayer,
}

unsafe impl Send for PAGAppSrc {}
unsafe impl Sync for PAGAppSrc {}

impl PAGAppSrc {
    pub fn poll_frame(&mut self) -> Option<(gst::Buffer, f32)> {
        if self.current_frame >= self.total_frames {
            return None;
        }
        self.check_movies();
        self.player.flush();

        let video_info = &self.video_info;
        let mut buffer = gst::Buffer::with_size(video_info.size()).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let frame = self.current_frame as u64;
            let fps = video_info.fps();
            let duration = gst::ClockTime::SECOND
                .mul_div_floor(
                    fps.denom().try_into().unwrap(),
                    fps.numer().try_into().unwrap(),
                )
                .expect("u64 overflow");
            buffer.set_pts(frame * duration);
            buffer.set_duration(duration);
            let mut vframe =
                gst_video::VideoFrameRef::from_buffer_ref_writable(buffer, video_info).unwrap();

            let video_ptr = vframe.plane_data_mut(0).unwrap();
            self.surface.read_rgba(video_ptr);
        }

        let progress = self.current_frame as f32 / self.total_frames as f32;
        self.player.next_frame();
        self.current_frame += 1;
        Some((buffer, progress))
    }

    fn check_movies(&self) {}

    pub fn to_app_src(mut self) -> gst_app::AppSrc {
        let appsrc = gst_app::AppSrc::builder()
            .caps(&self.video_info.to_caps().unwrap())
            .format(gst::Format::Time)
            .build();

        appsrc.set_callbacks(
            gst_app::AppSrcCallbacks::builder()
                .need_data(move |appsrc, _| match self.poll_frame() {
                    Some((buffer, progress)) => {
                        let _ = appsrc.push_buffer(buffer);
                        let message = ExportProgressMessage::new(progress);
                        let _ = appsrc.post_message(message);
                    }
                    None => {
                        let _ = appsrc.end_of_stream();
                    }
                })
                .build(),
        );
        appsrc
    }
}
