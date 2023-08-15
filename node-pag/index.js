const addon = require('./index.node');

const privateSymbol = Symbol('private constructor');

/**
 * 代表 PAGImage 的缩放类型
 */
const PAGScaleMode = {
  None: 0,
  Stretch: 1,
  LetterBox: 2,
  Zoom: 3
};
exports.PAGScaleMode = PAGScaleMode;

/**
 * 代表 PAGFile 的时间 stretch 模式
 */
const PAGTimeStretchMode = {
  None: 0,
  Stretch: 1,
  Repeat: 2,
  RepeatInverted: 2
}
exports.PAGTimeStretchMode = PAGTimeStretchMode;

/**
 * 代表 pag 中 Layer 的类型
 */
const LayerType = {
  Null: "Null",
  Solid: "Solid",
  Text: "Text",
  Shape: "Shape",
  Image: "Image",
  PreCompose: "PreCompose"
}
exports.LayerType = LayerType;


/***
 * TextDocument 中的段落对齐方式
 */
const ParagraphJustification = {
  LeftJustify: 0,
  CenterJustify: 1,
  RightJustify: 2,
  FullJustifyLastLineLeft: 3,
  FullJustifyLastLineRight: 4,
  FullJustifyLastLineCenter: 5,
  FullJustifyLastLineFull: 6,
}
exports.ParagraphJustification = ParagraphJustification;

/***
 * TextDocument 中的文本方向
 */
const TextDirection = {
  Default: 0,
  Horizontal: 1,
  Vertical: 2,
}
exports.TextDirection = TextDirection;

/**
 * 代表图片资源
 * 如果需要替换 pag 中的图片，需要使用 uri 构建一个 PAGImage 类
 * 然后使用 PAGFile 中的 replace_image/replace_image_by_name 方法替换成自定义的类
 */
class PAGImage {
  constructor(uri) {
    const uid = addon.pag_image_new(uri);
    this.uid = uid;
  }
  get unique_id() {
    return this.uid;
  }
  get width() {
    return addon.pag_image_width(this.uid);
  }
  get height() {
    return addon.pag_image_height(this.uid);
  }
  get scale_mode() {
    return addon.pag_image_scale_mode(this.uid);
  }
  set scale_mode(value) {
    if (!Object.values(PAGScaleMode).includes(value)) {
      throw new Error('invalid PAGScaleMode')
    }
    return addon.pag_image_set_scale_mode(this.uid, value);
  }
  dispose() {
    addon.pag_image_dispose(this.uid);
  }
}
exports.PAGImage = PAGImage;


/**
 * pag 文件中 layer 抽象基类
 * PAGComposition 继承自 PAGLayer
 * PAGFile 继承自 PAGLayer
 */
class PAGLayer {
  constructor(symbol, uid) {
    if (symbol !== privateSymbol) {
      throw new Error('Cannot constructor this class yourself');
    }
    this.uid = uid;
  }
  get unique_id() {
    return this.uid;
  }
  get layer_name() {
    return addon.pag_layer_layer_name(this.uid)
  }
  get layer_type() {
    return addon.pag_layer_layer_type(this.uid)
  }
  get alpha() {
    return addon.pag_layer_alpha(this.uid)
  }
  set alpha(value) {
    return addon.pag_layer_set_alpha(this.uid, value)
  }
  get visible() {
    return addon.pag_layer_visible(this.uid)
  }
  set visible(value) {
    return addon.pag_layer_set_visible(this.uid, value)
  }
  get editable_index() {
    return addon.pag_layer_editable_index(this.uid)
  }
  get frame_rate() {
    return addon.pag_layer_frame_rate(this.uid)
  }
  get duration() {
    return addon.pag_layer_duration(this.uid)
  }
  get start_time() {
    return addon.pag_layer_start_time(this.uid)
  }
  set start_time(value) {
    return addon.pag_layer_set_start_time(this.uid, value)
  }
  get current_time() {
    return addon.pag_layer_current_time(this.uid)
  }
  set current_time(value) {
    return addon.pag_layer_set_current_time(this.uid, value)
  }
  get progress() {
    return addon.pag_layer_get_progress(this.uid)
  }
  set progress(value) {
    return addon.pag_layer_set_progress(this.uid, value)
  }
  pre_frame() {
    return addon.pag_layer_pre_frame(this.uid)
  }
  prev_frame() {
    this.pre_frame();
  }
  next_frame() {
    return addon.pag_layer_next_frame(this.uid)
  }
  get is_pag_file() {
    return addon.pag_layer_is_pag_file(this.uid)
  }
  dispose() {
    return addon.pag_layer_dispose(this.uid)
  }
}

/**
 * PAGComposition 表示可嵌套的 PAGLayer
 */
