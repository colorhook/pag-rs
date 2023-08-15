use libpag::*;

fn main() {
    let composition = PAGComposition::make(-1, -1);
    println!("composition: {:?}", &composition);

    let num_children = composition.num_children();
    println!("num_children: {:?}", num_children);

    println!("size: {}x{}", composition.width(), composition.height());

    let text_layer = PAGTextLayer::make(10, "aa", 12.0, "", "");
    println!("text_layer: {:?}", &text_layer);

    let add_action = composition.add_layer(text_layer);
    println!("add text layer to composition result: {}", add_action);

    let add_action = composition.add_layer(PAGComposition::make(1, 1));
    println!("add composition to composition result: {}", add_action);

    println!("num_children: {:?}", composition.num_children());
    println!(
        "child 0 layer type is: {:?}",
        composition.get_layer_at(0).unwrap().layer_type()
    );
    println!("size: {}x{}", composition.width(), composition.height());
}
