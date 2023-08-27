use gst_pag::*;
use libpag::*;
use std::path::PathBuf;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    PAGFont::set_fallback_font_names(vec![
        "PingFang SC",
        "Apple SD Gothic Neo",
        "Apple Color Emoji",
        "Helvetica",
        "Myanmar Sangam MN",
        "Thonburi",
        "Mishafi",
        "Menlo",
        "Kailasa",
        "Kefa",
        "Kohinoor Telugu",
        "Hiragino Maru Gothic ProN",
    ]);
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    let file = format!("{}", assets_dir.join("TextDirection.pag").to_str().unwrap());
    let output = format!("{}", dir.join("session.mp4").to_str().unwrap());
    let pagfile = PAGFile::from_file(file).unwrap();

    // // 图片替换
    // let image_uri = format!("{}", dir.join("frame-first.png").to_str().unwrap());
    // let image = PAGImage::from_path(image_uri);
    // pagfile.replace_image(1, Some(image));

    // // 视频替换
    // let mp4 = format!("{}", dir.join("skia.mp4").to_str().unwrap());
    // let mut movie = PAGMovie::from_file(&mp4);
    // movie.offset = 2.0;
    // movie.rate = 3.0;
    // pagfile.replace_movie(2, Some(movie));

    let mut session = PAGExportSession::new(pagfile, &output);
    // println!("session: {:?}", session);
    let _ = session.start(Some(Box::new(|progress| {
        println!("progress: {}", progress);
    })));
}

fn main() {
    examples_common::run(example_main);
}
