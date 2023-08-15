use libpag::*;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    let invalid_file = PAGFile::from_file(assets_dir.join("not-a-pag-file.pag").to_str().unwrap());
    println!("invalid_file is {:?}", invalid_file);

    let pag_file = PAGFile::from_file(assets_dir.join("MT2.pag").to_str().unwrap()).unwrap();
    println!("tag level = {}", pag_file.tag_level());

    println!("text num = {}", pag_file.num_texts());
    println!("image num = {}", pag_file.num_images());
    println!("video num = {}", pag_file.num_videos());

    println!("path = {}", pag_file.path());
    println!("time_stretch_mode = {:?}", pag_file.time_stretch_mode());
    println!("is_pag_file = {}", pag_file.is_pag_file());

    // 父类 PAGComposition 方法
    println!("num_children = {}", pag_file.num_children());
    println!("width = {}", pag_file.width());
    println!("width = {}", pag_file.height());
    println!("audio_bytes = {:?}", pag_file.audio_bytes());

    // 父类 PAGLayer 方法
    println!("unique_id = {}", pag_file.unique_id());
    println!("layer_type = {}", pag_file.layer_type());
    println!("layer_name = {}", pag_file.layer_name());
    println!("alpha = {}", pag_file.alpha());
    pag_file.set_alpha(0.4);
    println!("alpha = {}", pag_file.alpha());
    println!("get_bounds = {}", pag_file.get_bounds());
    println!("visible = {}", pag_file.visible());
    pag_file.set_visible(false);
    println!("visible = {}", pag_file.visible());

    println!("editable_index = {}", pag_file.editable_index());
    println!("parent is None = {}", pag_file.parent().is_none());
    println!(
        "track_matte_layer is None = {}",
        pag_file.track_matte_layer().is_none()
    );

    println!("duration = {}", pag_file.duration());
    println!("frame_rate = {}", pag_file.frame_rate());
    println!("start_time = {}", pag_file.start_time());
    println!("current_time = {}", pag_file.current_time());
    println!("progress = {}", pag_file.get_progress());
    pag_file.next_frame();
    println!("start_time = {}", pag_file.start_time());
    println!("current_time = {}", pag_file.current_time());
    println!("progress = {}", pag_file.get_progress());
    pag_file.set_start_time(10);
    println!("start_time = {}", pag_file.start_time());
    println!("current_time = {}", pag_file.current_time());
    println!("progress = {}", pag_file.get_progress());

    println!(
        "excluded_from_timeline = {}",
        pag_file.excluded_from_timeline()
    );
    pag_file.set_excluded_from_timeline(true);
    println!(
        "excluded_from_timeline = {}",
        pag_file.excluded_from_timeline()
    );

    let matrix = pag_file.matrix();
    println!("matrix: {:?}", matrix.isIdentity());
}
