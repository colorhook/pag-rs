use gst::glib;
use gst_base::subclass::prelude::*;
use libpag::PAGFile;

mod imp {
    use std::sync::Mutex;

    use gst::glib;
    use gst_base::subclass::base_src::CreateSuccess;
    use gst_base::subclass::prelude::*;

    use once_cell::sync::Lazy;

    use libpag::PAGFile;

    #[derive(Debug)]
    struct Settings {
        pub file: Option<PAGFile>,
    }

    unsafe impl Send for Settings {}
    unsafe impl Sync for Settings {}

    impl Default for Settings {
        fn default() -> Self {
            Settings { file: None }
        }
    }

    impl Settings {
        pub fn file(&self) -> &PAGFile {
            self.file.as_ref().unwrap()
        }
    }

    // Struct containing all the element data
    #[derive(Default)]
    pub struct PAGAudioSrc {
        settings: Mutex<Settings>,
    }

    impl PAGAudioSrc {
        pub fn set_file(&self, file: PAGFile) {
            let mut settings = self.settings.lock().unwrap();
            settings.file = Some(file);
        }
    }

    // This trait registers our type with the GObject object system and
    // provides the entry points for creating a new instance and setting
    // up the class data
    #[glib::object_subclass]
    impl ObjectSubclass for PAGAudioSrc {
        const NAME: &'static str = "GstPAGAudioSrc";
        type Type = super::PAGAudioSrc;
        type ParentType = gst_base::BaseSrc;
    }

    // Implementation of glib::Object virtual methods
    impl ObjectImpl for PAGAudioSrc {}

    impl GstObjectImpl for PAGAudioSrc {}

    // Implementation of gst::Element virtual methods
    impl ElementImpl for PAGAudioSrc {
        fn pad_templates() -> &'static [gst::PadTemplate] {
            static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
                // On the src pad, we can produce F32/F64 with any sample rate
                // and any number of channels
                let caps = gst::Caps::new_any();
                // The src pad template must be named "src" for basesrc
                // and specific a pad that is always there
                let src_pad_template = gst::PadTemplate::new(
                    "src",
                    gst::PadDirection::Src,
                    gst::PadPresence::Always,
                    &caps,
                )
                .unwrap();

                vec![src_pad_template]
            });

            PAD_TEMPLATES.as_ref()
        }
    }

    // Implementation of gst_base::BaseSrc virtual methods
    impl BaseSrcImpl for PAGAudioSrc {
        fn size(&self) -> Option<u64> {
            let settings = self.settings.lock().unwrap();
            let file = settings.file();
            let audio_bytes: &[u8] = file.audio_bytes().unwrap();
            Some(audio_bytes.len() as u64)
        }

        fn is_seekable(&self) -> bool {
            true
        }

        fn create(
            &self,
            offset: u64,
            _buffer: Option<&mut gst::BufferRef>,
            length: u32,
        ) -> Result<CreateSuccess, gst::FlowError> {
            let offset: usize = offset.try_into().unwrap();
            let length: usize = length.try_into().unwrap();
            let mut buffer = gst::Buffer::with_size(length).unwrap();
            {
                let buffer = buffer.get_mut().unwrap();

                let settings = &self.settings.lock().unwrap();
                let file = settings.file();
                let audio_bytes: &[u8] = file.audio_bytes().unwrap();

                let end = offset + length;
                let slice = &audio_bytes[offset..end];
                buffer.copy_from_slice(0, slice).unwrap();
            }
            Ok(CreateSuccess::NewBuffer(buffer))
        }
    }
}

glib::wrapper! {
    pub struct PAGAudioSrc(ObjectSubclass<imp::PAGAudioSrc>) @extends gst_base::BaseSrc, gst::Element, gst::Object;
}

impl PAGAudioSrc {
    pub fn new(file: PAGFile) -> Self {
        let src: Self = glib::Object::builder().build();
        let imp = src.imp();
        imp.set_file(file);
        src
    }
}
