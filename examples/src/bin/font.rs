use libpag::*;

fn main() {
    let font = PAGFont::new("", "");
    println!("font new: {}", font);

    let fonts: Vec<String> = vec![];
    PAGFont::set_fallback_font_names(fonts);

    let data: Vec<u8> = vec![];
    let font = PAGFont::register_font_from_data(&data, 0, "", "");

    println!("font from_data: {}", font);
}
