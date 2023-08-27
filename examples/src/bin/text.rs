use libpag::*;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_dir = dir.join("../third_party/libpag/assets");

    let text_pag = PAGFile::from_file(assets_dir.join("bulgetext.pag").to_str().unwrap()).unwrap();

    println!(
        "text_pag get_editable_indices: {:?}",
        text_pag.get_editable_indices(libpag::LayerType::Text)
    );
    for i in 0..text_pag.num_children() {
        let layer = text_pag.get_layer_at(i).unwrap();
        println!(
            "layer type {:?} name {:?} index {:}",
            layer.layer_type(),
            layer.layer_name(),
            layer.editable_index()
        );

        if layer.editable_index() == 0 {
            let text_layer: libpag::PAGTextLayer = layer.into();
            println!(
                "text layer text: {:?} font_size: {:?}",
                text_layer.text(),
                text_layer.font_size()
            );
            text_layer.set_text("new test 0");
            println!("text layer text(after set_text): {:?}", text_layer.text());
            let font = text_layer.font();
            println!("text layer font: {:?}", font);
            let font = libpag::PAGFont::new("A", "B");
            text_layer.set_font(&font);
            let font = text_layer.font();
            println!("text layer font: {:?}", font);

            let document = text_pag.get_text_data(0).unwrap();
            println!("document apply_fill: {:?}", document.apply_fill());
            document.set_apply_fill(false);
            println!("document apply_fill: {:?}", document.apply_fill());
            text_pag.replace_text(0, None);
            println!("text layer text(after reset): {:?}", text_layer.text());
        }
    }

    // text
    let text_layer = libpag::PAGTextLayer::make(100, "AAA", 12.0, "", "");
    println!("text_layer fill_color: {:?}", text_layer.fill_color());
    text_layer.set_fill_color(&PAGColor {
        red: 255,
        green: 0,
        blue: 0,
    });
    println!("text_layer fill_color: {:?}", text_layer.fill_color());
    println!("text_layer text: {:?}", text_layer.text());
    text_layer.set_text("BBB");
    println!("text_layer text: {:?}", text_layer.text());
    println!("text_layer font_size: {:?}", text_layer.font_size());
    text_layer.set_font_size(24.0);
    println!("text_layer font_size: {:?}", text_layer.font_size());
    println!("text_layer stroke_color: {:?}", text_layer.stroke_color());
    text_layer.set_stroke_color(&PAGColor::BLUE);
    println!("text_layer stroke_color: {:?}", text_layer.stroke_color());

    text_layer.reset();
    println!("text_layer fill_color: {:?}", text_layer.fill_color());
    println!("text_layer stroke_color: {:?}", text_layer.stroke_color());
    println!("text_layer text: {:?}", text_layer.text());
    println!("text_layer font_size: {:?}", text_layer.font_size());
}
