use libpag::*;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    // image
    let image_layer = PAGImageLayer::make(512, 512, 20);
    println!(
        "image_layer content duration: {:?}",
        image_layer.content_duration()
    );
    println!(
        "image_layer video ranges: {:?}",
        image_layer.get_video_ranges()
    );
    println!(
        "image_layer layer_time_to_content: {:?}",
        image_layer.layer_time_to_content(100)
    );
    println!(
        "image_layer content_time_to_layer: {:?}",
        image_layer.content_time_to_layer(100)
    );

    let image = PAGImage::from_path(assets_dir.join("test.png").to_str().unwrap());
    println!("image width: {:?}", image.width());
    println!("image height: {:?}", image.height());
    println!("image scale_mode: {:?}", image.scale_mode());
    image.set_scale_mode(PAGScaleMode::Zoom);
    println!("image scale_mode: {:?}", image.scale_mode());

    let pag_file = PAGFile::from_file(assets_dir.join("MT2.pag").to_str().unwrap());
    let layer0 = pag_file.get_layer_at(0).unwrap();
    let layer0: PAGImageLayer = layer0.into();
    println!(
        "layer 0 layer_type: {:?} layer_name: {}",
        layer0.layer_type(),
        layer0.layer_name()
    );
    println!("layer 0 content_duration {}", layer0.content_duration());
    println!("layer 0 image_bytes.len: {:?}", layer0.image_bytes().len());
    image_layer.set_image(Some(image));
    let byte = image_layer.image_bytes();
    println!("image_layer bytes.len as set image: {:?}", byte.len());
}
