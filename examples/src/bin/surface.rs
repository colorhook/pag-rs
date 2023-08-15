use libpag::*;

fn main() {
    let surface = PAGSurface::make_offscreen(4, 4).unwrap();
    println!("surface width: {}", surface.width());
    println!("surface height: {}", surface.height());

    let mut data: Vec<u8> = Vec::with_capacity(4 * 4 * 4);
    unsafe {
        data.set_len(4 * 4 * 4);
    };
    surface.read_rgba(data.as_mut_slice());

    println!("data: {:?}", data);

    let surface = PAGSurface::make_offscreen(-1, -1);
    println!("surface creation should failed: {:?}", surface);
}
