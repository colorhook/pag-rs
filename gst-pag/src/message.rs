#[derive(Debug)]
pub struct ExportProgressMessage {
    pub progress: f32,
}

impl ExportProgressMessage {
    const NAME: &'static str = "export-progress";

    #[allow(clippy::new_ret_no_self)]
    pub fn new(progress: f32) -> gst::Message {
        let s = gst::Structure::builder(Self::NAME)
            .field("progress", progress)
            .build();
        gst::message::Application::new(s)
    }

    pub fn parse(message: &gst::message::Application) -> Option<Self> {
        match message.structure() {
            Some(s) if s.name() == Self::NAME => {
                let progress = s.get::<f32>("progress").unwrap();
                Some(ExportProgressMessage { progress })
            }
            _ => None,
        }
    }
}
