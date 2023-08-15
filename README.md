# libpag-rs

该仓库是 `libpag` 库的 Rust 绑定

## 目录

* ffi - 提供了 C++ 到 Rust 语音的绑定，并解决了部分结构体和方法无法正常绑定的问题
* pag - 封装了 ffi 模块，将 API 封装为 Rust 风格的代码，并提供了父子类型的转化
* gst-pag - Gstreamer PAG 插件，可以将 PAG 作为 Gstreamer Source Element 加入到 Gstreamer Pipeline 中去
* node-pag - pag 和 gst-pag 的 Node.js 绑定
  * node-pag/examples - JavaScript examples
* examples - pag API example
 
## 构建

仓库采用 `git submodule` 引用了 libpag 仓库。clone 仓库需要使用 `--recursive` 参数，或者在 clone 之后运行 `git submodule update --init` 拉取 libpag 代码到 `third_party` 目录。

进入 `third_party/libpag` 目录，参考第三方构建指令编译 C++ 代码，以 macOS 平台为例

```sh
# 拉取依赖
./sync_deps.sh
# 在 build 目录产生 Makefile
cmake . -B build -DPAG_BUILD_TESTS=OFF
# 构建
cd build
make
```

编译完成后，会生成平台下的 `libpag` 库，Rust 绑定的构建与运行依赖 `libpag` 的构建产物

运行 examples 文件夹下的案例（项目根目录运行即可）：

```sh
// pag 版本号
cargo run --bin pag
// 文件加载
cargo run --bin file
// 文本设置
cargo run --bin text
// 属性标记
cargo run --bin marker
// 图片替换
cargo run --bin image
```

## 参考

* https://cxx.rs/binding/result.html
* https://docs.rs/autocxx/latest/autocxx/index.html