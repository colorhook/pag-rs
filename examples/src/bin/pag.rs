use libpag::*;

fn main() {
    println!("pag sdk version: {}", sdk_version());
    println!(
        "max_supported_tag_level: {}",
        PAGFile::max_supported_tag_level()
    );
}
