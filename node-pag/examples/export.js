const path = require('path');

const {
  PAGFile,
  PAGImage,
  PAGExportSession,
  global_registry,
  PAGScaleMode
} = require('..');

const file = new PAGFile(path.join(__dirname, '../../third_party/libpag/assets/AudioMarker.pag'));
const image = new PAGImage(path.join(__dirname, '../../third_party/libpag/assets/mountain.jpg'));

// PAGFile important methods
image.scale_mode = PAGScaleMode.Zoom;
file.replace_image(4, image);

// video export
const session = new PAGExportSession(file, path.join(__dirname, "export.mp4"));
session.start((err) => {
  if (!err) {
    console.log("completed")
  } else {
    console.log("error: ", err)
  }
  // !important - 必要的时候需要进行内存回收
  global_registry.reset();
}, (progress) => {
  console.log("progress: ", progress)
});
console.log("start export...")
