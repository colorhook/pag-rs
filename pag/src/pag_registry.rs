use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::{LayerType, PAGComposition, PAGFile, PAGImage, PAGLayer, PAGMovie, ID};

// 为了支持视 Node.js 绑定，以及为了支持视频替换扩展能力
// 这里在提供了一个存储 PAGImage 和 PAGLayer 所有权的全局变量
lazy_static! {
    static ref GLOBAL_PAG_REGISTRY: Arc<PAGRegistry> = Arc::new(PAGRegistry::new());
}

pub fn global_pag_registry() -> Arc<PAGRegistry> {
    GLOBAL_PAG_REGISTRY.clone()
}

pub struct PAGRegistry {
    layers: RefCell<HashMap<ID, PAGLayer>>,
    images: RefCell<HashMap<ID, PAGImage>>,
    movies: RefCell<HashMap<ID, PAGMovie>>,
}

unsafe impl Send for PAGRegistry {}
unsafe impl Sync for PAGRegistry {}

impl PAGRegistry {
    fn new() -> Self {
        Self {
            layers: Default::default(),
            images: Default::default(),
            movies: Default::default(),
        }
    }

    // 重置
    pub fn reset(&self) {
        let mut map = self.layers.borrow_mut();
        map.clear();
        let mut map = self.images.borrow_mut();
        map.clear();
        let mut map = self.movies.borrow_mut();
        map.clear();
    }

    // 记录 PAGMovie
    pub fn add_movie(&self, id: ID, movie: PAGMovie) {
        let mut map = self.movies.borrow_mut();
        map.deref_mut().insert(id, movie);
    }

    // 获取 PAGMovie
    pub fn get_movie(&self, id: ID) -> Option<PAGMovie> {
        let movies = self.movies.borrow();
        movies.get(&id).cloned()
    }

    // 删除 PAGMovie
    pub fn remove_movie(&self, id: ID) -> bool {
        let mut map = self.movies.borrow_mut();
        map.remove(&id).is_some()
    }

    // 添加 PAGMovie
    pub fn clear_movies(&self) {
        let mut map = self.movies.borrow_mut();
        map.clear();
    }

    // 记录 PAGImage
    pub fn add_image(&self, image: PAGImage) {
        let mut map = self.images.borrow_mut();
        map.deref_mut().insert(image.unique_id(), image);
    }

    // 获取 PAGImage
    pub fn get_image(&self, id: ID) -> Option<PAGImage> {
        let map = self.images.borrow();
        map.get(&id).cloned()
    }

    // 删除 PAGImage
    pub fn remove_image(&self, id: ID) -> bool {
        let mut map = self.images.borrow_mut();
        map.remove(&id).is_some()
    }

    // 添加 PAGImage
    pub fn clear_images(&self) {
        let mut map = self.images.borrow_mut();
        map.clear();
    }

    // 添加 PAGLayer
    pub fn add(&self, file: impl Into<PAGLayer>) -> ID {
        let mut map = self.layers.borrow_mut();
        let file: PAGLayer = file.into();
        let id = file.unique_id();
        map.deref_mut().insert(id, file);
        id
    }

    // 删除 layers
    pub fn remove(&self, id: ID) -> bool {
        let mut map = self.layers.borrow_mut();
        map.remove(&id).is_some()
    }

    // 删除所有 PAGLayer
    pub fn clear(&self) {
        let mut map = self.layers.borrow_mut();
        map.clear();
    }

    // 返回所有匹配 id 的 PAGLayer
    pub fn get(&self, id: ID) -> Option<PAGLayer> {
        let layers = self.layers.borrow();
        layers.get(&id).cloned()
    }

    // 返回所有匹配 id 的 PAGComposition
    pub fn get_composition(&self, id: ID) -> Option<PAGComposition> {
        let layers = self.layers.borrow();
        let file = layers.get(&id);
        match file {
            Some(file) => {
                if file.layer_type() == LayerType::PreCompose {
                    return Some(file.clone().into());
                } else {
                    return None;
                }
            }
            None => None,
        }
    }

    // 返回所有匹配 id 的 PAGFile
    pub fn get_file(&self, id: ID) -> Option<PAGFile> {
        let layers = self.layers.borrow();
        let file = layers.get(&id);
        match file {
            Some(file) => {
                if file.is_pag_file() {
                    return Some(file.clone().into());
                } else {
                    return None;
                }
            }
            None => None,
        }
    }
}
