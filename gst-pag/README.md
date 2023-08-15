# gst-pag

## 调试

编译完成之后，可以通过 `gst-inspect` 查看插件信息

```sh
gst-inspect-1.0 target/debug/libgstpagsrc.dylib
```

基于 Streamer pipeline 调试自定义插件

```sh
export GST_PLUGIN_PATH=`pwd`/target/debug
gst-launch-1.0 pagsrc ! decodebin ! audioconvert ! autoaudiosink
```