class PAGComposition extends PAGLayer {
  static make(width, height) {
    const uid = addon.pag_composition_make(width, height);
    const composition = new PAGComposition(privateSymbol, uid);
    return composition;
  }
  get width() {
    return addon.pag_composition_width(this.uid)
  }
  get height() {
    return addon.pag_composition_height(this.uid)
  }
  get num_children() {
    return addon.pag_composition_num_children(this.uid)
  }
  get audio_start_time() {
    return addon.pag_composition_audio_start_time(this.uid)
  }
}

/**
 * PAGFile 从 .pag 文件加载构建而成
 */
class PAGFile extends PAGComposition {
  static MaxSupportedTagLevel() {
    return addon.pag_file_max_supported_tag_level();
  }
  constructor(file) {
    const uid = addon.pag_file_new(file);
    super(privateSymbol, uid);
  }
  get tag_level() {
    return addon.pag_file_tag_level(this.uid)
  }
  get num_texts() {
    return addon.pag_file_num_texts(this.uid)
  }
  get num_images() {
    return addon.pag_file_num_images(this.uid)
  }
  get num_videos() {
    return addon.pag_file_num_videos(this.uid)
  }
  get path() {
    return addon.pag_file_path(this.uid)
  }
  // 一直返回的是默认值，参考自 libpag C++ 的注释
  // It always returns the default text data
  get_text_data(index) {
    return addon.pag_file_get_text_data(this.uid, index)
  }
  replace_text(index, text_data) {
    addon.pag_file_replace_text(this.uid, index, text_data)
  }
  replace_image(index, image) {
    if (image && !(image instanceof PAGImage)) {
      throw new Error('invalid PAGImage instance')
    }
    addon.pag_file_replace_image(this.uid, index, image.uid)
  }
  get_editable_indices(type) {
    if (!Object.values(LayerType).includes(type)) {
      throw new Error('invalid LayerType')
    }
    return addon.pag_file_get_editable_indices(this.uid, type)
  }
  get time_stretch_mode() {
    return addon.pag_file_time_stretch_mode(this.uid);
  }
  set time_stretch_mode(value) {
    if (!Object.values(PAGTimeStretchMode).includes(value)) {
      throw new Error('invalid PAGTimeStretchMode')
    }
    addon.pag_file_set_time_stretch_mode(this.uid, value);
  }
}

exports.PAGFile = PAGFile;


/**
 * 内存管理工具
 * 
 * 因为 JavaScript 具备 VM，垃圾回收自动运行，而 Rust 语言是手动内存管理机制。
 * 在 JavaScript 中创建的 PAGImage、PAFile 构造函数，会通过 addon 的方法在 Rust 中创建对应的对象；
 * 为了保证 JavaScript 中调用它们的方法不会出错，这些对象不能主动在 Rust 中销毁，而是存储在一个全局的 Map 中。
 * 何时销毁需要在 JavaScript 中通过调用 PAGImage/PAFile 原型链上的 dispose 方法才能释放内存。
 * 
 * 如果在 Node.js 的 Web 服务中使用这些方法，使用者有可能忘记调用 dispose 方法，导致长时间运行后内存一直不释放的问题。
 * 为了让内存释放更简单，可以在特定的时机直接执行 global_registry.reset() 方法释放内存。
 * 注意：执行该方法后要，之前创建的 PAGFile、PAGImage 等对象将不能再正常工作。
 */
exports.global_registry = {
  reset() {
    addon.pag_registry_reset();
  }
}

/**
 * 视频录制模块
 */
class PAGExportSession {
  constructor(file, options) {
    if (!file || !(file instanceof PAGComposition)) {
      throw new Error('invalid PAGComposition instance')
    }
    this.file = file;
    if (!options) {
      throw new Error('invalid export options');
    }
    let config = {};
    if (typeof options === 'string') {
      config.output = options;
    } else {
      if (!options.output) {
        throw new Error('invalid export options.output');
      }
      config.output = options.output;

      // width/height >=8 纯粹是随便定义的一个约束
      if (options.width >= 8) {
        config.width = Math.floor(options.width)
      }
      if (options.height >= 8) {
        config.height = Math.floor(options.height)
      }
      if (options.frame_rate > 0) {
        config.frame_rate = options.frame_rate
      }
      
    }
    if (!config.output.toLowerCase().endsWith('.mp4')) {
      throw new Error('invalid export options.output, output should be a mp4 file');
    }

    this.config = config;
    console.log(this.config)
  }
  start(callback, progress_callback) {
    addon.pag_export_session_start_async(this.file.uid, this.config, (p) => {
      if (progress_callback) {
        progress_callback(p)
      }
    }, (err) => {
      if (callback) {
        callback(err)
      }
    })
  }
}

exports.PAGExportSession = PAGExportSession;