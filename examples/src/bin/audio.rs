use libpag::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    let file = PAGFile::from_file(assets_dir.join("AudioMarker.pag").to_str().unwrap()).unwrap();
    let audio_bytes = file.audio_bytes().unwrap();

    let file_path = dir.join("audio.mp3");
    let mut file = File::create(file_path).unwrap();
    file.write_all(audio_bytes).unwrap();
}
