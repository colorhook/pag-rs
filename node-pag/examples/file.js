const path = require('path');

const {
  PAGFile,
  PAGImage,
  LayerType,
} = require('..');

// static methods
console.log("MaxSupportedTagLevel: ", PAGFile.MaxSupportedTagLevel());

// image
const image = new PAGImage(path.join(__dirname, '../../third_party/libpag/assets/mountain.jpg'));
console.log('>>>> image.unique_id: ', image.unique_id);
console.log('width: ', image.width);
console.log('height: ', image.height);
console.log('scale_mode: ', image.scale_mode);

// file
const file = new PAGFile(path.join(__dirname, '../../third_party/libpag/assets/AudioMarker.pag'));

// PAGLayer methods
console.log('>>>> file.unique_id: ', file.unique_id);
console.log('width: ', file.width);
console.log('height: ', file.height);
console.log('duration: ', file.duration);
console.log('frame_rate: ', file.frame_rate);
console.log('start_time: ', file.start_time);
console.log('current_time: ', file.current_time);
console.log('progress: ', file.progress);
console.log('tag_level: ', file.tag_level);
console.log('layer_name: ', file.layer_name);
console.log('layer_type: ', file.layer_type);
console.log('alpha: ', file.alpha);
console.log('visible: ', file.visible);

// PAGComposition methods
console.log('num_children: ', file.num_children);
console.log('audio_start_time: ', file.audio_start_time);

// PAGFile methods
console.log('num_texts: ', file.num_texts);
console.log('num_images: ', file.num_images);
console.log('num_videos: ', file.num_videos);
console.log('path: ', file.path);
console.log('time_stretch_mode', file.time_stretch_mode);
const text_indices = file.get_editable_indices(LayerType.Text);
console.log("file editable text indices", text_indices);
const image_indices = file.get_editable_indices(LayerType.Image);
console.log("file editable image indices", image_indices);