use std::path::PathBuf;
use libpag::*;

fn main() {

  let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let assets_dir = dir.join("../third_party/libpag/assets");
  
  let marker_file = PAGFile::from_file(assets_dir.join("marker.pag").to_str().unwrap()).unwrap();
  let markers = marker_file.markers();
  println!("marker.pag root markers: {:?}", markers);
  let wave_layer = marker_file.get_layer_at(8).unwrap();
  println!("wave_layer name: {:?}", wave_layer.layer_name());
  let markers = wave_layer.markers();
  println!("wave_layer markers: {:?}", markers);


  let audio_file = PAGFile::from_file(assets_dir.join("AudioMarker.pag").to_str().unwrap()).unwrap();
  println!("AudioMarker.pag num_children: {:?} ", audio_file.num_children());

  let composition: PAGComposition = audio_file.get_layer_at(0).unwrap().parent().unwrap();
  let markers = composition.audio_markers();
  println!("AudioMarker.pag audio markers: {:?}", markers);
}