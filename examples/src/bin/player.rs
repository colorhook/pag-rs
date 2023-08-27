use image::{ImageBuffer, Rgba};
use libpag::*;
use std::path::PathBuf;

fn save_image(filename: &str, width: u32, height: u32, data: &[u8]) {
    let imgbuf: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(width, height, data).unwrap();
    imgbuf.save(filename).unwrap();

    // // Create a new ImgBuf with width: imgx and height: imgy
    // let mut imgbuf = ImageBuffer::new(width, height);
    // // Iterate over the coordinates and pixels of the image
    // let mut index = 0;
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let r = data[index];
    //     let g = data[index + 1];
    //     let b = data[index + 2];
    //     let a = data[index + 3];
    //     index += 4;
    //     *pixel = Rgba([r, g, b, a]);
    // }
    // imgbuf.save(filename).unwrap();
}

fn main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    let pag_file = PAGFile::from_file(assets_dir.join("MT2.pag").to_str().unwrap()).unwrap();

    let player = PAGPlayer::new();
    let surface = PAGSurface::make_offscreen(800, 600).unwrap();

    let width = surface.width() as u32;
    let height = surface.height() as u32;
    println!("surface width: {}", width);
    println!("surface height: {}", height);

    let total_frame = (pag_file.duration() as f32) * pag_file.frame_rate() / 1000000.0;
    let total_frame = total_frame as i32;
    println!("total frame: {}", total_frame);

    player.set_surface(&surface);
    player.set_composition(pag_file);

    println!("player.video_enabled {}", player.video_enabled());
    println!("player.cache_enabled {}", player.cache_enabled());
    println!("player.cache_scale {}", player.cache_scale());
    println!("player.max_frame_rate {}", player.max_frame_rate());
    println!("player.scale_mode {:?}", player.scale_mode());
    // println!("player.matrix {:?}", player.matrix());
    println!("player.duration {}", player.duration());
    println!("player.get_progress {}", player.get_progress());
    println!("player.auto_clear {}", player.auto_clear());

    println!("player.rendering_time {}", player.rendering_time());
    println!(
        "player.image_decoding_time {}",
        player.image_decoding_time()
    );
    println!("player.presenting_time {}", player.presenting_time());
    println!("player.graphics_memory {}", player.graphics_memory());

    #[allow(invalid_value)]
    let mut data: [u8; 800 * 600 * 4] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    let pixel: &mut c_void = unsafe { std::mem::transmute(&mut data) };

    // capture frame0
    player.flush();
    surface.read_pixels(
        ColorType::RGBA_8888,
        AlphaType::Premultiplied,
        pixel,
        (width * 4) as usize,
    );

    let file_path = dir.join("frame-first.png");
    let file_path = file_path.to_str().unwrap();
    save_image(file_path, width, height, &data);

    // capture frame10
    player.set_progress(0.5);
    player.flush();
    surface.read_pixels(
        ColorType::RGBA_8888,
        AlphaType::Premultiplied,
        pixel,
        (width * 4) as usize,
    );
    let file_path = dir.join("frame-progress-0.5.png");
    let file_path = file_path.to_str().unwrap();
    save_image(file_path, width, height, &data);
}
