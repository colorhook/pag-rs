# node-pag

libpag 和相关音视频能力的 Node.js 绑定

## Building node-pag

首先确保安装过 cargo-cp-artifact

```
npm install -g cargo-cp-artifact
```

然后编译 `.node` 模块

```sh
$ npm run build
```

该命令使用 [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) 运行 Rust 构建指令并将结果拷贝到 `./index.node`.

## 在 Node.js 中运行 node-pag

构建完成之后，在 Node.js 中使用 `index.node` 提供的方法：

```sh
$ npm install
$ node
> require('.').PAGFile.MaxSupportedTagLevel() // output: 91 (输出可能根据 libpag 的升级而变化)
```

## Learn More

* [Neon 文档](https://neon-bindings.com).