use crate::global_pag_registry;
use crate::{LayerType, PAGFile, PAGImageLayer};

#[derive(Debug, Clone)]
pub struct PAGMovie {
    pub uri: String,
    pub offset: f32,
    pub duration: Option<f32>,
    pub rate: f32,
    pub volume: f32,
}

impl Default for PAGMovie {
    fn default() -> Self {
        Self {
            uri: String::from(""),
            offset: 0.0,
            duration: None,
            rate: 1.0,
            volume: 1.0,
        }
    }
}

impl PAGMovie {
    pub fn from_file(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
            ..Default::default()
        }
    }
}

pub trait MovieLayerExt {
    fn set_movie(&self, movie: Option<PAGMovie>);
}

/// 为 PAGImageLayer 扩展实现 set_movie 方法
impl MovieLayerExt for PAGImageLayer {
    fn set_movie(&self, movie: Option<PAGMovie>) {
        match movie {
            Some(inner) => {
                global_pag_registry().add_movie(self.unique_id(), inner);
            }
            None => {
                global_pag_registry().remove_movie(self.unique_id());
            }
        }
    }
}

pub trait MovieFileExt {
    fn replace_movie(&self, index: i32, movie: Option<PAGMovie>);
}

/// 为 PAGFile 扩展实现 replace_movie 方法
impl MovieFileExt for PAGFile {
    fn replace_movie(&self, index: i32, movie: Option<PAGMovie>) {
        let layers = self.get_layers_by_editable_index(index, LayerType::Image);
        if let Some(layer) = layers.get(0) {
            let layer: PAGImageLayer = layer.clone().into();
            layer.set_movie(movie);
        }
    }
}
