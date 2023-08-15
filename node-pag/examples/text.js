const path = require('path');

const {
  PAGFile,
  PAGExportSession,
  global_registry,
} = require('..');

const file = new PAGFile(path.join(__dirname, '../../third_party/libpag/assets/bulgetext.pag'));

const text_data = file.get_text_data(0);
console.log('text_data: ', text_data);
text_data.text = "新的测试";
file.replace_text(0, text_data);

// 注意，参考 libpag 的文档，get_text_data 获取的永远是 pag 模版中的默认值
console.log('new text_data: ', file.get_text_data(0));

// video export
const session = new PAGExportSession(file, path.join(__dirname, "text.mp4"));
session.start((err) => {
  if (!err) {
    console.log("completed")
  } else {
    console.log("error: ", err)
  }
  global_registry.reset();
});
console.log("start export...")